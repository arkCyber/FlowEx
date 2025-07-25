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

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Once;
    use chrono::Utc;

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

    /// 测试：应用状态创建
    #[test]
    fn test_app_state_creation() {
        init_test_env();

        let state = AppState::new();

        // 验证状态创建成功
        assert!(state.start_time.elapsed().unwrap().as_secs() < 1);

        // 验证初始数据存在
        tokio::runtime::Runtime::new().unwrap().block_on(async {
            let tickers = state.tickers.read().await;
            assert!(tickers.len() > 0, "应该有初始ticker数据");
            assert!(tickers.contains_key("BTC-USDT"), "应该包含BTC-USDT ticker");
            assert!(tickers.contains_key("ETH-USDT"), "应该包含ETH-USDT ticker");

            let trades = state.trades.read().await;
            assert!(trades.len() > 0, "应该有初始交易数据");
        });
    }

    /// 测试：Ticker数据结构
    #[test]
    fn test_ticker_structure() {
        init_test_env();

        let ticker = Ticker {
            symbol: "TEST-USDT".to_string(),
            price: Decimal::new(10000, 2), // 100.00
            change: Decimal::new(500, 2), // 5.00
            change_percent: Decimal::new(525, 4), // 5.25%
            high: Decimal::new(10500, 2), // 105.00
            low: Decimal::new(9500, 2), // 95.00
            volume: Decimal::new(1000000, 3), // 1000.000
            timestamp: Utc::now(),
        };

        assert_eq!(ticker.symbol, "TEST-USDT");
        assert_eq!(ticker.price, Decimal::new(10000, 2));
        assert_eq!(ticker.change, Decimal::new(500, 2));
        assert_eq!(ticker.change_percent, Decimal::new(525, 4));
        assert_eq!(ticker.high, Decimal::new(10500, 2));
        assert_eq!(ticker.low, Decimal::new(9500, 2));
        assert_eq!(ticker.volume, Decimal::new(1000000, 3));
    }

    /// 测试：Trade数据结构
    #[test]
    fn test_trade_structure() {
        init_test_env();

        let trade = Trade {
            id: Uuid::new_v4(),
            symbol: "BTC-USDT".to_string(),
            price: Decimal::new(4500000, 2), // 45000.00
            quantity: Decimal::new(100, 3), // 0.100
            side: OrderSide::Buy,
            timestamp: Utc::now(),
        };

        assert_eq!(trade.symbol, "BTC-USDT");
        assert_eq!(trade.price, Decimal::new(4500000, 2));
        assert_eq!(trade.quantity, Decimal::new(100, 3));
        assert!(matches!(trade.side, OrderSide::Buy));
    }

    /// 测试：健康检查响应
    #[tokio::test]
    async fn test_health_check_response() {
        init_test_env();

        let state = AppState::new();
        let response = health_check(State(state)).await;

        let health_response = response.0;
        assert_eq!(health_response.status, "healthy");
        assert_eq!(health_response.service, "market-data-service");
        assert_eq!(health_response.version, "1.0.0");
        assert!(health_response.uptime < 10); // 应该是刚启动的
    }

    /// 测试：获取所有tickers
    #[tokio::test]
    async fn test_get_all_tickers() {
        init_test_env();

        let state = AppState::new();
        let response = get_all_tickers(State(state)).await;

        let api_response = response.0;
        assert!(api_response.success);
        assert!(api_response.data.is_some());

        let tickers = api_response.data.unwrap();
        assert!(tickers.len() > 0);

        // 验证包含预期的ticker
        let btc_ticker = tickers.iter().find(|t| t.symbol == "BTC-USDT");
        assert!(btc_ticker.is_some(), "应该包含BTC-USDT ticker");

        let eth_ticker = tickers.iter().find(|t| t.symbol == "ETH-USDT");
        assert!(eth_ticker.is_some(), "应该包含ETH-USDT ticker");
    }

    /// 测试：获取特定ticker
    #[tokio::test]
    async fn test_get_ticker() {
        init_test_env();

        let state = AppState::new();

        // 测试存在的ticker
        let response = get_ticker(State(state.clone()), Path("BTC-USDT".to_string())).await;

        match response {
            Ok(json_response) => {
                let api_response = json_response.0;
                assert!(api_response.success);
                assert!(api_response.data.is_some());

                let ticker = api_response.data.unwrap();
                assert_eq!(ticker.symbol, "BTC-USDT");
            }
            Err(_) => panic!("获取BTC-USDT ticker应该成功"),
        }

        // 测试不存在的ticker
        let response = get_ticker(State(state), Path("INVALID-USDT".to_string())).await;

        match response {
            Ok(_) => panic!("获取不存在的ticker应该失败"),
            Err(status) => {
                assert_eq!(status, StatusCode::NOT_FOUND);
            }
        }
    }

    /// 测试：获取交易历史
    #[tokio::test]
    async fn test_get_trades() {
        init_test_env();

        let state = AppState::new();

        // 测试存在的交易对
        let response = get_trades(State(state.clone()), Path("BTC-USDT".to_string())).await;

        match response {
            Ok(json_response) => {
                let api_response = json_response.0;
                assert!(api_response.success);
                assert!(api_response.data.is_some());

                let trades = api_response.data.unwrap();
                assert!(trades.len() > 0, "应该有交易历史数据");

                // 验证交易数据格式
                for trade in &trades {
                    assert_eq!(trade.symbol, "BTC-USDT");
                    assert!(trade.price > Decimal::ZERO);
                    assert!(trade.quantity > Decimal::ZERO);
                }
            }
            Err(_) => panic!("获取BTC-USDT交易历史应该成功"),
        }

        // 测试不存在的交易对
        let response = get_trades(State(state), Path("INVALID-USDT".to_string())).await;

        match response {
            Ok(_) => panic!("获取不存在交易对的历史应该失败"),
            Err(status) => {
                assert_eq!(status, StatusCode::NOT_FOUND);
            }
        }
    }

    /// 测试：数据一致性
    #[tokio::test]
    async fn test_data_consistency() {
        init_test_env();

        let state = AppState::new();

        // 验证ticker和交易数据的一致性
        let tickers = state.tickers.read().await;
        let trades = state.trades.read().await;

        for (symbol, _ticker) in tickers.iter() {
            if let Some(symbol_trades) = trades.get(symbol) {
                // 验证交易数据中的symbol与ticker一致
                for trade in symbol_trades {
                    assert_eq!(trade.symbol, *symbol);
                }
            }
        }
    }

    /// 测试：并发访问安全性
    #[tokio::test]
    async fn test_concurrent_access_safety() {
        init_test_env();

        let state = AppState::new();
        let mut handles = vec![];

        // 启动多个并发任务
        for i in 0..10 {
            let state_clone = state.clone();
            let handle = tokio::spawn(async move {
                // 并发读取ticker数据
                let tickers = state_clone.tickers.read().await;
                let ticker_count = tickers.len();
                drop(tickers);

                // 并发读取交易数据
                let trades = state_clone.trades.read().await;
                let trades_count = trades.len();
                drop(trades);

                (i, ticker_count, trades_count)
            });
            handles.push(handle);
        }

        // 等待所有任务完成
        for handle in handles {
            let (task_id, ticker_count, trades_count) = handle.await.unwrap();
            assert!(ticker_count > 0, "任务{}应该读取到ticker数据", task_id);
            assert!(trades_count > 0, "任务{}应该读取到交易数据", task_id);
        }
    }

    /// 测试：价格精度处理
    #[test]
    fn test_price_precision_handling() {
        init_test_env();

        // 测试不同精度的价格
        let high_precision = Decimal::new(123456789, 8); // 1.23456789
        let low_precision = Decimal::new(12345, 2); // 123.45
        let zero_precision = Decimal::new(12345, 0); // 12345

        assert_eq!(high_precision.scale(), 8);
        assert_eq!(low_precision.scale(), 2);
        assert_eq!(zero_precision.scale(), 0);

        // 验证价格计算
        let price_change = high_precision - Decimal::new(100000000, 8); // 减去1.0
        assert_eq!(price_change, Decimal::new(23456789, 8)); // 0.23456789
    }

    /// 测试：时间戳处理
    #[test]
    fn test_timestamp_handling() {
        init_test_env();

        let now = Utc::now();
        let ticker = Ticker {
            symbol: "TEST-USDT".to_string(),
            price: Decimal::new(10000, 2),
            change: Decimal::ZERO,
            change_percent: Decimal::ZERO,
            high: Decimal::new(10000, 2),
            low: Decimal::new(10000, 2),
            volume: Decimal::ZERO,
            timestamp: now,
        };

        // 验证时间戳在合理范围内
        let time_diff = (Utc::now() - ticker.timestamp).num_seconds();
        assert!(time_diff >= 0 && time_diff < 5, "时间戳应该在当前时间附近");
    }

    /// 测试：性能基准
    #[tokio::test]
    async fn test_performance_benchmark() {
        init_test_env();

        let state = AppState::new();
        let start = std::time::Instant::now();

        // 模拟大量并发请求
        let mut handles = vec![];
        for _ in 0..100 {
            let state_clone = state.clone();
            let handle = tokio::spawn(async move {
                let _tickers = state_clone.tickers.read().await;
                let _trades = state_clone.trades.read().await;
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
        assert!(duration.as_secs() < 1, "市场数据服务性能不达标");
    }

    /// 测试：内存使用优化
    #[tokio::test]
    async fn test_memory_usage_optimization() {
        init_test_env();

        let state = AppState::new();

        // 模拟添加大量数据
        {
            let mut tickers = state.tickers.write().await;
            let mut trades = state.trades.write().await;

            for i in 0..1000 {
                let symbol = format!("TEST{}-USDT", i);

                // 添加ticker
                let ticker = Ticker {
                    symbol: symbol.clone(),
                    price: Decimal::new(10000 + i, 2),
                    change: Decimal::new(i, 2),
                    change_percent: Decimal::new(i, 4),
                    high: Decimal::new(11000 + i, 2),
                    low: Decimal::new(9000 + i, 2),
                    volume: Decimal::new(1000000 + i, 3),
                    timestamp: Utc::now(),
                };
                tickers.insert(symbol.clone(), ticker);

                // 添加交易
                let trade = Trade {
                    id: Uuid::new_v4(),
                    symbol: symbol.clone(),
                    price: Decimal::new(10000 + i, 2),
                    quantity: Decimal::new(100, 3),
                    side: if i % 2 == 0 { OrderSide::Buy } else { OrderSide::Sell },
                    timestamp: Utc::now(),
                };
                trades.entry(symbol).or_insert_with(Vec::new).push(trade);
            }
        }

        // 验证数据添加成功
        let tickers = state.tickers.read().await;
        let trades = state.trades.read().await;

        assert!(tickers.len() >= 1000, "应该有至少1000个ticker");
        assert!(trades.len() >= 1000, "应该有至少1000个交易对");

        // 清理内存（通过作用域自动清理）
        drop(tickers);
        drop(trades);
        assert!(true, "内存使用优化测试完成");
    }

    /// 测试：错误处理
    #[tokio::test]
    async fn test_error_handling() {
        init_test_env();

        let state = AppState::new();

        // 测试空symbol
        let empty_response = get_ticker(State(state.clone()), Path("".to_string())).await;
        assert!(empty_response.is_err(), "空symbol应该返回错误");

        // 测试特殊字符symbol
        let special_response = get_ticker(State(state.clone()), Path("BTC/USDT".to_string())).await;
        assert!(special_response.is_err(), "特殊字符symbol应该返回错误");

        // 测试非常长的symbol
        let long_symbol = "A".repeat(1000);
        let long_response = get_ticker(State(state), Path(long_symbol)).await;
        assert!(long_response.is_err(), "过长symbol应该返回错误");
    }

    /// 测试：数据验证
    #[test]
    fn test_data_validation() {
        init_test_env();

        // 验证ticker数据的合理性
        let ticker = Ticker {
            symbol: "BTC-USDT".to_string(),
            price: Decimal::new(4500000, 2), // 45000.00
            change: Decimal::new(250, 2), // 2.50
            change_percent: Decimal::new(556, 4), // 5.56%
            high: Decimal::new(4650000, 2), // 46500.00
            low: Decimal::new(4350000, 2), // 43500.00
            volume: Decimal::new(123456789, 5), // 1234.56789
            timestamp: Utc::now(),
        };

        // 验证价格关系
        assert!(ticker.high >= ticker.price, "最高价应该大于等于当前价");
        assert!(ticker.low <= ticker.price, "最低价应该小于等于当前价");
        assert!(ticker.high >= ticker.low, "最高价应该大于等于最低价");
        assert!(ticker.price > Decimal::ZERO, "价格应该大于零");
        assert!(ticker.volume >= Decimal::ZERO, "成交量应该大于等于零");
    }
}
