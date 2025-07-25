//! FlowEx Trading Service
//!
//! Enterprise-grade trading service providing order management, order book operations,
//! and trade execution for the FlowEx cryptocurrency exchange platform.

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use flowex_types::{
    ApiResponse, CreateOrderRequest, FlowExError, FlowExResult, HealthResponse, Order,
    OrderBook, OrderBookLevel, OrderSide, OrderStatus, OrderType, TradingPair, TradingStatus,
};
use rust_decimal::Decimal;
use std::{collections::HashMap, str::FromStr, sync::Arc, time::SystemTime};
use tokio::sync::RwLock;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tracing::{info, warn};
use uuid::Uuid;

/// Application state for the trading service
#[derive(Clone)]
pub struct AppState {
    pub trading_pairs: Arc<RwLock<HashMap<String, TradingPair>>>,
    pub orders: Arc<RwLock<HashMap<Uuid, Order>>>,
    pub order_books: Arc<RwLock<HashMap<String, OrderBook>>>,
    pub start_time: SystemTime,
}

impl AppState {
    pub fn new() -> Self {
        let mut trading_pairs = HashMap::new();
        let mut order_books = HashMap::new();

        // Initialize demo trading pairs
        let btc_usdt = TradingPair {
            symbol: "BTC-USDT".to_string(),
            base_asset: "BTC".to_string(),
            quote_asset: "USDT".to_string(),
            status: TradingStatus::Trading,
            min_price: Decimal::new(1, 2), // 0.01
            max_price: Decimal::new(10000000, 0), // 10M
            min_qty: Decimal::new(1, 8), // 0.00000001
            max_qty: Decimal::new(1000000, 0), // 1M
            step_size: Decimal::new(1, 8),
            tick_size: Decimal::new(1, 2),
        };

        let eth_usdt = TradingPair {
            symbol: "ETH-USDT".to_string(),
            base_asset: "ETH".to_string(),
            quote_asset: "USDT".to_string(),
            status: TradingStatus::Trading,
            min_price: Decimal::new(1, 2),
            max_price: Decimal::new(1000000, 0),
            min_qty: Decimal::new(1, 8),
            max_qty: Decimal::new(1000000, 0),
            step_size: Decimal::new(1, 8),
            tick_size: Decimal::new(1, 2),
        };

        // Initialize order books
        let btc_order_book = OrderBook {
            symbol: "BTC-USDT".to_string(),
            bids: vec![
                OrderBookLevel {
                    price: Decimal::new(4499999, 2), // 44999.99
                    quantity: Decimal::new(12345, 5), // 0.12345
                },
            ],
            asks: vec![
                OrderBookLevel {
                    price: Decimal::new(4500001, 2), // 45000.01
                    quantity: Decimal::new(11111, 5),
                },
            ],
            timestamp: chrono::Utc::now(),
        };

        trading_pairs.insert("BTC-USDT".to_string(), btc_usdt);
        trading_pairs.insert("ETH-USDT".to_string(), eth_usdt);
        order_books.insert("BTC-USDT".to_string(), btc_order_book);

        Self {
            trading_pairs: Arc::new(RwLock::new(trading_pairs)),
            orders: Arc::new(RwLock::new(HashMap::new())),
            order_books: Arc::new(RwLock::new(order_books)),
            start_time: SystemTime::now(),
        }
    }
}

/// Health check endpoint
async fn health_check(State(state): State<AppState>) -> Json<HealthResponse> {
    let uptime = state.start_time.elapsed().unwrap_or_default().as_secs();

    Json(HealthResponse {
        status: "healthy".to_string(),
        service: "trading-service".to_string(),
        version: "1.0.0".to_string(),
        timestamp: chrono::Utc::now(),
        uptime,
    })
}

/// Get all trading pairs
async fn get_trading_pairs(State(state): State<AppState>) -> Json<ApiResponse<Vec<TradingPair>>> {
    let pairs = state.trading_pairs.read().await;
    let pairs_vec: Vec<TradingPair> = pairs.values().cloned().collect();
    Json(ApiResponse::success(pairs_vec))
}

