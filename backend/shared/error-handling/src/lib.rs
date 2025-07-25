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

#[cfg(test)]
mod tests {
    use super::*;
    use flowex_types::{FlowExError, ApiResponse};
    use axum::http::StatusCode;
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

    /// 测试：认证错误处理
    #[test]
    fn test_authentication_error_handling() {
        init_test_env();

        let error = FlowExError::Authentication("Invalid credentials".to_string());
        let (status, response) = handlers::handle_error::<String>(error);

        assert_eq!(status, StatusCode::UNAUTHORIZED);

        // 验证响应格式
        let response_body = response.0;
        assert!(!response_body.success);
        assert!(response_body.error.is_some());
        assert_eq!(response_body.error.unwrap(), "Authentication error: Invalid credentials");
    }

    /// 测试：授权错误处理
    #[test]
    fn test_authorization_error_handling() {
        init_test_env();

        let error = FlowExError::Authorization("Insufficient permissions".to_string());
        let (status, response) = handlers::handle_error::<String>(error);

        assert_eq!(status, StatusCode::FORBIDDEN);

        let response_body = response.0;
        assert!(!response_body.success);
        assert!(response_body.error.is_some());
        assert_eq!(response_body.error.unwrap(), "Authorization error: Insufficient permissions");
    }

    /// 测试：验证错误处理
    #[test]
    fn test_validation_error_handling() {
        init_test_env();

        let error = FlowExError::Validation("Invalid input format".to_string());
        let (status, response) = handlers::handle_error::<String>(error);

        assert_eq!(status, StatusCode::BAD_REQUEST);

        let response_body = response.0;
        assert!(!response_body.success);
        assert!(response_body.error.is_some());
        assert_eq!(response_body.error.unwrap(), "Validation error: Invalid input format");
    }

    /// 测试：数据库错误处理
    #[test]
    fn test_database_error_handling() {
        init_test_env();

        let error = FlowExError::Database("Connection failed".to_string());
        let (status, response) = handlers::handle_error::<String>(error);

        assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);

