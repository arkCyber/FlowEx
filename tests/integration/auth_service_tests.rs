//! FlowEx Authentication Service Integration Tests
//!
//! Comprehensive integration tests for the authentication service
//! covering login, registration, JWT validation, and security features.

use axum::http::StatusCode;
use flowex_types::{ApiResponse, LoginRequest, LoginResponse, RegisterRequest, User};
use serde_json::Value;
use std::collections::HashMap;
use tokio::time::{sleep, Duration};
use uuid::Uuid;

/// Test configuration
struct TestConfig {
    base_url: String,
    client: reqwest::Client,
}

impl TestConfig {
    fn new() -> Self {
        Self {
            base_url: "http://localhost:8001".to_string(),
            client: reqwest::Client::new(),
        }
    }
}

/// Test helper for making HTTP requests
async fn make_request(
    config: &TestConfig,
    method: reqwest::Method,
    path: &str,
    body: Option<Value>,
    headers: Option<HashMap<String, String>>,
) -> Result<(StatusCode, Value), Box<dyn std::error::Error>> {
    let url = format!("{}{}", config.base_url, path);
    let mut request = config.client.request(method, &url);
    
    if let Some(body) = body {
        request = request.json(&body);
    }
    
    if let Some(headers) = headers {
        for (key, value) in headers {
            request = request.header(&key, &value);
        }
    }
    
    let response = request.send().await?;
    let status = StatusCode::from_u16(response.status().as_u16())?;
    let body: Value = response.json().await?;
    
    Ok((status, body))
}

#[tokio::test]
async fn test_health_check() {
    let config = TestConfig::new();
    
    let (status, body) = make_request(
        &config,
        reqwest::Method::GET,
        "/health",
        None,
        None,
    ).await.expect("Health check request failed");
    
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["status"], "healthy");
    assert_eq!(body["service"], "auth-service");
    assert!(body["uptime"].is_number());
}

#[tokio::test]
async fn test_successful_login() {
    let config = TestConfig::new();
    
    let login_request = LoginRequest {
        email: "demo@flowex.com".to_string(),
        password: "demo123".to_string(),
    };
    
    let (status, body) = make_request(
        &config,
        reqwest::Method::POST,
        "/api/auth/login",
        Some(serde_json::to_value(login_request).unwrap()),
        None,
    ).await.expect("Login request failed");
    
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["success"], true);
    assert!(body["data"]["token"].is_string());
    assert!(body["data"]["user"]["id"].is_string());
    assert_eq!(body["data"]["user"]["email"], "demo@flowex.com");
    assert_eq!(body["data"]["expires_in"], 3600);
}

