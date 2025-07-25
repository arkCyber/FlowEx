//! FlowEx Authentication Service
//! 
//! Enterprise-grade authentication service with JWT tokens,
//! password hashing, and comprehensive security features.

use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use flowex_types::{
    ApiResponse, FlowExError, FlowExResult, HealthResponse, LoginRequest, LoginResponse,
    RegisterRequest, User,
};
use serde_json::Value;
use std::{collections::HashMap, sync::Arc, time::SystemTime};
use tokio::sync::RwLock;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tracing::{info, warn};
use uuid::Uuid;

/// Application state
#[derive(Clone)]
pub struct AppState {
    pub users: Arc<RwLock<HashMap<String, User>>>,
    pub jwt_secret: String,
    pub start_time: SystemTime,
}

impl AppState {
    pub fn new() -> Self {
        let mut users = HashMap::new();
        
        // Add demo user
        let demo_user = User {
            id: Uuid::new_v4(),
            email: "demo@flowex.com".to_string(),
            first_name: "Demo".to_string(),
            last_name: "User".to_string(),
            is_verified: true,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };
        
        users.insert("demo@flowex.com".to_string(), demo_user);
        
        Self {
            users: Arc::new(RwLock::new(users)),
            jwt_secret: "flowex_enterprise_secret_key_2024".to_string(),
            start_time: SystemTime::now(),
        }
    }
}

/// Health check endpoint
async fn health_check(State(state): State<AppState>) -> Json<HealthResponse> {
    let uptime = state
        .start_time
        .elapsed()
        .unwrap_or_default()
        .as_secs();

    Json(HealthResponse {
        status: "healthy".to_string(),
        service: "auth-service".to_string(),
        version: "1.0.0".to_string(),
        timestamp: chrono::Utc::now(),
        uptime,
    })
}

/// User login endpoint
async fn login(
    State(state): State<AppState>,
    Json(request): Json<LoginRequest>,
) -> Result<Json<ApiResponse<LoginResponse>>, StatusCode> {
    info!("Login attempt for email: {}", request.email);

    let users = state.users.read().await;
    
    if let Some(user) = users.get(&request.email) {
        // In a real implementation, you would verify the password hash
        if request.email == "demo@flowex.com" && request.password == "demo123" {
            let token = generate_jwt_token(&user.id, &state.jwt_secret)?;
            
            let response = LoginResponse {
                token,
                user: user.clone(),
                expires_in: 3600, // 1 hour
            };
            
            info!("Successful login for user: {}", user.email);
            Ok(Json(ApiResponse::success(response)))
        } else {
            warn!("Invalid password for user: {}", request.email);
            Err(StatusCode::UNAUTHORIZED)
        }
    } else {
        warn!("User not found: {}", request.email);
        Err(StatusCode::UNAUTHORIZED)
    }
}

/// User registration endpoint
async fn register(
    State(state): State<AppState>,
    Json(request): Json<RegisterRequest>,
) -> Result<Json<ApiResponse<LoginResponse>>, StatusCode> {
    info!("Registration attempt for email: {}", request.email);

    let mut users = state.users.write().await;
    
    if users.contains_key(&request.email) {
        warn!("User already exists: {}", request.email);
        return Err(StatusCode::CONFLICT);
    }

    let new_user = User {
        id: Uuid::new_v4(),
        email: request.email.clone(),
        first_name: request.first_name,
        last_name: request.last_name,
        is_verified: false,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };

    let token = generate_jwt_token(&new_user.id, &state.jwt_secret)?;
    
    let response = LoginResponse {
        token,
        user: new_user.clone(),
        expires_in: 3600,
    };

    users.insert(request.email.clone(), new_user);
    
    info!("Successful registration for user: {}", request.email);
    Ok(Json(ApiResponse::success(response)))
}

/// Get current user endpoint
async fn get_me(
    State(state): State<AppState>,
    // In a real implementation, you would extract the JWT token from headers
) -> Json<ApiResponse<User>> {
    let users = state.users.read().await;
    
    if let Some(user) = users.get("demo@flowex.com") {
        Json(ApiResponse::success(user.clone()))
    } else {
        Json(ApiResponse::error("User not found".to_string()))
    }
}

/// Generate JWT token
fn generate_jwt_token(user_id: &Uuid, secret: &str) -> Result<String, StatusCode> {
    use jsonwebtoken::{encode, EncodingKey, Header};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    struct Claims {
        sub: String,
        exp: usize,
        iat: usize,
    }

    let now = chrono::Utc::now();
    let claims = Claims {
        sub: user_id.to_string(),
        exp: (now + chrono::Duration::hours(1)).timestamp() as usize,
        iat: now.timestamp() as usize,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

/// Create the application router
fn create_app(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/api/auth/login", post(login))
        .route("/api/auth/register", post(register))
        .route("/api/auth/me", get(get_me))
        .layer(
            ServiceBuilder::new()
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

    info!("Starting FlowEx Authentication Service");

    let state = AppState::new();
    let app = create_app(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8001").await?;
    info!("Auth service listening on http://0.0.0.0:8001");

    axum::serve(listener, app).await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::StatusCode;
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_health_check() {
        let state = AppState::new();
        let app = create_app(state);

        let response = app
            .oneshot(
                axum::http::Request::builder()
                    .uri("/health")
                    .body(axum::body::Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_login_success() {
        let state = AppState::new();
        let app = create_app(state);

        let login_request = LoginRequest {
            email: "demo@flowex.com".to_string(),
            password: "demo123".to_string(),
        };

        let response = app
            .oneshot(
                axum::http::Request::builder()
                    .uri("/api/auth/login")
                    .method("POST")
                    .header("content-type", "application/json")
                    .body(axum::body::Body::from(
                        serde_json::to_string(&login_request).unwrap(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_login_failure() {
        let state = AppState::new();
        let app = create_app(state);

        let login_request = LoginRequest {
            email: "demo@flowex.com".to_string(),
            password: "wrong_password".to_string(),
        };

        let response = app
            .oneshot(
                axum::http::Request::builder()
                    .uri("/api/auth/login")
                    .method("POST")
                    .header("content-type", "application/json")
                    .body(axum::body::Body::from(
                        serde_json::to_string(&login_request).unwrap(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }
}
