//! FlowEx Market Data Service
//!
//! Enterprise-grade market data service providing real-time price feeds,
//! historical data, and market statistics.

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::get,
    Router,
};
use flowex_types::{
    ApiResponse, HealthResponse, Ticker, Trade, OrderSide,
};
use rust_decimal::Decimal;
use std::{collections::HashMap, sync::Arc, time::SystemTime};
use tokio::sync::RwLock;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tracing::{info};
use uuid::Uuid;

/// Application state for the market data service
#[derive(Clone)]
pub struct AppState {
    pub tickers: Arc<RwLock<HashMap<String, Ticker>>>,
    pub trades: Arc<RwLock<HashMap<String, Vec<Trade>>>>,
    pub start_time: SystemTime,
}

impl AppState {
    pub fn new() -> Self {
        let mut tickers = HashMap::new();
        let mut trades = HashMap::new();

        // Initialize demo tickers
        let btc_ticker = Ticker {
            symbol: "BTC-USDT".to_string(),
            price: Decimal::new(4500000, 2), // 45000.00
            change: Decimal::new(250, 2), // 2.50
            change_percent: Decimal::new(556, 4), // 5.56%
            high: Decimal::new(4650000, 2), // 46500.00
            low: Decimal::new(4350000, 2), // 43500.00
            volume: Decimal::new(123456789, 5), // 1234.56789
            timestamp: chrono::Utc::now(),
        };

        let eth_ticker = Ticker {
            symbol: "ETH-USDT".to_string(),
            price: Decimal::new(300000, 2), // 3000.00
            change: Decimal::new(150, 2), // 1.50
            change_percent: Decimal::new(526, 4), // 5.26%
            high: Decimal::new(310000, 2), // 3100.00
            low: Decimal::new(290000, 2), // 2900.00
            volume: Decimal::new(987654321, 5), // 9876.54321
            timestamp: chrono::Utc::now(),
        };

        // Initialize demo trades
        let btc_trades = vec![
            Trade {
                id: Uuid::new_v4(),
                symbol: "BTC-USDT".to_string(),
                price: Decimal::new(4500000, 2),
                quantity: Decimal::new(12345, 5),
                side: OrderSide::Buy,
                timestamp: chrono::Utc::now(),
            },
            Trade {
                id: Uuid::new_v4(),
                symbol: "BTC-USDT".to_string(),
                price: Decimal::new(4499999, 2),
                quantity: Decimal::new(23456, 5),
                side: OrderSide::Sell,
                timestamp: chrono::Utc::now(),
            },
        ];

        tickers.insert("BTC-USDT".to_string(), btc_ticker);
        tickers.insert("ETH-USDT".to_string(), eth_ticker);
        trades.insert("BTC-USDT".to_string(), btc_trades);

        Self {
            tickers: Arc::new(RwLock::new(tickers)),
            trades: Arc::new(RwLock::new(trades)),
            start_time: SystemTime::now(),
        }
    }
}

/// Health check endpoint
async fn health_check(State(state): State<AppState>) -> Json<HealthResponse> {
    let uptime = state.start_time.elapsed().unwrap_or_default().as_secs();

    Json(HealthResponse {
        status: "healthy".to_string(),
        service: "market-data-service".to_string(),
        version: "1.0.0".to_string(),
        timestamp: chrono::Utc::now(),
        uptime,
    })
}

/// Get all market tickers
async fn get_tickers(State(state): State<AppState>) -> Json<ApiResponse<Vec<Ticker>>> {
    let tickers = state.tickers.read().await;
    let tickers_vec: Vec<Ticker> = tickers.values().cloned().collect();
    Json(ApiResponse::success(tickers_vec))
}

/// Get ticker for a specific symbol
async fn get_ticker(
    State(state): State<AppState>,
    Path(symbol): Path<String>,
) -> Result<Json<ApiResponse<Ticker>>, StatusCode> {
    let tickers = state.tickers.read().await;
    
    if let Some(ticker) = tickers.get(&symbol) {
        Ok(Json(ApiResponse::success(ticker.clone())))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

/// Get recent trades for a symbol
async fn get_trades(
    State(state): State<AppState>,
    Path(symbol): Path<String>,
) -> Result<Json<ApiResponse<Vec<Trade>>>, StatusCode> {
    let trades = state.trades.read().await;
    
    if let Some(symbol_trades) = trades.get(&symbol) {
        Ok(Json(ApiResponse::success(symbol_trades.clone())))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

/// Create the application router
fn create_app(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/api/market-data/tickers", get(get_tickers))
        .route("/api/market-data/ticker/:symbol", get(get_ticker))
        .route("/api/market-data/trades/:symbol", get(get_trades))
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

    info!("Starting FlowEx Market Data Service");

    let state = AppState::new();
    let app = create_app(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8003").await?;
    info!("Market data service listening on http://0.0.0.0:8003");

    axum::serve(listener, app).await?;

    Ok(())
}
