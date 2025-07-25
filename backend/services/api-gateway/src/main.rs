//! FlowEx API Gateway
//!
//! Enterprise-grade API gateway providing load balancing, rate limiting,
//! authentication, and request routing for FlowEx microservices.

use axum::{
    extract::{Request, State, Path},
    http::{StatusCode, HeaderMap, Method, Uri},
    response::{Response, Json},
    routing::{any, get},
    Router,
    body::Body,
};
use flowex_types::{ApiResponse, HealthResponse, FlowExError, FlowExResult};
use flowex_metrics::MetricsCollector;
use flowex_cache::CacheManager;
use governor::{Quota, RateLimiter, state::{InMemoryState, NotKeyed}};
use hyper::client::HttpConnector;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::Arc,
    time::{Duration, SystemTime},
    net::SocketAddr,
};
use tokio::sync::RwLock;
use tower::ServiceBuilder;
use tower_http::{
    cors::CorsLayer,
    trace::TraceLayer,
    compression::CompressionLayer,
    timeout::TimeoutLayer,
};
use tracing::{info, warn, error, debug};
use uuid::Uuid;

/// API Gateway configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatewayConfig {
    pub host: String,
    pub port: u16,
    pub services: HashMap<String, ServiceConfig>,
    pub rate_limit: RateLimitConfig,
    pub timeout_seconds: u64,
    pub max_request_size: usize,
}

/// Service configuration for routing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceConfig {
    pub name: String,
    pub instances: Vec<ServiceInstance>,
    pub health_check_path: String,
    pub load_balancer: LoadBalancerType,
    pub circuit_breaker: CircuitBreakerConfig,
}

/// Service instance configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceInstance {
    pub id: String,
    pub host: String,
    pub port: u16,
    pub weight: u32,
    pub healthy: bool,
}

/// Load balancer types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LoadBalancerType {
    RoundRobin,
    WeightedRoundRobin,
    LeastConnections,
    Random,
}

/// Circuit breaker configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CircuitBreakerConfig {
    pub failure_threshold: u32,
    pub timeout_seconds: u64,
    pub half_open_max_calls: u32,
}

/// Rate limiting configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitConfig {
    pub requests_per_minute: u32,
    pub burst_size: u32,
    pub enabled: bool,
}

/// Application state
#[derive(Clone)]
pub struct AppState {
    pub config: GatewayConfig,
    pub http_client: Client,
    pub metrics: MetricsCollector,
    pub cache: CacheManager,
    pub rate_limiter: Arc<RateLimiter<NotKeyed, InMemoryState>>,
    pub service_states: Arc<RwLock<HashMap<String, ServiceState>>>,
    pub start_time: SystemTime,
}

/// Service state for health monitoring
#[derive(Debug, Clone)]
pub struct ServiceState {
    pub healthy_instances: Vec<ServiceInstance>,
    pub unhealthy_instances: Vec<ServiceInstance>,
    pub current_index: usize,
    pub total_requests: u64,
    pub failed_requests: u64,
    pub last_health_check: SystemTime,
}

impl AppState {
    /// Create new application state
    pub async fn new(config: GatewayConfig, cache: CacheManager) -> FlowExResult<Self> {
        let http_client = Client::builder()
            .timeout(Duration::from_secs(config.timeout_seconds))
            .build()
            .map_err(|e| FlowExError::Internal(format!("Failed to create HTTP client: {}", e)))?;

        let metrics = MetricsCollector::new();

        // Create rate limiter
        let quota = Quota::per_minute(config.rate_limit.requests_per_minute)
            .allow_burst(config.rate_limit.burst_size);
        let rate_limiter = Arc::new(RateLimiter::direct(quota));

        // Initialize service states
        let mut service_states = HashMap::new();
        for (service_name, service_config) in &config.services {
            let state = ServiceState {
                healthy_instances: service_config.instances.clone(),
                unhealthy_instances: Vec::new(),
                current_index: 0,
                total_requests: 0,
                failed_requests: 0,
                last_health_check: SystemTime::now(),
            };
            service_states.insert(service_name.clone(), state);
        }

        Ok(Self {
            config,
            http_client,
            metrics,
            cache,
            rate_limiter,
            service_states: Arc::new(RwLock::new(service_states)),
            start_time: SystemTime::now(),
        })
    }

