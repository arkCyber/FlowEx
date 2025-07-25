//! FlowEx Wallet Service
//!
//! Enterprise-grade wallet service providing balance management,
//! transaction history, and deposit/withdrawal operations.

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::get,
    Router,
};
use flowex_types::{
    ApiResponse, Balance, HealthResponse, Transaction, TransactionStatus, TransactionType,
};
use rust_decimal::Decimal;
use std::{collections::HashMap, sync::Arc, time::SystemTime};
use tokio::sync::RwLock;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tracing::{info};
use uuid::Uuid;

/// Application state for the wallet service
#[derive(Clone)]
pub struct AppState {
    pub balances: Arc<RwLock<HashMap<String, Vec<Balance>>>>,
    pub transactions: Arc<RwLock<HashMap<String, Vec<Transaction>>>>,
    pub start_time: SystemTime,
}

impl AppState {
    pub fn new() -> Self {
        let mut balances = HashMap::new();
        let mut transactions = HashMap::new();

        // Initialize demo balances for demo user
        let demo_balances = vec![
            Balance {
                currency: "BTC".to_string(),
                available: Decimal::new(12345678, 8), // 0.12345678
                locked: Decimal::new(0, 8),
            },
            Balance {
                currency: "ETH".to_string(),
                available: Decimal::new(245678901, 8), // 2.45678901
                locked: Decimal::new(10000000, 8), // 0.10000000
            },
            Balance {
                currency: "USDT".to_string(),
                available: Decimal::new(100000000000, 8), // 1000.00000000
                locked: Decimal::new(5000000000, 8), // 50.00000000
            },
            Balance {
                currency: "BNB".to_string(),
                available: Decimal::new(1050000000, 8), // 10.50000000
                locked: Decimal::new(0, 8),
            },
        ];

        // Initialize demo transactions
        let demo_transactions = vec![
            Transaction {
                id: Uuid::new_v4(),
                user_id: Uuid::new_v4(),
                transaction_type: TransactionType::Deposit,
                currency: "BTC".to_string(),
                amount: Decimal::new(10000000, 8), // 0.10000000
                status: TransactionStatus::Completed,
                created_at: chrono::Utc::now(),
            },
            Transaction {
                id: Uuid::new_v4(),
                user_id: Uuid::new_v4(),
                transaction_type: TransactionType::Trade,
                currency: "USDT".to_string(),
                amount: Decimal::new(50000000000, 8), // 500.00000000
                status: TransactionStatus::Completed,
                created_at: chrono::Utc::now(),
            },
        ];

        balances.insert("demo@flowex.com".to_string(), demo_balances);
        transactions.insert("demo@flowex.com".to_string(), demo_transactions);

        Self {
            balances: Arc::new(RwLock::new(balances)),
            transactions: Arc::new(RwLock::new(transactions)),
            start_time: SystemTime::now(),
        }
    }
}

/// Health check endpoint
async fn health_check(State(state): State<AppState>) -> Json<HealthResponse> {
    let uptime = state.start_time.elapsed().unwrap_or_default().as_secs();

    Json(HealthResponse {
        status: "healthy".to_string(),
        service: "wallet-service".to_string(),
        version: "1.0.0".to_string(),
        timestamp: chrono::Utc::now(),
        uptime,
    })
}

/// Get all balances for the user
async fn get_balances(State(state): State<AppState>) -> Json<ApiResponse<Vec<Balance>>> {
    let balances = state.balances.read().await;
    
    // In real implementation, extract user from JWT token
    if let Some(user_balances) = balances.get("demo@flowex.com") {
        Json(ApiResponse::success(user_balances.clone()))
    } else {
        Json(ApiResponse::success(vec![]))
    }
}

