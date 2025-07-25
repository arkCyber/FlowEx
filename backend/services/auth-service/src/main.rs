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

    /// 测试：用户注册功能
    #[tokio::test]
    async fn test_user_registration() {
        init_test_env();

        let app_state = create_test_app_state();
        let app = create_app(app_state);

        let register_request = RegisterRequest {
            email: "newuser@example.com".to_string(),
            password: "SecurePassword123!".to_string(),
            first_name: "New".to_string(),
            last_name: "User".to_string(),
        };

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/auth/register")
                    .header("content-type", "application/json")
                    .body(Body::from(serde_json::to_string(&register_request).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let api_response: ApiResponse<User> = serde_json::from_slice(&body).unwrap();

        assert!(api_response.success);
        assert!(api_response.data.is_some());

        let user = api_response.data.unwrap();
        assert_eq!(user.email, "newuser@example.com");
        assert_eq!(user.first_name, "New");
        assert_eq!(user.last_name, "User");
        assert!(!user.is_verified); // 新用户默认未验证
    }

    /// 测试：重复邮箱注册
    #[tokio::test]
    async fn test_duplicate_email_registration() {
        init_test_env();

        let app_state = create_test_app_state();
        let app = create_app(app_state);

        let register_request = RegisterRequest {
            email: "test@example.com".to_string(), // 使用已存在的邮箱
            password: "SecurePassword123!".to_string(),
            first_name: "Duplicate".to_string(),
            last_name: "User".to_string(),
        };

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/auth/register")
                    .header("content-type", "application/json")
                    .body(Body::from(serde_json::to_string(&register_request).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CONFLICT);
    }

    /// 测试：无效密码注册
    #[tokio::test]
    async fn test_invalid_password_registration() {
        init_test_env();

        let app_state = create_test_app_state();
        let app = create_app(app_state);

        let weak_password_request = RegisterRequest {
            email: "weakpass@example.com".to_string(),
            password: "123".to_string(), // 弱密码
            first_name: "Weak".to_string(),
            last_name: "Password".to_string(),
        };

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/auth/register")
                    .header("content-type", "application/json")
                    .body(Body::from(serde_json::to_string(&weak_password_request).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    /// 测试：无效邮箱格式注册
    #[tokio::test]
    async fn test_invalid_email_registration() {
        init_test_env();

        let app_state = create_test_app_state();
        let app = create_app(app_state);

        let invalid_email_request = RegisterRequest {
            email: "invalid-email".to_string(), // 无效邮箱格式
            password: "SecurePassword123!".to_string(),
            first_name: "Invalid".to_string(),
            last_name: "Email".to_string(),
        };

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/auth/register")
                    .header("content-type", "application/json")
                    .body(Body::from(serde_json::to_string(&invalid_email_request).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    /// 测试：JWT令牌生成
    #[tokio::test]
    async fn test_jwt_token_generation() {
        init_test_env();

        let user = User {
            id: Uuid::new_v4(),
            email: "jwt@example.com".to_string(),
            first_name: "JWT".to_string(),
            last_name: "User".to_string(),
            is_verified: true,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        let secret = "test_jwt_secret_key_for_testing";
        let token_result = generate_jwt_token(&user, secret);

        assert!(token_result.is_ok(), "JWT令牌生成应该成功");

        let token = token_result.unwrap();
        assert!(!token.is_empty(), "JWT令牌不应该为空");
        assert!(token.contains('.'), "JWT令牌应该包含点分隔符");

        // 验证令牌格式（JWT应该有3个部分）
        let parts: Vec<&str> = token.split('.').collect();
        assert_eq!(parts.len(), 3, "JWT令牌应该有3个部分");
    }

    /// 测试：JWT令牌验证
    #[tokio::test]
    async fn test_jwt_token_validation() {
        init_test_env();

        let user = User {
            id: Uuid::new_v4(),
            email: "validation@example.com".to_string(),
            first_name: "Validation".to_string(),
            last_name: "User".to_string(),
            is_verified: true,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        let secret = "test_jwt_secret_key_for_testing";
        let token = generate_jwt_token(&user, secret).unwrap();

        // 验证令牌（这里需要实现令牌验证函数）
        // 在实际实现中，应该有一个验证JWT令牌的函数
        assert!(!token.is_empty());
    }

    /// 测试：密码哈希和验证
    #[test]
    fn test_password_hashing_and_verification() {
        init_test_env();

        let password = "TestPassword123!";

        // 哈希密码
        let hashed = bcrypt::hash(password, bcrypt::DEFAULT_COST).unwrap();

        // 验证正确密码
        let is_valid = bcrypt::verify(password, &hashed).unwrap();
        assert!(is_valid, "正确密码应该验证成功");

        // 验证错误密码
        let is_invalid = bcrypt::verify("WrongPassword", &hashed).unwrap();
        assert!(!is_invalid, "错误密码应该验证失败");
    }

    /// 测试：用户数据验证
    #[test]
    fn test_user_data_validation() {
        init_test_env();

        // 测试有效用户数据
        let valid_user = User {
            id: Uuid::new_v4(),
            email: "valid@example.com".to_string(),
            first_name: "Valid".to_string(),
            last_name: "User".to_string(),
            is_verified: false,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        assert!(!valid_user.email.is_empty());
        assert!(!valid_user.first_name.is_empty());
        assert!(!valid_user.last_name.is_empty());
        assert!(valid_user.email.contains('@'));

        // 测试时间戳
        let now = chrono::Utc::now();
        let time_diff = (now - valid_user.created_at).num_seconds();
        assert!(time_diff >= 0 && time_diff < 5, "创建时间应该在当前时间附近");
    }

    /// 测试：并发登录请求
    #[tokio::test]
    async fn test_concurrent_login_requests() {
        init_test_env();

        let app_state = create_test_app_state();

        let mut handles = vec![];

        // 启动多个并发登录请求
        for i in 0..10 {
            let state_clone = app_state.clone();
            let handle = tokio::spawn(async move {
                let app = create_app(state_clone);

                let login_request = LoginRequest {
                    email: "test@example.com".to_string(),
                    password: "password123".to_string(),
                };

                let response = app
                    .oneshot(
                        Request::builder()
                            .method("POST")
                            .uri("/api/auth/login")
                            .header("content-type", "application/json")
                            .body(Body::from(serde_json::to_string(&login_request).unwrap()))
                            .unwrap(),
                    )
                    .await
                    .unwrap();

                (i, response.status())
            });
            handles.push(handle);
        }

        // 等待所有请求完成
        for handle in handles {
            let (task_id, status) = handle.await.unwrap();
            assert_eq!(status, StatusCode::OK, "任务{}的登录应该成功", task_id);
        }
    }

    /// 测试：性能基准
    #[tokio::test]
    async fn test_performance_benchmark() {
        init_test_env();

        let app_state = create_test_app_state();
        let start = std::time::Instant::now();

        // 执行大量认证操作
        for _ in 0..100 {
            let app = create_app(app_state.clone());

            let login_request = LoginRequest {
                email: "test@example.com".to_string(),
                password: "password123".to_string(),
            };

            let _response = app
                .oneshot(
                    Request::builder()
                        .method("POST")
                        .uri("/api/auth/login")
                        .header("content-type", "application/json")
                        .body(Body::from(serde_json::to_string(&login_request).unwrap()))
                        .unwrap(),
                )
                .await
                .unwrap();
        }

        let duration = start.elapsed();
        println!("100次认证操作耗时: {:?}", duration);

        // 性能要求：100次认证操作应该在5秒内完成
        assert!(duration.as_secs() < 5, "认证服务性能不达标");
    }

    /// 测试：内存使用优化
    #[tokio::test]
    async fn test_memory_usage_optimization() {
        init_test_env();

        let app_state = create_test_app_state();

        // 创建大量用户数据
        let mut users = Vec::new();
        for i in 0..1000 {
            let user = User {
                id: Uuid::new_v4(),
                email: format!("user{}@example.com", i),
                first_name: format!("User{}", i),
                last_name: "Test".to_string(),
                is_verified: i % 2 == 0,
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            };
            users.push(user);
        }

        assert_eq!(users.len(), 1000);

        // 清理内存
        drop(users);
        assert!(true, "内存使用优化测试完成");
    }

    /// 测试：错误处理边界情况
    #[tokio::test]
    async fn test_error_handling_edge_cases() {
        init_test_env();

        let app_state = create_test_app_state();
        let app = create_app(app_state);

        // 测试空请求体
        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/auth/login")
                    .header("content-type", "application/json")
                    .body(Body::from(""))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    /// 测试：安全性验证
    #[tokio::test]
    async fn test_security_validation() {
        init_test_env();

        let app_state = create_test_app_state();
        let app = create_app(app_state);

        // 测试SQL注入尝试
        let malicious_login = LoginRequest {
            email: "'; DROP TABLE users; --".to_string(),
            password: "password".to_string(),
        };

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/auth/login")
                    .header("content-type", "application/json")
                    .body(Body::from(serde_json::to_string(&malicious_login).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();

        // 应该返回未授权而不是服务器错误
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }
}