    /// Get next available service instance using load balancing
    pub async fn get_service_instance(&self, service_name: &str) -> FlowExResult<ServiceInstance> {
        let mut states = self.service_states.write().await;
        let state = states.get_mut(service_name)
            .ok_or_else(|| FlowExError::Internal(format!("Service not found: {}", service_name)))?;

        if state.healthy_instances.is_empty() {
            return Err(FlowExError::Internal(format!("No healthy instances for service: {}", service_name)));
        }

        let service_config = self.config.services.get(service_name)
            .ok_or_else(|| FlowExError::Internal(format!("Service config not found: {}", service_name)))?;

        let instance = match service_config.load_balancer {
            LoadBalancerType::RoundRobin => {
                let instance = state.healthy_instances[state.current_index].clone();
                state.current_index = (state.current_index + 1) % state.healthy_instances.len();
                instance
            }
            LoadBalancerType::WeightedRoundRobin => {
                // Simplified weighted round robin
                let total_weight: u32 = state.healthy_instances.iter().map(|i| i.weight).sum();
                let mut current_weight = 0;
                let target = (state.total_requests % total_weight as u64) as u32;
                
                for instance in &state.healthy_instances {
                    current_weight += instance.weight;
                    if current_weight > target {
                        return Ok(instance.clone());
                    }
                }
                state.healthy_instances[0].clone()
            }
            LoadBalancerType::Random => {
                let index = rand::random::<usize>() % state.healthy_instances.len();
                state.healthy_instances[index].clone()
            }
            LoadBalancerType::LeastConnections => {
                // For simplicity, use round robin (in production, track active connections)
                let instance = state.healthy_instances[state.current_index].clone();
                state.current_index = (state.current_index + 1) % state.healthy_instances.len();
                instance
            }
        };

        state.total_requests += 1;
        Ok(instance)
    }

    /// Record service request result
    pub async fn record_service_result(&self, service_name: &str, success: bool) {
        let mut states = self.service_states.write().await;
        if let Some(state) = states.get_mut(service_name) {
            if !success {
                state.failed_requests += 1;
            }
        }
    }
}

/// Health check endpoint
async fn health_check(State(state): State<AppState>) -> Json<HealthResponse> {
    let uptime = state.start_time.elapsed().unwrap_or_default().as_secs();

    Json(HealthResponse {
        status: "healthy".to_string(),
        service: "api-gateway".to_string(),
        version: "1.0.0".to_string(),
        timestamp: chrono::Utc::now(),
        uptime,
    })
}

/// Gateway statistics endpoint
async fn gateway_stats(State(state): State<AppState>) -> Json<ApiResponse<GatewayStats>> {
    let states = state.service_states.read().await;
    let mut service_stats = HashMap::new();

    for (service_name, service_state) in states.iter() {
        let stats = ServiceStats {
            healthy_instances: service_state.healthy_instances.len(),
            unhealthy_instances: service_state.unhealthy_instances.len(),
            total_requests: service_state.total_requests,
            failed_requests: service_state.failed_requests,
            error_rate: if service_state.total_requests > 0 {
                service_state.failed_requests as f64 / service_state.total_requests as f64
            } else {
                0.0
            },
        };
        service_stats.insert(service_name.clone(), stats);
    }

    let gateway_stats = GatewayStats {
        uptime_seconds: state.start_time.elapsed().unwrap_or_default().as_secs(),
        total_services: state.config.services.len(),
        service_stats,
    };

    Json(ApiResponse::success(gateway_stats))
}

/// Gateway statistics
#[derive(Debug, Serialize)]
pub struct GatewayStats {
    pub uptime_seconds: u64,
    pub total_services: usize,
    pub service_stats: HashMap<String, ServiceStats>,
}

/// Service statistics
#[derive(Debug, Serialize)]
pub struct ServiceStats {
    pub healthy_instances: usize,
    pub unhealthy_instances: usize,
    pub total_requests: u64,
    pub failed_requests: u64,
    pub error_rate: f64,
}