/// Get order book for a specific trading pair
async fn get_order_book(
    State(state): State<AppState>,
    Path(symbol): Path<String>,
) -> Result<Json<ApiResponse<OrderBook>>, StatusCode> {
    let order_books = state.order_books.read().await;
    
    if let Some(order_book) = order_books.get(&symbol) {
        Ok(Json(ApiResponse::success((*order_book).clone())))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

/// Create a new order
async fn create_order(
    State(state): State<AppState>,
    Json(request): Json<CreateOrderRequest>,
) -> Result<Json<ApiResponse<Order>>, StatusCode> {
    info!("Creating order for trading pair: {}", request.trading_pair);

    // Create new order
    let order = Order {
        id: Uuid::new_v4(),
        user_id: Uuid::new_v4(), // In real implementation, extract from JWT
        trading_pair: request.trading_pair,
        side: request.side,
        order_type: request.order_type,
        price: request.price,
        quantity: request.quantity,
        filled_quantity: Decimal::ZERO,
        status: OrderStatus::New,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };

    // Store order
    let mut orders = state.orders.write().await;
    orders.insert(order.id, order.clone());

    info!("Order created successfully: {}", order.id);
    Ok(Json(ApiResponse::success(order)))
}

/// Get user orders
async fn get_orders(State(state): State<AppState>) -> Json<ApiResponse<Vec<Order>>> {
    let orders = state.orders.read().await;
    let orders_vec: Vec<Order> = orders.values().cloned().collect();
    Json(ApiResponse::success(orders_vec))
}

/// Create the application router
fn create_app(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/api/trading/pairs", get(get_trading_pairs))
        .route("/api/trading/orderbook/:symbol", get(get_order_book))
        .route("/api/trading/orders", post(create_order))
        .route("/api/trading/orders", get(get_orders))
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

    info!("Starting FlowEx Trading Service");

    let state = AppState::new();
    let app = create_app(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8002").await?;
    info!("Trading service listening on http://0.0.0.0:8002");

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
        let mut trading_pairs = HashMap::new();
        let mut orders = HashMap::new();

        // 添加测试交易对
        trading_pairs.insert("BTCUSDT".to_string(), TradingPair {
            symbol: "BTCUSDT".to_string(),
            base_asset: "BTC".to_string(),
            quote_asset: "USDT".to_string(),
            status: "TRADING".to_string(),
            min_price: Decimal::new(1, 8), // 0.00000001
            max_price: Decimal::new(99999999999999999, 8), // 999999999.99999999
            min_qty: Decimal::new(1, 8), // 0.00000001
            max_qty: Decimal::new(99999999999999999, 8), // 999999999.99999999
            step_size: Decimal::new(1, 8), // 0.00000001
            tick_size: Decimal::new(1, 8), // 0.00000001
        });

        trading_pairs.insert("ETHUSDT".to_string(), TradingPair {
            symbol: "ETHUSDT".to_string(),
            base_asset: "ETH".to_string(),
            quote_asset: "USDT".to_string(),
            status: "TRADING".to_string(),
            min_price: Decimal::new(1, 8),
            max_price: Decimal::new(99999999999999999, 8),
            min_qty: Decimal::new(1, 8),
            max_qty: Decimal::new(99999999999999999, 8),
            step_size: Decimal::new(1, 8),
            tick_size: Decimal::new(1, 8),
        });

        // 添加测试订单
        let test_order = Order {
            id: Uuid::new_v4(),
            user_id: Uuid::new_v4(),
            trading_pair: "BTCUSDT".to_string(),
            side: OrderSide::Buy,
            order_type: OrderType::Limit,
            price: Some(Decimal::new(4500000, 2)), // 45000.00
            quantity: Decimal::new(100, 3), // 0.100
            filled_quantity: Decimal::ZERO,
            remaining_quantity: Decimal::new(100, 3), // 0.100
            status: OrderStatus::New,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };
        orders.insert(test_order.id, test_order);

        AppState {
            trading_pairs: Arc::new(RwLock::new(trading_pairs)),
            orders: Arc::new(RwLock::new(orders)),
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
            let trading_pairs = state.trading_pairs.read().await;
            assert!(trading_pairs.len() > 0, "应该有初始交易对数据");
            assert!(trading_pairs.contains_key("BTCUSDT"), "应该包含BTCUSDT交易对");
            assert!(trading_pairs.contains_key("ETHUSDT"), "应该包含ETHUSDT交易对");

            let orders = state.orders.read().await;
            assert!(orders.len() > 0, "应该有初始订单数据");
        });
    }

    /// 测试：交易对数据结构
    #[test]
    fn test_trading_pair_structure() {
        init_test_env();

        let trading_pair = TradingPair {
            symbol: "BTCUSDT".to_string(),
            base_asset: "BTC".to_string(),
            quote_asset: "USDT".to_string(),
            status: "TRADING".to_string(),
            min_price: Decimal::new(1, 8),
            max_price: Decimal::new(99999999999999999, 8),
            min_qty: Decimal::new(1, 8),
            max_qty: Decimal::new(99999999999999999, 8),
            step_size: Decimal::new(1, 8),
            tick_size: Decimal::new(1, 8),
        };

        assert_eq!(trading_pair.symbol, "BTCUSDT");
        assert_eq!(trading_pair.base_asset, "BTC");
        assert_eq!(trading_pair.quote_asset, "USDT");
        assert_eq!(trading_pair.status, "TRADING");
        assert!(trading_pair.min_price > Decimal::ZERO);
        assert!(trading_pair.max_price > trading_pair.min_price);
        assert!(trading_pair.min_qty > Decimal::ZERO);
        assert!(trading_pair.max_qty > trading_pair.min_qty);
    }

    /// 测试：订单数据结构
    #[test]
    fn test_order_structure() {
        init_test_env();

        let order = Order {
            id: Uuid::new_v4(),
            user_id: Uuid::new_v4(),
            trading_pair: "ETHUSDT".to_string(),
            side: OrderSide::Sell,
            order_type: OrderType::Market,
            price: None, // 市价单没有价格
            quantity: Decimal::new(250, 2), // 2.50
            filled_quantity: Decimal::ZERO,
            remaining_quantity: Decimal::new(250, 2), // 2.50
            status: OrderStatus::New,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        assert_eq!(order.trading_pair, "ETHUSDT");
        assert!(matches!(order.side, OrderSide::Sell));
        assert!(matches!(order.order_type, OrderType::Market));
        assert_eq!(order.price, None);
        assert_eq!(order.quantity, Decimal::new(250, 2));
        assert!(matches!(order.status, OrderStatus::New));
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
        assert_eq!(health_response.service, "trading-service");
        assert_eq!(health_response.version, "1.0.0");
        assert!(health_response.uptime < 10); // 应该是刚启动的
    }

    /// 测试：获取所有交易对
    #[tokio::test]
    async fn test_get_all_trading_pairs() {
        init_test_env();

        let state = create_test_app_state();
        let app = create_app(state);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/trading/pairs")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let api_response: ApiResponse<Vec<TradingPair>> = serde_json::from_slice(&body).unwrap();

        assert!(api_response.success);
        assert!(api_response.data.is_some());

        let trading_pairs = api_response.data.unwrap();
        assert!(trading_pairs.len() > 0);

        // 验证包含预期的交易对
        let btc_pair = trading_pairs.iter().find(|p| p.symbol == "BTCUSDT");
        assert!(btc_pair.is_some(), "应该包含BTCUSDT交易对");

        let eth_pair = trading_pairs.iter().find(|p| p.symbol == "ETHUSDT");
        assert!(eth_pair.is_some(), "应该包含ETHUSDT交易对");
    }

    /// 测试：获取所有订单
    #[tokio::test]
    async fn test_get_all_orders() {
        init_test_env();

        let state = create_test_app_state();
        let app = create_app(state);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/trading/orders")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let api_response: ApiResponse<Vec<Order>> = serde_json::from_slice(&body).unwrap();

        assert!(api_response.success);
        assert!(api_response.data.is_some());

        let orders = api_response.data.unwrap();
        assert!(orders.len() > 0, "应该有订单数据");

        // 验证订单数据格式
        for order in &orders {
            assert!(!order.trading_pair.is_empty());
            assert!(order.quantity > Decimal::ZERO);
            assert!(!order.id.is_nil());
            assert!(!order.user_id.is_nil());
        }
    }

    /// 测试：创建限价买单
    #[tokio::test]
    async fn test_create_limit_buy_order() {
        init_test_env();

        let state = create_test_app_state();
        let app = create_app(state);

        let order_request = CreateOrderRequest {
            trading_pair: "BTCUSDT".to_string(),
            side: OrderSide::Buy,
            order_type: OrderType::Limit,
            price: Some(Decimal::new(4400000, 2)), // 44000.00
            quantity: Decimal::new(50, 3), // 0.050
        };

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/trading/orders")
                    .header("content-type", "application/json")
                    .body(Body::from(serde_json::to_string(&order_request).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let api_response: ApiResponse<Order> = serde_json::from_slice(&body).unwrap();

        assert!(api_response.success);
        assert!(api_response.data.is_some());

        let order = api_response.data.unwrap();
        assert_eq!(order.trading_pair, "BTCUSDT");
        assert!(matches!(order.side, OrderSide::Buy));
        assert!(matches!(order.order_type, OrderType::Limit));
        assert_eq!(order.price, Some(Decimal::new(4400000, 2)));
        assert_eq!(order.quantity, Decimal::new(50, 3));
        assert!(matches!(order.status, OrderStatus::New));
    }

    /// 测试：创建市价卖单
    #[tokio::test]
    async fn test_create_market_sell_order() {
        init_test_env();

        let state = create_test_app_state();
        let app = create_app(state);

        let order_request = CreateOrderRequest {
            trading_pair: "ETHUSDT".to_string(),
            side: OrderSide::Sell,
            order_type: OrderType::Market,
            price: None, // 市价单没有价格
            quantity: Decimal::new(100, 2), // 1.00
        };

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/trading/orders")
                    .header("content-type", "application/json")
                    .body(Body::from(serde_json::to_string(&order_request).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let api_response: ApiResponse<Order> = serde_json::from_slice(&body).unwrap();

        assert!(api_response.success);
        assert!(api_response.data.is_some());

        let order = api_response.data.unwrap();
        assert_eq!(order.trading_pair, "ETHUSDT");
        assert!(matches!(order.side, OrderSide::Sell));
        assert!(matches!(order.order_type, OrderType::Market));
        assert_eq!(order.price, None);
        assert_eq!(order.quantity, Decimal::new(100, 2));
    }

    /// 测试：创建无效交易对订单
    #[tokio::test]
    async fn test_create_invalid_trading_pair_order() {
        init_test_env();

        let state = create_test_app_state();
        let app = create_app(state);

        let order_request = CreateOrderRequest {
            trading_pair: "INVALIDUSDT".to_string(), // 不存在的交易对
            side: OrderSide::Buy,
            order_type: OrderType::Limit,
            price: Some(Decimal::new(100, 0)),
            quantity: Decimal::new(1, 0),
        };

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/trading/orders")
                    .header("content-type", "application/json")
                    .body(Body::from(serde_json::to_string(&order_request).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    /// 测试：订单边界值验证
    #[tokio::test]
    async fn test_order_boundary_validation() {
        init_test_env();

        let state = create_test_app_state();
        let app = create_app(state);

        // 测试零数量订单
        let zero_quantity_request = CreateOrderRequest {
            trading_pair: "BTCUSDT".to_string(),
            side: OrderSide::Buy,
            order_type: OrderType::Limit,
            price: Some(Decimal::new(45000, 0)),
            quantity: Decimal::ZERO, // 零数量
        };

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/trading/orders")
                    .header("content-type", "application/json")
                    .body(Body::from(serde_json::to_string(&zero_quantity_request).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    /// 测试：订单类型枚举
    #[test]
    fn test_order_type_enum() {
        init_test_env();

        let market = OrderType::Market;
        let limit = OrderType::Limit;
        let stop_loss = OrderType::StopLoss;
        let take_profit = OrderType::TakeProfit;

        // 验证订单类型可以正确创建和比较
        match market {
            OrderType::Market => assert!(true),
            _ => assert!(false, "应该是市价单类型"),
        }

        match limit {
            OrderType::Limit => assert!(true),
            _ => assert!(false, "应该是限价单类型"),
        }

        match stop_loss {
            OrderType::StopLoss => assert!(true),
            _ => assert!(false, "应该是止损单类型"),
        }

        match take_profit {
            OrderType::TakeProfit => assert!(true),
            _ => assert!(false, "应该是止盈单类型"),
        }
    }

    /// 测试：订单状态枚举
    #[test]
    fn test_order_status_enum() {
        init_test_env();

        let new = OrderStatus::New;
        let partially_filled = OrderStatus::PartiallyFilled;
        let filled = OrderStatus::Filled;
        let cancelled = OrderStatus::Cancelled;
        let rejected = OrderStatus::Rejected;
        let expired = OrderStatus::Expired;

        // 验证订单状态可以正确创建和比较
        match new {
            OrderStatus::New => assert!(true),
            _ => assert!(false, "应该是新订单状态"),
        }

        match partially_filled {
            OrderStatus::PartiallyFilled => assert!(true),
            _ => assert!(false, "应该是部分成交状态"),
        }

        match filled {
            OrderStatus::Filled => assert!(true),
            _ => assert!(false, "应该是完全成交状态"),
        }

        match cancelled {
            OrderStatus::Cancelled => assert!(true),
            _ => assert!(false, "应该是已取消状态"),
        }

        match rejected {
            OrderStatus::Rejected => assert!(true),
            _ => assert!(false, "应该是已拒绝状态"),
        }

        match expired {
            OrderStatus::Expired => assert!(true),
            _ => assert!(false, "应该是已过期状态"),
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
                // 并发读取交易对数据
                let trading_pairs = state_clone.trading_pairs.read().await;
                let pair_count = trading_pairs.len();
                drop(trading_pairs);

                // 并发读取订单数据
                let orders = state_clone.orders.read().await;
                let order_count = orders.len();
                drop(orders);

                (i, pair_count, order_count)
            });
            handles.push(handle);
        }

        // 等待所有任务完成
        for handle in handles {
            let (task_id, pair_count, order_count) = handle.await.unwrap();
            assert!(pair_count > 0, "任务{}应该读取到交易对数据", task_id);
            assert!(order_count > 0, "任务{}应该读取到订单数据", task_id);
        }
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
                let _trading_pairs = state_clone.trading_pairs.read().await;
                let _orders = state_clone.orders.read().await;
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
        assert!(duration.as_secs() < 1, "交易服务性能不达标");
    }

    /// 测试：数据验证
    #[test]
    fn test_data_validation() {
        init_test_env();

        // 验证交易对数据的合理性
        let trading_pair = TradingPair {
            symbol: "BTCUSDT".to_string(),
            base_asset: "BTC".to_string(),
            quote_asset: "USDT".to_string(),
            status: "TRADING".to_string(),
            min_price: Decimal::new(1, 8),
            max_price: Decimal::new(99999999999999999, 8),
            min_qty: Decimal::new(1, 8),
            max_qty: Decimal::new(99999999999999999, 8),
            step_size: Decimal::new(1, 8),
            tick_size: Decimal::new(1, 8),
        };

        // 验证交易对关系
        assert!(trading_pair.max_price > trading_pair.min_price, "最大价格应该大于最小价格");
        assert!(trading_pair.max_qty > trading_pair.min_qty, "最大数量应该大于最小数量");
        assert!(trading_pair.min_price > Decimal::ZERO, "最小价格应该大于零");
        assert!(trading_pair.min_qty > Decimal::ZERO, "最小数量应该大于零");
        assert!(trading_pair.step_size > Decimal::ZERO, "步长应该大于零");
        assert!(trading_pair.tick_size > Decimal::ZERO, "价格精度应该大于零");

        // 验证订单数据的合理性
        let order = Order {
            id: Uuid::new_v4(),
            user_id: Uuid::new_v4(),
            trading_pair: "ETHUSDT".to_string(),
            side: OrderSide::Buy,
            order_type: OrderType::Limit,
            price: Some(Decimal::new(300000, 2)), // 3000.00
            quantity: Decimal::new(100, 2), // 1.00
            filled_quantity: Decimal::ZERO,
            remaining_quantity: Decimal::new(100, 2), // 1.00
            status: OrderStatus::New,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        assert!(order.quantity > Decimal::ZERO, "订单数量应该大于零");
        assert!(order.remaining_quantity <= order.quantity, "剩余数量应该小于等于总数量");
        assert!(order.filled_quantity <= order.quantity, "已成交数量应该小于等于总数量");
        assert_eq!(order.filled_quantity + order.remaining_quantity, order.quantity, "已成交+剩余应该等于总数量");
        assert!(!order.trading_pair.is_empty(), "交易对不应该为空");
        assert!(!order.id.is_nil(), "订单ID不应该为空");
        assert!(!order.user_id.is_nil(), "用户ID不应该为空");
    }
}
