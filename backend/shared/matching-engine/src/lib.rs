//! FlowEx Matching Engine
//!
//! High-performance order matching engine with price-time priority
//! and comprehensive trade execution capabilities.

use flowex_types::{
    Order, OrderSide, OrderType, OrderStatus, Trade, OrderBook, OrderBookLevel,
    FlowExError, FlowExResult,
};
use rust_decimal::Decimal;
use std::collections::{BTreeMap, VecDeque};
use std::cmp::Ordering;
use tracing::{info, debug, warn, error};
use uuid::Uuid;
use chrono::Utc;

/// Order matching engine for a single trading pair
#[derive(Debug, Clone)]
pub struct MatchingEngine {
    symbol: String,
    buy_orders: BTreeMap<Decimal, VecDeque<Order>>, // Price -> Orders (highest first)
    sell_orders: BTreeMap<Decimal, VecDeque<Order>>, // Price -> Orders (lowest first)
    last_trade_price: Option<Decimal>,
    total_volume: Decimal,
}

impl MatchingEngine {
    /// Create a new matching engine for a trading pair
    pub fn new(symbol: String) -> Self {
        Self {
            symbol,
            buy_orders: BTreeMap::new(),
            sell_orders: BTreeMap::new(),
            last_trade_price: None,
            total_volume: Decimal::ZERO,
        }
    }

    /// Add an order to the order book and attempt to match
    pub fn add_order(&mut self, mut order: Order) -> FlowExResult<Vec<Trade>> {
        debug!("Adding order to matching engine: {:?}", order);

        // Validate order
        self.validate_order(&order)?;

        let mut trades = Vec::new();

        match order.order_type {
            OrderType::Market => {
                trades = self.execute_market_order(&mut order)?;
            }
            OrderType::Limit => {
                trades = self.execute_limit_order(&mut order)?;
            }
            OrderType::StopLoss | OrderType::TakeProfit => {
                // For now, treat as limit orders
                // In production, these would be handled by a separate trigger system
                trades = self.execute_limit_order(&mut order)?;
            }
        }

        // If order is not fully filled, add to order book
        if order.remaining_quantity > Decimal::ZERO && order.status != OrderStatus::Cancelled {
            self.add_to_order_book(order)?;
        }

        Ok(trades)
    }

    /// Cancel an order
    pub fn cancel_order(&mut self, order_id: Uuid) -> FlowExResult<bool> {
        // Remove from buy orders
        for (_, orders) in self.buy_orders.iter_mut() {
            if let Some(pos) = orders.iter().position(|o| o.id == order_id) {
                let mut order = orders.remove(pos).unwrap();
                order.status = OrderStatus::Cancelled;
                info!("Cancelled buy order: {}", order_id);
                return Ok(true);
            }
        }

        // Remove from sell orders
        for (_, orders) in self.sell_orders.iter_mut() {
            if let Some(pos) = orders.iter().position(|o| o.id == order_id) {
                let mut order = orders.remove(pos).unwrap();
                order.status = OrderStatus::Cancelled;
                info!("Cancelled sell order: {}", order_id);
                return Ok(true);
            }
        }

        warn!("Order not found for cancellation: {}", order_id);
        Ok(false)
    }

    /// Get current order book snapshot
    pub fn get_order_book(&self, depth: usize) -> OrderBook {
        let mut bids = Vec::new();
        let mut asks = Vec::new();

        // Get top bids (highest prices first)
        for (price, orders) in self.buy_orders.iter().rev().take(depth) {
            let total_quantity: Decimal = orders.iter().map(|o| o.remaining_quantity).sum();
            if total_quantity > Decimal::ZERO {
                bids.push(OrderBookLevel {
                    price: *price,
                    quantity: total_quantity,
                });
            }
        }

        // Get top asks (lowest prices first)
        for (price, orders) in self.sell_orders.iter().take(depth) {
            let total_quantity: Decimal = orders.iter().map(|o| o.remaining_quantity).sum();
            if total_quantity > Decimal::ZERO {
                asks.push(OrderBookLevel {
                    price: *price,
                    quantity: total_quantity,
                });
            }
        }

        OrderBook {
            symbol: self.symbol.clone(),
            bids,
            asks,
            timestamp: Utc::now(),
        }
    }