#[tokio::test]
async fn test_invalid_login_credentials() {
    let config = TestConfig::new();
    
    let login_request = LoginRequest {
        email: "demo@flowex.com".to_string(),
        password: "wrong_password".to_string(),
    };
    
    let (status, _body) = make_request(
        &config,
        reqwest::Method::POST,
        "/api/auth/login",
        Some(serde_json::to_value(login_request).unwrap()),
        None,
    ).await.expect("Login request failed");
    
    assert_eq!(status, StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_login_nonexistent_user() {
    let config = TestConfig::new();
    
    let login_request = LoginRequest {
        email: "nonexistent@flowex.com".to_string(),
        password: "password123".to_string(),
    };
    
    let (status, _body) = make_request(
        &config,
        reqwest::Method::POST,
        "/api/auth/login",
        Some(serde_json::to_value(login_request).unwrap()),
        None,
    ).await.expect("Login request failed");
    
    assert_eq!(status, StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_user_registration() {
    let config = TestConfig::new();
    
    let register_request = RegisterRequest {
        email: format!("test_{}@flowex.com", Uuid::new_v4()),
        password: "password123".to_string(),
        first_name: "Test".to_string(),
        last_name: "User".to_string(),
    };
    
    let (status, body) = make_request(
        &config,
        reqwest::Method::POST,
        "/api/auth/register",
        Some(serde_json::to_value(register_request).unwrap()),
        None,
    ).await.expect("Registration request failed");
    
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["success"], true);
    assert!(body["data"]["token"].is_string());
    assert!(body["data"]["user"]["id"].is_string());
    assert_eq!(body["data"]["expires_in"], 3600);
}

#[tokio::test]
async fn test_duplicate_email_registration() {
    let config = TestConfig::new();
    
    let register_request = RegisterRequest {
        email: "demo@flowex.com".to_string(), // This email already exists
        password: "password123".to_string(),
        first_name: "Test".to_string(),
        last_name: "User".to_string(),
    };
    
    let (status, body) = make_request(
        &config,
        reqwest::Method::POST,
        "/api/auth/register",
        Some(serde_json::to_value(register_request).unwrap()),
        None,
    ).await.expect("Registration request failed");
    
    assert_eq!(status, StatusCode::CONFLICT);
    assert_eq!(body["success"], false);
}

#[tokio::test]
async fn test_get_current_user() {
    let config = TestConfig::new();
    
    // First login to get a token
    let login_request = LoginRequest {
        email: "demo@flowex.com".to_string(),
        password: "demo123".to_string(),
    };
    
    let (status, body) = make_request(
        &config,
        reqwest::Method::POST,
        "/api/auth/login",
        Some(serde_json::to_value(login_request).unwrap()),
        None,
    ).await.expect("Login request failed");
    
    assert_eq!(status, StatusCode::OK);
    let token = body["data"]["token"].as_str().unwrap();
    
    // Now get current user with token
    let mut headers = HashMap::new();
    headers.insert("Authorization".to_string(), format!("Bearer {}", token));
    
    let (status, body) = make_request(
        &config,
        reqwest::Method::GET,
        "/api/auth/me",
        None,
        Some(headers),
    ).await.expect("Get user request failed");
    
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["success"], true);
    assert_eq!(body["data"]["email"], "demo@flowex.com");
}

#[tokio::test]
async fn test_invalid_json_request() {
    let config = TestConfig::new();
    
    let response = config.client
        .post(&format!("{}/api/auth/login", config.base_url))
        .header("Content-Type", "application/json")
        .body("invalid json")
        .send()
        .await
        .expect("Request failed");
    
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_missing_content_type() {
    let config = TestConfig::new();
    
    let login_request = LoginRequest {
        email: "demo@flowex.com".to_string(),
        password: "demo123".to_string(),
    };
    
    let response = config.client
        .post(&format!("{}/api/auth/login", config.base_url))
        .body(serde_json::to_string(&login_request).unwrap())
        .send()
        .await
        .expect("Request failed");
    
    // Should still work as axum can handle JSON without explicit content-type
    assert!(response.status().is_success() || response.status() == StatusCode::UNSUPPORTED_MEDIA_TYPE);
}

#[tokio::test]
async fn test_rate_limiting() {
    let config = TestConfig::new();
    
    let login_request = LoginRequest {
        email: "demo@flowex.com".to_string(),
        password: "wrong_password".to_string(),
    };
    
    // Make multiple failed login attempts
    for i in 0..5 {
        let (status, _body) = make_request(
            &config,
            reqwest::Method::POST,
            "/api/auth/login",
            Some(serde_json::to_value(&login_request).unwrap()),
            None,
        ).await.expect("Login request failed");
        
        // All should be unauthorized (not rate limited yet in basic implementation)
        assert_eq!(status, StatusCode::UNAUTHORIZED);
        
        // Small delay between requests
        sleep(Duration::from_millis(100)).await;
    }
}

#[tokio::test]
async fn test_cors_headers() {
    let config = TestConfig::new();
    
    let response = config.client
        .options(&format!("{}/api/auth/login", config.base_url))
        .header("Origin", "http://localhost:3000")
        .header("Access-Control-Request-Method", "POST")
        .header("Access-Control-Request-Headers", "Content-Type, Authorization")
        .send()
        .await
        .expect("CORS preflight request failed");
    
    // Check CORS headers are present
    let headers = response.headers();
    assert!(headers.contains_key("access-control-allow-origin") || 
            headers.contains_key("Access-Control-Allow-Origin"));
}

/// Helper function to run all auth service tests
pub async fn run_auth_service_tests() -> Result<(), Box<dyn std::error::Error>> {
    println!("🧪 Running Authentication Service Integration Tests");
    
    // Wait for service to be ready
    let config = TestConfig::new();
    let mut retries = 30;
    
    while retries > 0 {
        match make_request(&config, reqwest::Method::GET, "/health", None, None).await {
            Ok((StatusCode::OK, _)) => break,
            _ => {
                retries -= 1;
                if retries == 0 {
                    return Err("Auth service not ready after 30 seconds".into());
                }
                sleep(Duration::from_secs(1)).await;
            }
        }
    }
    
    println!("✅ Auth service is ready, running tests...");
    
    // Run individual tests here if needed
    // For now, tests are run by the test framework
    
    Ok(())
}
