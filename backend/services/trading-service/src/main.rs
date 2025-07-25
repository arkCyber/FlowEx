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