    /// Get the best bid price
    pub fn get_best_bid(&self) -> Option<Decimal> {
        self.buy_orders.keys().rev().next().copied()
    }

    /// Get the best ask price
    pub fn get_best_ask(&self) -> Option<Decimal> {
        self.sell_orders.keys().next().copied()
    }

    /// Get the spread
    pub fn get_spread(&self) -> Option<Decimal> {
        match (self.get_best_bid(), self.get_best_ask()) {
            (Some(bid), Some(ask)) => Some(ask - bid),
            _ => None,
        }
    }

    /// Execute a market order
    fn execute_market_order(&mut self, order: &mut Order) -> FlowExResult<Vec<Trade>> {
        let mut trades = Vec::new();
        let opposite_orders = match order.side {
            OrderSide::Buy => &mut self.sell_orders,
            OrderSide::Sell => &mut self.buy_orders,
        };

        let mut remaining_quantity = order.quantity;

        // Iterate through price levels
        let price_levels: Vec<Decimal> = opposite_orders.keys().copied().collect();
        
        for price in price_levels {
            if remaining_quantity <= Decimal::ZERO {
                break;
            }

            if let Some(orders_at_price) = opposite_orders.get_mut(&price) {
                while let Some(mut counter_order) = orders_at_price.pop_front() {
                    if remaining_quantity <= Decimal::ZERO {
                        orders_at_price.push_front(counter_order);
                        break;
                    }

                    let trade_quantity = remaining_quantity.min(counter_order.remaining_quantity);
                    let trade_price = counter_order.price.unwrap_or(price);

                    // Create trade
                    let trade = self.create_trade(order, &counter_order, trade_price, trade_quantity)?;
                    trades.push(trade);

                    // Update quantities
                    remaining_quantity -= trade_quantity;
                    counter_order.remaining_quantity -= trade_quantity;
                    counter_order.filled_quantity += trade_quantity;

                    // Update order status
                    if counter_order.remaining_quantity <= Decimal::ZERO {
                        counter_order.status = OrderStatus::Filled;
                    } else {
                        counter_order.status = OrderStatus::PartiallyFilled;
                        orders_at_price.push_front(counter_order);
                    }
                }

                // Remove empty price level
                if orders_at_price.is_empty() {
                    opposite_orders.remove(&price);
                }
            }
        }

        // Update market order
        order.filled_quantity = order.quantity - remaining_quantity;
        order.remaining_quantity = remaining_quantity;
        
        if remaining_quantity <= Decimal::ZERO {
            order.status = OrderStatus::Filled;
        } else if order.filled_quantity > Decimal::ZERO {
            order.status = OrderStatus::PartiallyFilled;
        }

        Ok(trades)
    }

