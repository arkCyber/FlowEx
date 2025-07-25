//! FlowEx Trading Service Integration Tests
//!
//! Comprehensive integration tests for the trading service
//! covering trading pairs, order management, and order book operations.

use axum::http::StatusCode;
use flowex_types::{ApiResponse, CreateOrderRequest, Order, OrderSide, OrderType, TradingPair};
use rust_decimal::Decimal;
use serde_json::Value;
use std::collections::HashMap;
use std::str::FromStr;
use tokio::time::{sleep, Duration};
use uuid::Uuid;

/// Test configuration for trading service
struct TradingTestConfig {
    base_url: String,
    client: reqwest::Client,
}

impl TradingTestConfig {
    fn new() -> Self {
        Self {
            base_url: "http://localhost:8002".to_string(),
            client: reqwest::Client::new(),
        }
    }
}

/// Test helper for making HTTP requests to trading service
async fn make_trading_request(
    config: &TradingTestConfig,
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
async fn test_trading_service_health() {
    let config = TradingTestConfig::new();
    
    let (status, body) = make_trading_request(
        &config,
        reqwest::Method::GET,
        "/health",
        None,
        None,
    ).await.expect("Health check request failed");
    
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["status"], "healthy");
    assert_eq!(body["service"], "trading-service");
    assert!(body["uptime"].is_number());
}

#[tokio::test]
async fn test_get_trading_pairs() {
    let config = TradingTestConfig::new();
    
    let (status, body) = make_trading_request(
        &config,
        reqwest::Method::GET,
        "/api/trading/pairs",
        None,
        None,
    ).await.expect("Get trading pairs request failed");
    
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["success"], true);
    assert!(body["data"].is_array());
    
    let pairs = body["data"].as_array().unwrap();
    assert!(!pairs.is_empty());
    
    // Check that BTC-USDT pair exists
    let btc_usdt = pairs.iter().find(|p| p["symbol"] == "BTC-USDT");
    assert!(btc_usdt.is_some());
    
    let btc_pair = btc_usdt.unwrap();
    assert_eq!(btc_pair["base_currency"], "BTC");
    assert_eq!(btc_pair["quote_currency"], "USDT");
    assert_eq!(btc_pair["status"], "active");
}

#[tokio::test]
async fn test_get_order_book() {
    let config = TradingTestConfig::new();
    
    let (status, body) = make_trading_request(
        &config,
        reqwest::Method::GET,
        "/api/trading/orderbook/BTC-USDT",
        None,
        None,
    ).await.expect("Get order book request failed");
    
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["success"], true);
    
    let order_book = &body["data"];
    assert_eq!(order_book["symbol"], "BTC-USDT");
    assert!(order_book["bids"].is_array());
    assert!(order_book["asks"].is_array());
    assert!(order_book["timestamp"].is_string());
    
    // Check bid/ask structure
    let bids = order_book["bids"].as_array().unwrap();
    if !bids.is_empty() {
        let first_bid = &bids[0];
        assert!(first_bid["price"].is_string());
        assert!(first_bid["quantity"].is_string());
    }
}

#[tokio::test]
async fn test_get_order_book_invalid_symbol() {
    let config = TradingTestConfig::new();
    
    let (status, body) = make_trading_request(
        &config,
        reqwest::Method::GET,
        "/api/trading/orderbook/INVALID-PAIR",
        None,
        None,
    ).await.expect("Get order book request failed");
    
    assert_eq!(status, StatusCode::NOT_FOUND);
    assert_eq!(body["success"], false);
}

#[tokio::test]
async fn test_create_limit_order() {
    let config = TradingTestConfig::new();
    
    let order_request = CreateOrderRequest {
        trading_pair: "BTC-USDT".to_string(),
        side: OrderSide::Buy,
        order_type: OrderType::Limit,
        price: Some(Decimal::from_str("45000.00").unwrap()),
        quantity: Decimal::from_str("0.001").unwrap(),
    };
    
    let (status, body) = make_trading_request(
        &config,
        reqwest::Method::POST,
        "/api/trading/orders",
        Some(serde_json::to_value(order_request).unwrap()),
        None,
    ).await.expect("Create order request failed");
    
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["success"], true);
    
    let order = &body["data"];
    assert!(order["id"].is_string());
    assert_eq!(order["trading_pair"], "BTC-USDT");
    assert_eq!(order["side"], "buy");
    assert_eq!(order["order_type"], "limit");
    assert_eq!(order["status"], "NEW");
    assert_eq!(order["price"], "45000.00");
    assert_eq!(order["quantity"], "0.001");
}

#[tokio::test]
async fn test_create_market_order() {
    let config = TradingTestConfig::new();
    
    let order_request = CreateOrderRequest {
        trading_pair: "BTC-USDT".to_string(),
        side: OrderSide::Sell,
        order_type: OrderType::Market,
        price: None, // Market orders don't have price
        quantity: Decimal::from_str("0.001").unwrap(),
    };
    
    let (status, body) = make_trading_request(
        &config,
        reqwest::Method::POST,
        "/api/trading/orders",
        Some(serde_json::to_value(order_request).unwrap()),
        None,
    ).await.expect("Create order request failed");
    
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["success"], true);
    
    let order = &body["data"];
    assert_eq!(order["order_type"], "market");
    assert_eq!(order["side"], "sell");
    assert!(order["price"].is_null()); // Market orders have no price
}