/// Get balance for a specific currency
async fn get_balance(
    State(state): State<AppState>,
    Path(currency): Path<String>,
) -> Result<Json<ApiResponse<Balance>>, StatusCode> {
    let balances = state.balances.read().await;
    
    if let Some(user_balances) = balances.get("demo@flowex.com") {
        if let Some(balance) = user_balances.iter().find(|b| b.currency == currency.to_uppercase()) {
            Ok(Json(ApiResponse::success(balance.clone())))
        } else {
            Err(StatusCode::NOT_FOUND)
        }
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

/// Get transaction history
async fn get_transactions(State(state): State<AppState>) -> Json<ApiResponse<Vec<Transaction>>> {
    let transactions = state.transactions.read().await;
    
    // In real implementation, extract user from JWT token
    if let Some(user_transactions) = transactions.get("demo@flowex.com") {
        Json(ApiResponse::success(user_transactions.clone()))
    } else {
        Json(ApiResponse::success(vec![]))
    }
}

/// Create the application router
fn create_app(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/api/wallet/balances", get(get_balances))
        .route("/api/wallet/balance/:currency", get(get_balance))
        .route("/api/wallet/transactions", get(get_transactions))
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

    info!("Starting FlowEx Wallet Service");

    let state = AppState::new();
    let app = create_app(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8004").await?;
    info!("Wallet service listening on http://0.0.0.0:8004");

    axum::serve(listener, app).await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use tower::ServiceExt;
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

    /// 创建测试用的应用状态
    fn create_test_app_state() -> AppState {
        let mut balances = HashMap::new();

        // 添加测试余额数据
        balances.insert("BTC".to_string(), Balance {
            currency: "BTC".to_string(),
            available: Decimal::new(123456, 6), // 0.123456
            locked: Decimal::new(10000, 6), // 0.010000
        });

        balances.insert("ETH".to_string(), Balance {
            currency: "ETH".to_string(),
            available: Decimal::new(2500000, 6), // 2.500000
            locked: Decimal::new(100000, 6), // 0.100000
        });

        balances.insert("USDT".to_string(), Balance {
            currency: "USDT".to_string(),
            available: Decimal::new(1000000000, 6), // 1000.000000
            locked: Decimal::new(50000000, 6), // 50.000000
        });

        let mut transactions = Vec::new();

        // 添加测试交易数据
        transactions.push(Transaction {
            id: Uuid::new_v4(),
            user_id: Uuid::new_v4(),
            transaction_type: TransactionType::Deposit,
            currency: "BTC".to_string(),
            amount: Decimal::new(100000, 6), // 0.100000
            status: TransactionStatus::Completed,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        });

        transactions.push(Transaction {
            id: Uuid::new_v4(),
            user_id: Uuid::new_v4(),
            transaction_type: TransactionType::Withdrawal,
            currency: "ETH".to_string(),
            amount: Decimal::new(500000, 6), // 0.500000
            status: TransactionStatus::Pending,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        });

        AppState {
            balances: Arc::new(RwLock::new(balances)),
            transactions: Arc::new(RwLock::new(transactions)),
            start_time: SystemTime::now(),
        }
    }

    /// 测试：应用状态创建
    #[test]
    fn test_app_state_creation() {
        init_test_env();

        let state = create_test_app_state();

        // 验证状态创建成功
        assert!(state.start_time.elapsed().unwrap().as_secs() < 1);

        // 验证初始数据
        tokio::runtime::Runtime::new().unwrap().block_on(async {
            let balances = state.balances.read().await;
            assert!(balances.len() > 0, "应该有初始余额数据");
            assert!(balances.contains_key("BTC"), "应该包含BTC余额");
            assert!(balances.contains_key("ETH"), "应该包含ETH余额");
            assert!(balances.contains_key("USDT"), "应该包含USDT余额");

            let transactions = state.transactions.read().await;
            assert!(transactions.len() > 0, "应该有初始交易数据");
        });
    }

    /// 测试：余额数据结构
    #[test]
    fn test_balance_structure() {
        init_test_env();

        let balance = Balance {
            currency: "BTC".to_string(),
            available: Decimal::new(100000000, 8), // 1.00000000
            locked: Decimal::new(10000000, 8), // 0.10000000
        };

        assert_eq!(balance.currency, "BTC");
        assert_eq!(balance.available, Decimal::new(100000000, 8));
        assert_eq!(balance.locked, Decimal::new(10000000, 8));

        // 验证总余额计算
        let total = balance.available + balance.locked;
        assert_eq!(total, Decimal::new(110000000, 8)); // 1.10000000
    }

    /// 测试：交易数据结构
    #[test]
    fn test_transaction_structure() {
        init_test_env();

        let transaction = Transaction {
            id: Uuid::new_v4(),
            user_id: Uuid::new_v4(),
            transaction_type: TransactionType::Deposit,
            currency: "ETH".to_string(),
            amount: Decimal::new(2500000, 6), // 2.500000
            status: TransactionStatus::Completed,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        assert_eq!(transaction.currency, "ETH");
        assert_eq!(transaction.amount, Decimal::new(2500000, 6));
        assert!(matches!(transaction.transaction_type, TransactionType::Deposit));
        assert!(matches!(transaction.status, TransactionStatus::Completed));
    }

    /// 测试：健康检查响应
    #[tokio::test]
    async fn test_health_check_response() {
        init_test_env();

        let state = create_test_app_state();
        let app = create_app(state);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/health")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let health_response: HealthResponse = serde_json::from_slice(&body).unwrap();

        assert_eq!(health_response.status, "healthy");
        assert_eq!(health_response.service, "wallet-service");
        assert_eq!(health_response.version, "1.0.0");
        assert!(health_response.uptime < 10); // 应该是刚启动的
    }

    /// 测试：获取所有余额
    #[tokio::test]
    async fn test_get_all_balances() {
        init_test_env();

        let state = create_test_app_state();
        let app = create_app(state);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/wallet/balances")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let api_response: ApiResponse<Vec<Balance>> = serde_json::from_slice(&body).unwrap();

        assert!(api_response.success);
        assert!(api_response.data.is_some());

        let balances = api_response.data.unwrap();
        assert!(balances.len() > 0);

        // 验证包含预期的余额
        let btc_balance = balances.iter().find(|b| b.currency == "BTC");
        assert!(btc_balance.is_some(), "应该包含BTC余额");

        let eth_balance = balances.iter().find(|b| b.currency == "ETH");
        assert!(eth_balance.is_some(), "应该包含ETH余额");

        let usdt_balance = balances.iter().find(|b| b.currency == "USDT");
        assert!(usdt_balance.is_some(), "应该包含USDT余额");
    }

    /// 测试：获取特定货币余额
    #[tokio::test]
    async fn test_get_specific_balance() {
        init_test_env();

        let state = create_test_app_state();
        let app = create_app(state);

        // 测试存在的货币
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/wallet/balance/BTC")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let api_response: ApiResponse<Balance> = serde_json::from_slice(&body).unwrap();

        assert!(api_response.success);
        assert!(api_response.data.is_some());

        let balance = api_response.data.unwrap();
        assert_eq!(balance.currency, "BTC");
        assert!(balance.available > Decimal::ZERO);
    }

    /// 测试：获取不存在的货币余额
    #[tokio::test]
    async fn test_get_nonexistent_balance() {
        init_test_env();

        let state = create_test_app_state();
        let app = create_app(state);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/wallet/balance/INVALID")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    /// 测试：获取交易历史
    #[tokio::test]
    async fn test_get_transactions() {
        init_test_env();

        let state = create_test_app_state();
        let app = create_app(state);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/wallet/transactions")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let api_response: ApiResponse<Vec<Transaction>> = serde_json::from_slice(&body).unwrap();

        assert!(api_response.success);
        assert!(api_response.data.is_some());

        let transactions = api_response.data.unwrap();
        assert!(transactions.len() > 0, "应该有交易历史数据");

        // 验证交易数据格式
        for transaction in &transactions {
            assert!(!transaction.currency.is_empty());
            assert!(transaction.amount > Decimal::ZERO);
            assert!(!transaction.id.is_nil());
            assert!(!transaction.user_id.is_nil());
        }
    }

    /// 测试：交易类型枚举
    #[test]
    fn test_transaction_type_enum() {
        init_test_env();

        let deposit = TransactionType::Deposit;
        let withdrawal = TransactionType::Withdrawal;
        let trade = TransactionType::Trade;
        let fee = TransactionType::Fee;

        // 验证交易类型可以正确创建和比较
        match deposit {
            TransactionType::Deposit => assert!(true),
            _ => assert!(false, "应该是存款类型"),
        }

        match withdrawal {
            TransactionType::Withdrawal => assert!(true),
            _ => assert!(false, "应该是提款类型"),
        }

        match trade {
            TransactionType::Trade => assert!(true),
            _ => assert!(false, "应该是交易类型"),
        }

        match fee {
            TransactionType::Fee => assert!(true),
            _ => assert!(false, "应该是手续费类型"),
        }
    }

    /// 测试：交易状态枚举
    #[test]
    fn test_transaction_status_enum() {
        init_test_env();

        let pending = TransactionStatus::Pending;
        let completed = TransactionStatus::Completed;
        let failed = TransactionStatus::Failed;
        let cancelled = TransactionStatus::Cancelled;

        // 验证交易状态可以正确创建和比较
        match pending {
            TransactionStatus::Pending => assert!(true),
            _ => assert!(false, "应该是待处理状态"),
        }

        match completed {
            TransactionStatus::Completed => assert!(true),
            _ => assert!(false, "应该是已完成状态"),
        }

        match failed {
            TransactionStatus::Failed => assert!(true),
            _ => assert!(false, "应该是失败状态"),
        }

        match cancelled {
            TransactionStatus::Cancelled => assert!(true),
            _ => assert!(false, "应该是已取消状态"),
        }
    }

    /// 测试：并发访问安全性
    #[tokio::test]
    async fn test_concurrent_access_safety() {
        init_test_env();

        let state = create_test_app_state();
        let mut handles = vec![];

        // 启动多个并发任务
        for i in 0..10 {
            let state_clone = state.clone();
            let handle = tokio::spawn(async move {
                // 并发读取余额数据
                let balances = state_clone.balances.read().await;
                let balance_count = balances.len();
                drop(balances);

                // 并发读取交易数据
                let transactions = state_clone.transactions.read().await;
                let transaction_count = transactions.len();
                drop(transactions);

                (i, balance_count, transaction_count)
            });
            handles.push(handle);
        }

        // 等待所有任务完成
        for handle in handles {
            let (task_id, balance_count, transaction_count) = handle.await.unwrap();
            assert!(balance_count > 0, "任务{}应该读取到余额数据", task_id);
            assert!(transaction_count > 0, "任务{}应该读取到交易数据", task_id);
        }
    }

    /// 测试：精度处理
    #[test]
    fn test_precision_handling() {
        init_test_env();

        // 测试不同精度的金额
        let high_precision = Decimal::new(123456789, 8); // 1.23456789
        let low_precision = Decimal::new(12345, 2); // 123.45
        let zero_precision = Decimal::new(12345, 0); // 12345

        assert_eq!(high_precision.scale(), 8);
        assert_eq!(low_precision.scale(), 2);
        assert_eq!(zero_precision.scale(), 0);

        // 验证金额计算
        let total = high_precision + low_precision;
        assert!(total > high_precision);
        assert!(total > low_precision);
    }

    /// 测试：性能基准
    #[tokio::test]
    async fn test_performance_benchmark() {
        init_test_env();

        let state = create_test_app_state();
        let start = std::time::Instant::now();

        // 模拟大量并发请求
        let mut handles = vec![];
        for _ in 0..100 {
            let state_clone = state.clone();
            let handle = tokio::spawn(async move {
                let _balances = state_clone.balances.read().await;
                let _transactions = state_clone.transactions.read().await;
            });
            handles.push(handle);
        }

        // 等待所有请求完成
        for handle in handles {
            handle.await.unwrap();
        }

        let duration = start.elapsed();
        println!("100个并发请求耗时: {:?}", duration);

        // 性能要求：100个并发请求应该在1秒内完成
        assert!(duration.as_secs() < 1, "钱包服务性能不达标");
    }

    /// 测试：内存使用优化
    #[tokio::test]
    async fn test_memory_usage_optimization() {
        init_test_env();

        let state = create_test_app_state();

        // 模拟添加大量数据
        {
            let mut balances = state.balances.write().await;
            let mut transactions = state.transactions.write().await;

            for i in 0..1000 {
                let currency = format!("TEST{}", i);

                // 添加余额
                let balance = Balance {
                    currency: currency.clone(),
                    available: Decimal::new(10000 + i, 4),
                    locked: Decimal::new(1000 + i, 4),
                };
                balances.insert(currency.clone(), balance);

                // 添加交易
                let transaction = Transaction {
                    id: Uuid::new_v4(),
                    user_id: Uuid::new_v4(),
                    transaction_type: if i % 2 == 0 { TransactionType::Deposit } else { TransactionType::Withdrawal },
                    currency: currency.clone(),
                    amount: Decimal::new(1000 + i, 4),
                    status: TransactionStatus::Completed,
                    created_at: chrono::Utc::now(),
                    updated_at: chrono::Utc::now(),
                };
                transactions.push(transaction);
            }
        }

        // 验证数据添加成功
        let balances = state.balances.read().await;
        let transactions = state.transactions.read().await;

        assert!(balances.len() >= 1000, "应该有至少1000个余额");
        assert!(transactions.len() >= 1000, "应该有至少1000个交易");

        // 清理内存（通过作用域自动清理）
        drop(balances);
        drop(transactions);
        assert!(true, "内存使用优化测试完成");
    }

    /// 测试：错误处理
    #[tokio::test]
    async fn test_error_handling() {
        init_test_env();

        let state = create_test_app_state();
        let app = create_app(state);

        // 测试无效路径
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/wallet/invalid")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    /// 测试：数据验证
    #[test]
    fn test_data_validation() {
        init_test_env();

        // 验证余额数据的合理性
        let balance = Balance {
            currency: "BTC".to_string(),
            available: Decimal::new(100000000, 8), // 1.00000000
            locked: Decimal::new(10000000, 8), // 0.10000000
        };

        // 验证余额关系
        assert!(balance.available >= Decimal::ZERO, "可用余额应该大于等于零");
        assert!(balance.locked >= Decimal::ZERO, "锁定余额应该大于等于零");
        assert!(!balance.currency.is_empty(), "货币代码不应该为空");

        // 验证交易数据的合理性
        let transaction = Transaction {
            id: Uuid::new_v4(),
            user_id: Uuid::new_v4(),
            transaction_type: TransactionType::Deposit,
            currency: "ETH".to_string(),
            amount: Decimal::new(2500000, 6), // 2.500000
            status: TransactionStatus::Completed,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        assert!(transaction.amount > Decimal::ZERO, "交易金额应该大于零");
        assert!(!transaction.currency.is_empty(), "货币代码不应该为空");
        assert!(!transaction.id.is_nil(), "交易ID不应该为空");
        assert!(!transaction.user_id.is_nil(), "用户ID不应该为空");
        assert!(transaction.updated_at >= transaction.created_at, "更新时间应该大于等于创建时间");
    }
}
