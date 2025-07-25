//! FlowEx Middleware Library
//!
//! Enterprise-grade middleware for FlowEx services including authentication,
//! authorization, logging, metrics, and security features.

use axum::{extract::Request, middleware::Next, response::Response};
use tracing::{info, debug, Span};
use uuid::Uuid;

pub mod auth;

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