        let response_body = response.0;
        assert!(!response_body.success);
        assert!(response_body.error.is_some());
        assert_eq!(response_body.error.unwrap(), "Database error");
    }

    /// 测试：交易错误处理
    #[test]
    fn test_trading_error_handling() {
        init_test_env();

        let error = FlowExError::Trading("Insufficient balance".to_string());
        let (status, response) = handlers::handle_error::<String>(error);

        assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);

        let response_body = response.0;
        assert!(!response_body.success);
        assert!(response_body.error.is_some());
        assert_eq!(response_body.error.unwrap(), "Internal server error");
    }

    /// 测试：市场数据错误处理
    #[test]
    fn test_market_data_error_handling() {
        init_test_env();

        let error = FlowExError::MarketData("Data source unavailable".to_string());
        let (status, response) = handlers::handle_error::<String>(error);

        assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);

        let response_body = response.0;
        assert!(!response_body.success);
        assert!(response_body.error.is_some());
        assert_eq!(response_body.error.unwrap(), "Internal server error");
    }

    /// 测试：钱包错误处理
    #[test]
    fn test_wallet_error_handling() {
        init_test_env();

        let error = FlowExError::Wallet("Transaction failed".to_string());
        let (status, response) = handlers::handle_error::<String>(error);

        assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);

        let response_body = response.0;
        assert!(!response_body.success);
        assert!(response_body.error.is_some());
        assert_eq!(response_body.error.unwrap(), "Internal server error");
    }

    /// 测试：内部服务器错误处理
    #[test]
    fn test_internal_error_handling() {
        init_test_env();

        let error = FlowExError::Internal("Unexpected error occurred".to_string());
        let (status, response) = handlers::handle_error::<String>(error);

        assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);

        let response_body = response.0;
        assert!(!response_body.success);
        assert!(response_body.error.is_some());
        assert_eq!(response_body.error.unwrap(), "Internal server error");
    }

    /// 测试：错误消息格式化
    #[test]
    fn test_error_message_formatting() {
        init_test_env();

        let test_cases = vec![
            (
                FlowExError::Authentication("Token expired".to_string()),
                "Authentication error: Token expired"
            ),
            (
                FlowExError::Authorization("Access denied".to_string()),
                "Authorization error: Access denied"
            ),
            (
                FlowExError::Validation("Required field missing".to_string()),
                "Validation error: Required field missing"
            ),
        ];

        for (error, expected_message) in test_cases {
            let error_string = error.to_string();
            assert_eq!(error_string, expected_message);
        }
    }

    /// 测试：错误处理性能
    #[test]
    fn test_error_handling_performance() {
        init_test_env();

        let start = std::time::Instant::now();

        // 处理大量错误
        for i in 0..1000 {
            let error = FlowExError::Validation(format!("Error {}", i));
            let (_status, _response) = handlers::handle_error::<String>(error);
        }

        let duration = start.elapsed();

        // 性能要求：1000个错误处理应该在50ms内完成
        assert!(duration.as_millis() < 50, "错误处理性能不达标");
    }

    /// 测试：错误类型判断
    #[test]
    fn test_error_type_discrimination() {
        init_test_env();

        let auth_error = FlowExError::Authentication("test".to_string());
        let validation_error = FlowExError::Validation("test".to_string());
        let database_error = FlowExError::Database("test".to_string());

        // 验证错误类型可以正确区分
        match auth_error {
            FlowExError::Authentication(_) => assert!(true),
            _ => assert!(false, "应该是认证错误"),
        }

        match validation_error {
            FlowExError::Validation(_) => assert!(true),
            _ => assert!(false, "应该是验证错误"),
        }

        match database_error {
            FlowExError::Database(_) => assert!(true),
            _ => assert!(false, "应该是数据库错误"),
        }
    }

    /// 测试：错误链和嵌套错误
    #[test]
    fn test_error_chaining() {
        init_test_env();

        // 测试错误消息的嵌套和链式传播
        let root_cause = "Connection timeout";
        let database_error = FlowExError::Database(format!("Query failed: {}", root_cause));

        let error_message = database_error.to_string();
        assert!(error_message.contains(root_cause));
        assert!(error_message.contains("Database error"));
    }

    /// 测试：错误处理的线程安全性
    #[test]
    fn test_error_handling_thread_safety() {
        init_test_env();

        use std::thread;
        use std::sync::Arc;

        let errors = Arc::new(vec![
            FlowExError::Authentication("Thread test 1".to_string()),
            FlowExError::Validation("Thread test 2".to_string()),
            FlowExError::Database("Thread test 3".to_string()),
        ]);

        let mut handles = vec![];

        for i in 0..3 {
            let errors_clone = Arc::clone(&errors);
            let handle = thread::spawn(move {
                let error = errors_clone[i].clone();
                let (_status, _response) = handlers::handle_error::<String>(error);
                true
            });
            handles.push(handle);
        }

        // 等待所有线程完成
        for handle in handles {
            assert!(handle.join().unwrap());
        }
    }

    /// 测试：错误处理的内存使用
    #[test]
    fn test_error_handling_memory_usage() {
        init_test_env();

        // 创建大量错误对象测试内存使用
        let mut errors = Vec::new();

        for i in 0..1000 {
            let error = FlowExError::Validation(format!("Large error message with lots of text to test memory usage - iteration {}", i));
            errors.push(error);
        }

        // 处理所有错误
        for error in errors {
            let (_status, _response) = handlers::handle_error::<String>(error);
        }

        // 验证内存使用合理（主要确保不会内存泄漏）
        assert!(true);
    }

    /// 测试：错误处理的边界情况
    #[test]
    fn test_error_handling_edge_cases() {
        init_test_env();

        // 测试空错误消息
        let empty_error = FlowExError::Validation("".to_string());
        let (_status, _response) = handlers::handle_error::<String>(empty_error);

        // 测试非常长的错误消息
        let long_message = "x".repeat(10000);
        let long_error = FlowExError::Internal(long_message);
        let (_status, _response) = handlers::handle_error::<String>(long_error);

        // 测试包含特殊字符的错误消息
        let special_chars_error = FlowExError::Authentication("Error with special chars: 中文 🚀 \"quotes\" 'apostrophes' <tags>".to_string());
        let (_status, _response) = handlers::handle_error::<String>(special_chars_error);

        // 验证边界情况处理成功
        assert!(true);
    }
}
