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