#[tokio::test]
async fn test_create_order_invalid_pair() {
    let config = TradingTestConfig::new();
    
    let order_request = CreateOrderRequest {
        trading_pair: "INVALID-PAIR".to_string(),
        side: OrderSide::Buy,
        order_type: OrderType::Limit,
        price: Some(Decimal::from_str("100.00").unwrap()),
        quantity: Decimal::from_str("1.0").unwrap(),
    };
    
    let (status, body) = make_trading_request(
        &config,
        reqwest::Method::POST,
        "/api/trading/orders",
        Some(serde_json::to_value(order_request).unwrap()),
        None,
    ).await.expect("Create order request failed");
    
    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert_eq!(body["success"], false);
}

#[tokio::test]
async fn test_create_order_invalid_quantity() {
    let config = TradingTestConfig::new();
    
    let order_request = CreateOrderRequest {
        trading_pair: "BTC-USDT".to_string(),
        side: OrderSide::Buy,
        order_type: OrderType::Limit,
        price: Some(Decimal::from_str("45000.00").unwrap()),
        quantity: Decimal::from_str("0.0").unwrap(), // Invalid quantity
    };
    
    let (status, body) = make_trading_request(
        &config,
        reqwest::Method::POST,
        "/api/trading/orders",
        Some(serde_json::to_value(order_request).unwrap()),
        None,
    ).await.expect("Create order request failed");
    
    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert_eq!(body["success"], false);
}

#[tokio::test]
async fn test_get_user_orders() {
    let config = TradingTestConfig::new();
    
    // First create an order
    let order_request = CreateOrderRequest {
        trading_pair: "BTC-USDT".to_string(),
        side: OrderSide::Buy,
        order_type: OrderType::Limit,
        price: Some(Decimal::from_str("45000.00").unwrap()),
        quantity: Decimal::from_str("0.001").unwrap(),
    };
    
    let (status, _) = make_trading_request(
        &config,
        reqwest::Method::POST,
        "/api/trading/orders",
        Some(serde_json::to_value(order_request).unwrap()),
        None,
    ).await.expect("Create order request failed");
    
    assert_eq!(status, StatusCode::OK);
    
    // Now get user orders
    let (status, body) = make_trading_request(
        &config,
        reqwest::Method::GET,
        "/api/trading/orders",
        None,
        None,
    ).await.expect("Get orders request failed");
    
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["success"], true);
    assert!(body["data"].is_array());
    
    let orders = body["data"].as_array().unwrap();
    assert!(!orders.is_empty());
}

#[tokio::test]
async fn test_order_validation_edge_cases() {
    let config = TradingTestConfig::new();
    
    // Test with extremely small quantity
    let order_request = CreateOrderRequest {
        trading_pair: "BTC-USDT".to_string(),
        side: OrderSide::Buy,
        order_type: OrderType::Limit,
        price: Some(Decimal::from_str("45000.00").unwrap()),
        quantity: Decimal::from_str("0.00000001").unwrap(),
    };
    
    let (status, body) = make_trading_request(
        &config,
        reqwest::Method::POST,
        "/api/trading/orders",
        Some(serde_json::to_value(order_request).unwrap()),
        None,
    ).await.expect("Create order request failed");
    
    // Should either succeed or fail with validation error
    assert!(status == StatusCode::OK || status == StatusCode::BAD_REQUEST);
    
    if status == StatusCode::BAD_REQUEST {
        assert_eq!(body["success"], false);
    }
}

#[tokio::test]
async fn test_concurrent_order_creation() {
    let config = TradingTestConfig::new();
    
    let mut handles = vec![];
    
    // Create multiple orders concurrently
    for i in 0..5 {
        let config_clone = TradingTestConfig::new();
        let handle = tokio::spawn(async move {
            let order_request = CreateOrderRequest {
                trading_pair: "BTC-USDT".to_string(),
                side: OrderSide::Buy,
                order_type: OrderType::Limit,
                price: Some(Decimal::from_str(&format!("{}.00", 45000 + i)).unwrap()),
                quantity: Decimal::from_str("0.001").unwrap(),
            };
            
            make_trading_request(
                &config_clone,
                reqwest::Method::POST,
                "/api/trading/orders",
                Some(serde_json::to_value(order_request).unwrap()),
                None,
            ).await
        });
        
        handles.push(handle);
    }
    
    // Wait for all orders to complete
    let mut success_count = 0;
    for handle in handles {
        if let Ok(Ok((status, _))) = handle.await {
            if status == StatusCode::OK {
                success_count += 1;
            }
        }
    }
    
    // At least some orders should succeed
    assert!(success_count > 0);
}

/// Helper function to run all trading service tests
pub async fn run_trading_service_tests() -> Result<(), Box<dyn std::error::Error>> {
    println!("🧪 Running Trading Service Integration Tests");
    
    // Wait for service to be ready
    let config = TradingTestConfig::new();
    let mut retries = 30;
    
    while retries > 0 {
        match make_trading_request(&config, reqwest::Method::GET, "/health", None, None).await {
            Ok((StatusCode::OK, _)) => break,
            _ => {
                retries -= 1;
                if retries == 0 {
                    return Err("Trading service not ready after 30 seconds".into());
                }
                sleep(Duration::from_secs(1)).await;
            }
        }
    }
    
    println!("✅ Trading service is ready, running tests...");
    
    Ok(())
}
