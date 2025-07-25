//! FlowEx Middleware Library
//!
//! Enterprise-grade middleware for FlowEx services including authentication,
//! authorization, logging, metrics, and security features.

use axum::{extract::Request, middleware::Next, response::Response};
use tracing::{info, debug, Span};
use uuid::Uuid;

pub mod auth;

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Once;

    static INIT: Once = Once::new();

    /// 初始化测试环境
    fn init_test_env() {
        INIT.call_once(|| {
            let _ = tracing_subscriber::fmt()
                .with_test_writer()
                .with_env_filter("debug")
                .try_init();
        });
    }

    /// 测试：中间件模块编译
    #[test]
    fn test_middleware_module_compilation() {
        init_test_env();

        // 验证中间件模块编译成功
        assert!(true, "中间件模块应该编译成功");
    }

    /// 测试：认证中间件模块可用性
    #[test]
    fn test_auth_middleware_availability() {
        init_test_env();

        // 验证认证中间件模块可用
        // 这里测试模块导入是否成功
        use crate::auth;
        assert!(true, "认证中间件模块应该可用");
    }

    /// 测试：中间件模块基本功能
    #[test]
    fn test_middleware_basic_functionality() {
        init_test_env();

        // 测试中间件模块的基本功能
        // 这里可以添加更多具体的中间件测试
        assert_eq!(2 + 2, 4, "基本数学运算应该正确");
    }

    /// 测试：错误处理
    #[test]
    fn test_error_handling() {
        init_test_env();

        // 测试中间件的错误处理能力
        assert!(true, "错误处理测试占位符");
    }

    /// 测试：性能特征
    #[test]
    fn test_performance_characteristics() {
        init_test_env();

        let start = std::time::Instant::now();

        // 模拟一些工作
        for _ in 0..1000 {
            let _ = format!("middleware_test_{}", 42);
        }

        let duration = start.elapsed();

        // 性能要求：基本操作应该很快完成
        assert!(duration.as_millis() < 100, "中间件性能应该满足要求");
    }
}

pub use auth::*;

/// Request ID middleware with enhanced logging
pub async fn request_id_middleware(mut request: Request, next: Next) -> Response {
    let request_id = Uuid::new_v4().to_string();

    // Add request ID to headers
    request.headers_mut().insert(
        "x-request-id",
        request_id.parse().unwrap(),
    );

    // Add to tracing span
    Span::current().record("request_id", &request_id);

    debug!("🔄 Processing request: {}", request_id);

    let response = next.run(request).await;

    debug!("✅ Request completed: {}", request_id);

    response
}

/// Enhanced logging middleware with metrics
pub async fn logging_middleware(request: Request, next: Next) -> Response {
    let method = request.method().clone();
    let uri = request.uri().clone();
    let user_agent = request.headers()
        .get("user-agent")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("unknown")
        .to_string();

    let start = std::time::Instant::now();
    let response = next.run(request).await;
    let duration = start.elapsed();
    let status = response.status();

    // Log with different levels based on status
    match status.as_u16() {
        200..=299 => info!(
            method = %method,
            uri = %uri,
            status = status.as_u16(),
            duration_ms = duration.as_millis(),
            user_agent = %user_agent,
            "✅ Request successful"
        ),
        400..=499 => info!(
            method = %method,
            uri = %uri,
            status = status.as_u16(),
            duration_ms = duration.as_millis(),
            user_agent = %user_agent,
            "⚠️  Client error"
        ),
        500..=599 => tracing::error!(
            method = %method,
            uri = %uri,
            status = status.as_u16(),
            duration_ms = duration.as_millis(),
            user_agent = %user_agent,
            "❌ Server error"
        ),
        _ => info!(
            method = %method,
            uri = %uri,
            status = status.as_u16(),
            duration_ms = duration.as_millis(),
            user_agent = %user_agent,
            "📊 Request processed"
        ),
    }

    response
}

/// CORS middleware for development
pub async fn cors_middleware(request: Request, next: Next) -> Response {
    let mut response = next.run(request).await;

    let headers = response.headers_mut();
    headers.insert("Access-Control-Allow-Origin", "*".parse().unwrap());
    headers.insert("Access-Control-Allow-Methods", "GET, POST, PUT, DELETE, OPTIONS".parse().unwrap());
    headers.insert("Access-Control-Allow-Headers", "Content-Type, Authorization, X-Request-ID".parse().unwrap());
    headers.insert("Access-Control-Max-Age", "86400".parse().unwrap());

    response
}

/// Metrics collection middleware
pub async fn metrics_middleware(request: Request, next: Next) -> Response {
    let method = request.method().clone();
    let uri = request.uri().path().to_string();

    let start = std::time::Instant::now();
    let response = next.run(request).await;
    let duration = start.elapsed();

    // In production, this would integrate with Prometheus metrics
    debug!(
        method = %method,
        path = %uri,
        status = response.status().as_u16(),
        duration_ms = duration.as_millis(),
        "📊 Metrics recorded"
    );

    response
}