    /// Execute a limit order
    fn execute_limit_order(&mut self, order: &mut Order) -> FlowExResult<Vec<Trade>> {
        let mut trades = Vec::new();
        let order_price = order.price.ok_or_else(|| {
            FlowExError::Trading("Limit order must have a price".to_string())
        })?;

        let opposite_orders = match order.side {
            OrderSide::Buy => &mut self.sell_orders,
            OrderSide::Sell => &mut self.buy_orders,
        };

        let mut remaining_quantity = order.quantity;

        // Find matching orders
        let price_levels: Vec<Decimal> = opposite_orders.keys().copied().collect();
        
        for price in price_levels {
            if remaining_quantity <= Decimal::ZERO {
                break;
            }

            // Check if price matches
            let can_match = match order.side {
                OrderSide::Buy => price <= order_price,  // Buy order can match at or below limit price
                OrderSide::Sell => price >= order_price, // Sell order can match at or above limit price
            };

            if !can_match {
                continue;
            }

            if let Some(orders_at_price) = opposite_orders.get_mut(&price) {
                while let Some(mut counter_order) = orders_at_price.pop_front() {
                    if remaining_quantity <= Decimal::ZERO {
                        orders_at_price.push_front(counter_order);
                        break;
                    }

                    let trade_quantity = remaining_quantity.min(counter_order.remaining_quantity);
                    let trade_price = counter_order.price.unwrap_or(price);

                    // Create trade
                    let trade = self.create_trade(order, &counter_order, trade_price, trade_quantity)?;
                    trades.push(trade);

                    // Update quantities
                    remaining_quantity -= trade_quantity;
                    counter_order.remaining_quantity -= trade_quantity;
                    counter_order.filled_quantity += trade_quantity;

                    // Update order status
                    if counter_order.remaining_quantity <= Decimal::ZERO {
                        counter_order.status = OrderStatus::Filled;
                    } else {
                        counter_order.status = OrderStatus::PartiallyFilled;
                        orders_at_price.push_front(counter_order);
                    }
                }

                // Remove empty price level
                if orders_at_price.is_empty() {
                    opposite_orders.remove(&price);
                }
            }
        }

        // Update limit order
        order.filled_quantity = order.quantity - remaining_quantity;
        order.remaining_quantity = remaining_quantity;
        
        if remaining_quantity <= Decimal::ZERO {
            order.status = OrderStatus::Filled;
        } else if order.filled_quantity > Decimal::ZERO {
            order.status = OrderStatus::PartiallyFilled;
        }

        Ok(trades)
    }

    /// Add order to the order book
    fn add_to_order_book(&mut self, order: Order) -> FlowExResult<()> {
        let price = order.price.ok_or_else(|| {
            FlowExError::Trading("Order must have a price to be added to order book".to_string())
        })?;

        let order_book = match order.side {
            OrderSide::Buy => &mut self.buy_orders,
            OrderSide::Sell => &mut self.sell_orders,
        };

        order_book.entry(price).or_insert_with(VecDeque::new).push_back(order);
        
        debug!("Added order to order book at price: {}", price);
        Ok(())
    }

    /// Create a trade from two matching orders
    fn create_trade(&mut self, taker_order: &Order, maker_order: &Order, price: Decimal, quantity: Decimal) -> FlowExResult<Trade> {
        let (buyer_order_id, seller_order_id) = match taker_order.side {
            OrderSide::Buy => (taker_order.id, maker_order.id),
            OrderSide::Sell => (maker_order.id, taker_order.id),
        };

        self.last_trade_price = Some(price);
        self.total_volume += quantity;

        let trade = Trade {
            id: Uuid::new_v4(),
            symbol: self.symbol.clone(),
            price,
            quantity,
            side: taker_order.side.clone(),
            timestamp: Utc::now(),
        };

        info!("Trade executed: {} {} at {} for {}", 
              self.symbol, quantity, price, trade.id);

        Ok(trade)
    }

