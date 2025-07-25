//! FlowEx Error Handling Library
//!
//! Centralized error handling for FlowEx services.

pub use flowex_types::FlowExError;

/// Error handling utilities
pub mod handlers {
    use axum::{http::StatusCode, response::Json};
    use flowex_types::ApiResponse;
    use tracing::error;
    
    /// Convert FlowExError to HTTP response
    pub fn handle_error<T>(err: super::FlowExError) -> (StatusCode, Json<ApiResponse<T>>) {
        error!("Request failed: {}", err);
        
        let (status, message) = match err {
            super::FlowExError::Authentication(_) => (StatusCode::UNAUTHORIZED, err.to_string()),
            super::FlowExError::Authorization(_) => (StatusCode::FORBIDDEN, err.to_string()),
            super::FlowExError::Validation(_) => (StatusCode::BAD_REQUEST, err.to_string()),
            super::FlowExError::Database(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database error".to_string()),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string()),
        };
        
        (status, Json(ApiResponse::error(message)))
    }
}