/// Proxy request to backend service
async fn proxy_request(
    State(state): State<AppState>,
    Path(service_name): Path<String>,
    method: Method,
    uri: Uri,
    headers: HeaderMap,
    body: Body,
) -> Result<Response<Body>, StatusCode> {
    let timer = state.metrics.start_timer();

    // Rate limiting
    if state.config.rate_limit.enabled {
        if state.rate_limiter.check().is_err() {
            state.metrics.record_http_request(&method.to_string(), &uri.path(), 429);
            return Err(StatusCode::TOO_MANY_REQUESTS);
        }
    }

    // Get service instance
    let instance = match state.get_service_instance(&service_name).await {
        Ok(instance) => instance,
        Err(_) => {
            state.metrics.record_http_request(&method.to_string(), &uri.path(), 503);
            return Err(StatusCode::SERVICE_UNAVAILABLE);
        }
    };

    // Build target URL
    let target_url = format!("http://{}:{}{}", instance.host, instance.port, uri.path_and_query().map(|pq| pq.as_str()).unwrap_or(""));

    // Forward request
    let mut request_builder = state.http_client.request(method.clone(), &target_url);

    // Forward headers (excluding hop-by-hop headers)
    for (name, value) in headers.iter() {
        if !is_hop_by_hop_header(name.as_str()) {
            request_builder = request_builder.header(name, value);
        }
    }

    // Convert body
    let body_bytes = match hyper::body::to_bytes(body).await {
        Ok(bytes) => bytes,
        Err(_) => {
            state.metrics.record_http_request(&method.to_string(), &uri.path(), 400);
            return Err(StatusCode::BAD_REQUEST);
        }
    };

    let response = match request_builder.body(body_bytes).send().await {
        Ok(response) => response,
        Err(_) => {
            state.record_service_result(&service_name, false).await;
            state.metrics.record_http_request(&method.to_string(), &uri.path(), 502);
            return Err(StatusCode::BAD_GATEWAY);
        }
    };

    // Record metrics
    let status_code = response.status().as_u16();
    let success = status_code < 400;
    state.record_service_result(&service_name, success).await;
    state.metrics.record_http_request(&method.to_string(), &uri.path(), status_code);
    timer.record_and_finish("flowex_gateway_request_duration_seconds", vec![
        ("service", service_name),
        ("method", method.to_string()),
    ]);

    // Convert response
    let mut response_builder = Response::builder().status(response.status());

    // Forward response headers
    for (name, value) in response.headers().iter() {
        if !is_hop_by_hop_header(name.as_str()) {
            response_builder = response_builder.header(name, value);
        }
    }

    let response_body = match response.bytes().await {
        Ok(bytes) => Body::from(bytes),
        Err(_) => return Err(StatusCode::BAD_GATEWAY),
    };

    response_builder.body(response_body).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

/// Check if header is hop-by-hop
fn is_hop_by_hop_header(name: &str) -> bool {
    matches!(name.to_lowercase().as_str(),
        "connection" | "keep-alive" | "proxy-authenticate" | "proxy-authorization" |
        "te" | "trailers" | "transfer-encoding" | "upgrade"
    )
}

/// Create the application router
fn create_app(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/gateway/stats", get(gateway_stats))
        .route("/api/:service/*path", any(proxy_request))
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(CompressionLayer::new())
                .layer(TimeoutLayer::new(Duration::from_secs(30)))
                .layer(CorsLayer::permissive())
                .into_inner(),
        )
        .with_state(state)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    info!("Starting FlowEx API Gateway");

    // Load configuration (simplified - in production use proper config management)
    let config = GatewayConfig {
        host: "0.0.0.0".to_string(),
        port: 8000,
        services: HashMap::from([
            ("auth".to_string(), ServiceConfig {
                name: "auth-service".to_string(),
                instances: vec![ServiceInstance {
                    id: "auth-1".to_string(),
                    host: "localhost".to_string(),
                    port: 8001,
                    weight: 1,
                    healthy: true,
                }],
                health_check_path: "/health".to_string(),
                load_balancer: LoadBalancerType::RoundRobin,
                circuit_breaker: CircuitBreakerConfig {
                    failure_threshold: 5,
                    timeout_seconds: 60,
                    half_open_max_calls: 3,
                },
            }),
            ("trading".to_string(), ServiceConfig {
                name: "trading-service".to_string(),
                instances: vec![ServiceInstance {
                    id: "trading-1".to_string(),
                    host: "localhost".to_string(),
                    port: 8002,
                    weight: 1,
                    healthy: true,
                }],
                health_check_path: "/health".to_string(),
                load_balancer: LoadBalancerType::RoundRobin,
                circuit_breaker: CircuitBreakerConfig {
                    failure_threshold: 5,
                    timeout_seconds: 60,
                    half_open_max_calls: 3,
                },
            }),
        ]),
        rate_limit: RateLimitConfig {
            requests_per_minute: 1000,
            burst_size: 100,
            enabled: true,
        },
        timeout_seconds: 30,
        max_request_size: 1024 * 1024, // 1MB
    };

    // Initialize cache (simplified - use proper Redis URL in production)
    let cache = CacheManager::new("redis://localhost:6379", Duration::from_secs(300)).await
        .map_err(|e| anyhow::anyhow!("Failed to initialize cache: {}", e))?;

    let state = AppState::new(config.clone(), cache).await?;
    let app = create_app(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    let listener = tokio::net::TcpListener::bind(addr).await?;
    
    info!("API Gateway listening on http://{}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}