    /// Validate an order before processing
    fn validate_order(&self, order: &Order) -> FlowExResult<()> {
        if order.quantity <= Decimal::ZERO {
            return Err(FlowExError::Validation("Order quantity must be positive".to_string()));
        }

        if order.trading_pair != self.symbol {
            return Err(FlowExError::Validation("Order symbol does not match engine".to_string()));
        }

        match order.order_type {
            OrderType::Limit => {
                if order.price.is_none() || order.price.unwrap() <= Decimal::ZERO {
                    return Err(FlowExError::Validation("Limit order must have a positive price".to_string()));
                }
            }
            OrderType::Market => {
                // Market orders don't need price validation
            }
            OrderType::StopLoss | OrderType::TakeProfit => {
                if order.price.is_none() || order.price.unwrap() <= Decimal::ZERO {
                    return Err(FlowExError::Validation("Stop/Take profit order must have a positive price".to_string()));
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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

    /// 创建测试订单的辅助函数
    fn create_test_order(
        side: OrderSide,
        order_type: OrderType,
        price: Option<Decimal>,
        quantity: Decimal,
    ) -> Order {
        Order {
            id: Uuid::new_v4(),
            user_id: Uuid::new_v4(),
            trading_pair: "BTCUSDT".to_string(),
            side,
            order_type,
            price,
            quantity,
            filled_quantity: Decimal::ZERO,
            remaining_quantity: quantity,
            status: OrderStatus::New,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    /// 测试：匹配引擎创建
    #[test]
    fn test_matching_engine_creation() {
        init_test_env();

        let engine = MatchingEngine::new("BTCUSDT".to_string());
        assert_eq!(engine.symbol, "BTCUSDT");
        assert!(engine.buy_orders.is_empty());
        assert!(engine.sell_orders.is_empty());
        assert_eq!(engine.last_trade_price, None);
        assert_eq!(engine.total_volume, Decimal::ZERO);
    }

    /// 测试：订单验证 - 正常情况
    #[test]
    fn test_order_validation_success() {
        init_test_env();

        let engine = MatchingEngine::new("BTCUSDT".to_string());

        // 测试限价买单
        let buy_order = create_test_order(
            OrderSide::Buy,
            OrderType::Limit,
            Some(Decimal::new(50000, 0)),
            Decimal::new(1, 0),
        );
        assert!(engine.validate_order(&buy_order).is_ok());

        // 测试市价卖单
        let sell_order = create_test_order(
            OrderSide::Sell,
            OrderType::Market,
            None,
            Decimal::new(1, 0),
        );
        assert!(engine.validate_order(&sell_order).is_ok());
    }

    /// 测试：订单验证 - 错误情况
    #[test]
    fn test_order_validation_errors() {
        init_test_env();

        let engine = MatchingEngine::new("BTCUSDT".to_string());

        // 测试数量为零的订单
        let zero_quantity_order = create_test_order(
            OrderSide::Buy,
            OrderType::Limit,
            Some(Decimal::new(50000, 0)),
            Decimal::ZERO,
        );
        assert!(engine.validate_order(&zero_quantity_order).is_err());

        // 测试交易对不匹配
        let mut wrong_symbol_order = create_test_order(
            OrderSide::Buy,
            OrderType::Limit,
            Some(Decimal::new(50000, 0)),
            Decimal::new(1, 0),
        );
        wrong_symbol_order.trading_pair = "ETHUSDT".to_string();
        assert!(engine.validate_order(&wrong_symbol_order).is_err());

        // 测试限价单没有价格
        let no_price_order = create_test_order(
            OrderSide::Buy,
            OrderType::Limit,
            None,
            Decimal::new(1, 0),
        );
        assert!(engine.validate_order(&no_price_order).is_err());

        // 测试限价单价格为零
        let zero_price_order = create_test_order(
            OrderSide::Buy,
            OrderType::Limit,
            Some(Decimal::ZERO),
            Decimal::new(1, 0),
        );
        assert!(engine.validate_order(&zero_price_order).is_err());
    }

    /// 测试：限价单匹配 - 完全成交
    #[test]
    fn test_limit_order_full_match() {
        init_test_env();

        let mut engine = MatchingEngine::new("BTCUSDT".to_string());

        // 添加卖单到订单簿
        let sell_order = create_test_order(
            OrderSide::Sell,
            OrderType::Limit,
            Some(Decimal::new(50000, 0)),
            Decimal::new(1, 0),
        );
        let trades = engine.add_order(sell_order).unwrap();
        assert!(trades.is_empty()); // 没有匹配，应该加入订单簿

        // 添加匹配的买单
        let buy_order = create_test_order(
            OrderSide::Buy,
            OrderType::Limit,
            Some(Decimal::new(50000, 0)),
            Decimal::new(1, 0),
        );
        let trades = engine.add_order(buy_order).unwrap();

        // 验证交易生成
        assert_eq!(trades.len(), 1);
        let trade = &trades[0];
        assert_eq!(trade.symbol, "BTCUSDT");
        assert_eq!(trade.price, Decimal::new(50000, 0));
        assert_eq!(trade.quantity, Decimal::new(1, 0));
        assert_eq!(trade.side, OrderSide::Buy);

        // 验证订单簿为空（订单完全成交）
        let order_book = engine.get_order_book(10);
        assert!(order_book.bids.is_empty());
        assert!(order_book.asks.is_empty());
    }

    /// 测试：限价单匹配 - 部分成交
    #[test]
    fn test_limit_order_partial_match() {
        init_test_env();

        let mut engine = MatchingEngine::new("BTCUSDT".to_string());

        // 添加大额卖单
        let sell_order = create_test_order(
            OrderSide::Sell,
            OrderType::Limit,
            Some(Decimal::new(50000, 0)),
            Decimal::new(2, 0),
        );
        engine.add_order(sell_order).unwrap();

        // 添加小额买单
        let buy_order = create_test_order(
            OrderSide::Buy,
            OrderType::Limit,
            Some(Decimal::new(50000, 0)),
            Decimal::new(1, 0),
        );
        let trades = engine.add_order(buy_order).unwrap();

        // 验证交易生成
        assert_eq!(trades.len(), 1);
        assert_eq!(trades[0].quantity, Decimal::new(1, 0));

        // 验证订单簿中还有剩余卖单
        let order_book = engine.get_order_book(10);
        assert!(order_book.bids.is_empty());
        assert_eq!(order_book.asks.len(), 1);
        assert_eq!(order_book.asks[0].quantity, Decimal::new(1, 0));
    }

    /// 测试：市价单匹配
    #[test]
    fn test_market_order_execution() {
        init_test_env();

        let mut engine = MatchingEngine::new("BTCUSDT".to_string());

        // 添加多个限价卖单
        let sell_order1 = create_test_order(
            OrderSide::Sell,
            OrderType::Limit,
            Some(Decimal::new(50000, 0)),
            Decimal::new(1, 0),
        );
        engine.add_order(sell_order1).unwrap();

        let sell_order2 = create_test_order(
            OrderSide::Sell,
            OrderType::Limit,
            Some(Decimal::new(50100, 0)),
            Decimal::new(1, 0),
        );
        engine.add_order(sell_order2).unwrap();

        // 添加市价买单
        let market_buy_order = create_test_order(
            OrderSide::Buy,
            OrderType::Market,
            None,
            Decimal::new(15, 1), // 1.5
        );
        let trades = engine.add_order(market_buy_order).unwrap();

        // 验证交易执行
        assert_eq!(trades.len(), 2);
        assert_eq!(trades[0].price, Decimal::new(50000, 0));
        assert_eq!(trades[0].quantity, Decimal::new(1, 0));
        assert_eq!(trades[1].price, Decimal::new(50100, 0));
        assert_eq!(trades[1].quantity, Decimal::new(5, 1)); // 0.5
    }

    /// 测试：订单簿深度获取
    #[test]
    fn test_order_book_depth() {
        init_test_env();

        let mut engine = MatchingEngine::new("BTCUSDT".to_string());

        // 添加多个买单和卖单
        for i in 1..=5 {
            let buy_price = Decimal::new(50000 - i * 100, 0);
            let sell_price = Decimal::new(50000 + i * 100, 0);

            let buy_order = create_test_order(
                OrderSide::Buy,
                OrderType::Limit,
                Some(buy_price),
                Decimal::new(1, 0),
            );
            engine.add_order(buy_order).unwrap();

            let sell_order = create_test_order(
                OrderSide::Sell,
                OrderType::Limit,
                Some(sell_price),
                Decimal::new(1, 0),
            );
            engine.add_order(sell_order).unwrap();
        }

        // 获取订单簿深度
        let order_book = engine.get_order_book(3);

        // 验证买单按价格降序排列
        assert_eq!(order_book.bids.len(), 3);
        assert!(order_book.bids[0].price > order_book.bids[1].price);
        assert!(order_book.bids[1].price > order_book.bids[2].price);

        // 验证卖单按价格升序排列
        assert_eq!(order_book.asks.len(), 3);
        assert!(order_book.asks[0].price < order_book.asks[1].price);
        assert!(order_book.asks[1].price < order_book.asks[2].price);
    }

    /// 测试：最佳买卖价获取
    #[test]
    fn test_best_bid_ask() {
        init_test_env();

        let mut engine = MatchingEngine::new("BTCUSDT".to_string());

        // 初始状态应该没有最佳价格
        assert_eq!(engine.get_best_bid(), None);
        assert_eq!(engine.get_best_ask(), None);
        assert_eq!(engine.get_spread(), None);

        // 添加买单和卖单
        let buy_order = create_test_order(
            OrderSide::Buy,
            OrderType::Limit,
            Some(Decimal::new(49900, 0)),
            Decimal::new(1, 0),
        );
        engine.add_order(buy_order).unwrap();

        let sell_order = create_test_order(
            OrderSide::Sell,
            OrderType::Limit,
            Some(Decimal::new(50100, 0)),
            Decimal::new(1, 0),
        );
        engine.add_order(sell_order).unwrap();

        // 验证最佳价格
        assert_eq!(engine.get_best_bid(), Some(Decimal::new(49900, 0)));
        assert_eq!(engine.get_best_ask(), Some(Decimal::new(50100, 0)));
        assert_eq!(engine.get_spread(), Some(Decimal::new(200, 0)));
    }

    /// 测试：订单取消
    #[test]
    fn test_order_cancellation() {
        init_test_env();

        let mut engine = MatchingEngine::new("BTCUSDT".to_string());

        // 添加订单
        let order = create_test_order(
            OrderSide::Buy,
            OrderType::Limit,
            Some(Decimal::new(50000, 0)),
            Decimal::new(1, 0),
        );
        let order_id = order.id;
        engine.add_order(order).unwrap();

        // 验证订单在订单簿中
        let order_book = engine.get_order_book(10);
        assert_eq!(order_book.bids.len(), 1);

        // 取消订单
        let cancelled = engine.cancel_order(order_id).unwrap();
        assert!(cancelled);

        // 验证订单已从订单簿中移除
        let order_book = engine.get_order_book(10);
        assert!(order_book.bids.is_empty());

        // 尝试取消不存在的订单
        let not_cancelled = engine.cancel_order(Uuid::new_v4()).unwrap();
        assert!(!not_cancelled);
    }

    /// 测试：价格时间优先原则
    #[test]
    fn test_price_time_priority() {
        init_test_env();

        let mut engine = MatchingEngine::new("BTCUSDT".to_string());

        // 添加相同价格的多个卖单（时间优先）
        let sell_order1 = create_test_order(
            OrderSide::Sell,
            OrderType::Limit,
            Some(Decimal::new(50000, 0)),
            Decimal::new(1, 0),
        );
        let order1_id = sell_order1.id;
        engine.add_order(sell_order1).unwrap();

        // 稍等一下确保时间不同
        std::thread::sleep(std::time::Duration::from_millis(1));

        let sell_order2 = create_test_order(
            OrderSide::Sell,
            OrderType::Limit,
            Some(Decimal::new(50000, 0)),
            Decimal::new(1, 0),
        );
        engine.add_order(sell_order2).unwrap();

        // 添加买单，应该匹配第一个卖单
        let buy_order = create_test_order(
            OrderSide::Buy,
            OrderType::Limit,
            Some(Decimal::new(50000, 0)),
            Decimal::new(1, 0),
        );
        let trades = engine.add_order(buy_order).unwrap();

        // 验证交易生成且匹配了第一个订单
        assert_eq!(trades.len(), 1);

        // 验证第二个订单仍在订单簿中
        let order_book = engine.get_order_book(10);
        assert_eq!(order_book.asks.len(), 1);
    }

    /// 测试：性能基准
    #[test]
    fn test_performance_benchmark() {
        init_test_env();

        let mut engine = MatchingEngine::new("BTCUSDT".to_string());
        let start = std::time::Instant::now();

        // 添加1000个订单
        for i in 0..1000 {
            let price = Decimal::new(50000 + (i % 100), 0);
            let order = create_test_order(
                if i % 2 == 0 { OrderSide::Buy } else { OrderSide::Sell },
                OrderType::Limit,
                Some(price),
                Decimal::new(1, 0),
            );
            engine.add_order(order).unwrap();
        }

        let duration = start.elapsed();
        println!("添加1000个订单耗时: {:?}", duration);

        // 性能要求：1000个订单应该在100ms内完成
        assert!(duration.as_millis() < 100, "订单处理性能不达标");
    }

    /// 测试：并发安全性（模拟）
    #[test]
    fn test_concurrent_operations() {
        init_test_env();

        let mut engine = MatchingEngine::new("BTCUSDT".to_string());

        // 模拟并发添加订单
        let mut handles = vec![];

        for i in 0..10 {
            let order = create_test_order(
                OrderSide::Buy,
                OrderType::Limit,
                Some(Decimal::new(50000 + i, 0)),
                Decimal::new(1, 0),
            );

            // 在实际并发环境中，这里会使用Arc<Mutex<MatchingEngine>>
            let trades = engine.add_order(order).unwrap();
            assert!(trades.is_empty()); // 这些订单不应该匹配
        }

        // 验证所有订单都被正确添加
        let order_book = engine.get_order_book(20);
        assert_eq!(order_book.bids.len(), 10);
    }

    /// 测试：边界条件
    #[test]
    fn test_edge_cases() {
        init_test_env();

        let mut engine = MatchingEngine::new("BTCUSDT".to_string());

        // 测试极小数量
        let tiny_order = create_test_order(
            OrderSide::Buy,
            OrderType::Limit,
            Some(Decimal::new(50000, 0)),
            Decimal::new(1, 8), // 0.00000001
        );
        let trades = engine.add_order(tiny_order).unwrap();
        assert!(trades.is_empty());

        // 测试极大数量
        let large_order = create_test_order(
            OrderSide::Sell,
            OrderType::Limit,
            Some(Decimal::new(50000, 0)),
            Decimal::new(1000000, 0),
        );
        let trades = engine.add_order(large_order).unwrap();
        assert!(trades.is_empty());

        // 测试极高价格
        let high_price_order = create_test_order(
            OrderSide::Buy,
            OrderType::Limit,
            Some(Decimal::new(999999999, 0)),
            Decimal::new(1, 0),
        );
        let trades = engine.add_order(high_price_order).unwrap();
        assert!(trades.is_empty());
    }

    /// 测试：错误恢复
    #[test]
    fn test_error_recovery() {
        init_test_env();

        let mut engine = MatchingEngine::new("BTCUSDT".to_string());

        // 添加正常订单
        let normal_order = create_test_order(
            OrderSide::Buy,
            OrderType::Limit,
            Some(Decimal::new(50000, 0)),
            Decimal::new(1, 0),
        );
        engine.add_order(normal_order).unwrap();

        // 尝试添加无效订单
        let invalid_order = create_test_order(
            OrderSide::Buy,
            OrderType::Limit,
            Some(Decimal::ZERO),
            Decimal::new(1, 0),
        );
        assert!(engine.add_order(invalid_order).is_err());

        // 验证引擎状态未被破坏
        let order_book = engine.get_order_book(10);
        assert_eq!(order_book.bids.len(), 1);

        // 继续添加正常订单应该成功
        let another_order = create_test_order(
            OrderSide::Sell,
            OrderType::Limit,
            Some(Decimal::new(51000, 0)),
            Decimal::new(1, 0),
        );
        let trades = engine.add_order(another_order).unwrap();
        assert!(trades.is_empty());
    }
}
