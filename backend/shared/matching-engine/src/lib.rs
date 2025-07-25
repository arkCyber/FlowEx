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
