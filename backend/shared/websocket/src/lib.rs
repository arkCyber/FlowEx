//! FlowEx WebSocket Service
//!
//! Real-time data streaming service for market data, order updates,
//! and trading notifications using WebSocket connections.

use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::Response,
};
use dashmap::DashMap;
use flowex_types::{OrderBook, Ticker, Trade, Order, FlowExError, FlowExResult};
use futures_util::{sink::SinkExt, stream::StreamExt};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::broadcast;
use tracing::{info, warn, error, debug};
use uuid::Uuid;

/// WebSocket message types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum WsMessage {
    // Subscription management
    Subscribe { channels: Vec<String> },
    Unsubscribe { channels: Vec<String> },
    
    // Market data
    OrderBookUpdate(OrderBook),
    TickerUpdate(Ticker),
    TradeUpdate(Trade),
    
    // User-specific data
    OrderUpdate(Order),
    BalanceUpdate { currency: String, available: String, locked: String },
    
    // System messages
    Ping,
    Pong,
    Error { message: String },
    Success { message: String },
}

/// WebSocket connection information
#[derive(Debug, Clone)]
pub struct ConnectionInfo {
    pub id: Uuid,
    pub user_id: Option<Uuid>,
    pub subscriptions: Vec<String>,
    pub connected_at: chrono::DateTime<chrono::Utc>,
    pub last_ping: chrono::DateTime<chrono::Utc>,
}

/// WebSocket manager for handling real-time connections
#[derive(Clone)]
pub struct WebSocketManager {
    connections: Arc<DashMap<Uuid, ConnectionInfo>>,
    market_data_tx: broadcast::Sender<WsMessage>,
    user_data_txs: Arc<DashMap<Uuid, broadcast::Sender<WsMessage>>>,
    max_connections: usize,
}

impl WebSocketManager {
    /// Create a new WebSocket manager
    pub fn new(max_connections: usize) -> Self {
        let (market_data_tx, _) = broadcast::channel(1000);
        
        Self {
            connections: Arc::new(DashMap::new()),
            market_data_tx,
            user_data_txs: Arc::new(DashMap::new()),
            max_connections,
        }
    }

    /// Handle WebSocket upgrade
    pub async fn handle_websocket(
        &self,
        ws: WebSocketUpgrade,
        user_id: Option<Uuid>,
    ) -> Response {
        let manager = self.clone();
        
        ws.on_upgrade(move |socket| async move {
            if let Err(e) = manager.handle_connection(socket, user_id).await {
                error!("WebSocket connection error: {}", e);
            }
        })
    }

    /// Handle a WebSocket connection
    async fn handle_connection(&self, socket: WebSocket, user_id: Option<Uuid>) -> FlowExResult<()> {
        // Check connection limit
        if self.connections.len() >= self.max_connections {
            warn!("WebSocket connection limit reached");
            return Err(FlowExError::Internal("Connection limit reached".to_string()));
        }

        let connection_id = Uuid::new_v4();
        let connection_info = ConnectionInfo {
            id: connection_id,
            user_id,
            subscriptions: Vec::new(),
            connected_at: chrono::Utc::now(),
            last_ping: chrono::Utc::now(),
        };

        // Add connection to manager
        self.connections.insert(connection_id, connection_info);
        info!("New WebSocket connection: {} (user: {:?})", connection_id, user_id);

        // Split socket into sender and receiver
        let (mut sender, mut receiver) = socket.split();

        // Subscribe to market data
        let mut market_data_rx = self.market_data_tx.subscribe();

        // Subscribe to user data if authenticated
        let mut user_data_rx = if let Some(uid) = user_id {
            let (tx, rx) = broadcast::channel(100);
            self.user_data_txs.insert(uid, tx);
            Some(rx)
        } else {
            None
        };

        // Handle incoming messages
        let connections = self.connections.clone();
        let incoming_task = tokio::spawn(async move {
            while let Some(msg) = receiver.next().await {
                match msg {
                    Ok(Message::Text(text)) => {
                        if let Err(e) = Self::handle_incoming_message(&connections, connection_id, &text).await {
                            error!("Error handling incoming message: {}", e);
                        }
                    }
                    Ok(Message::Ping(data)) => {
                        if sender.send(Message::Pong(data)).await.is_err() {
                            break;
                        }
                        // Update last ping time
                        if let Some(mut conn) = connections.get_mut(&connection_id) {
                            conn.last_ping = chrono::Utc::now();
                        }
                    }
                    Ok(Message::Close(_)) => {
                        info!("WebSocket connection closed: {}", connection_id);
                        break;
                    }
                    Err(e) => {
                        error!("WebSocket error: {}", e);
                        break;
                    }
                    _ => {}
                }
            }
        });

        // Handle outgoing messages
        let outgoing_task = tokio::spawn(async move {
            loop {
                tokio::select! {
                    // Market data messages
                    Ok(msg) = market_data_rx.recv() => {
                        if Self::should_send_message(&connections, connection_id, &msg) {
                            let json = serde_json::to_string(&msg).unwrap_or_default();
                            if sender.send(Message::Text(json)).await.is_err() {
                                break;
                            }
                        }
                    }
                    
                    // User-specific messages
                    Ok(msg) = async {
                        if let Some(ref mut rx) = user_data_rx {
                            rx.recv().await
                        } else {
                            std::future::pending().await
                        }
                    } => {
                        let json = serde_json::to_string(&msg).unwrap_or_default();
                        if sender.send(Message::Text(json)).await.is_err() {
                            break;
                        }
                    }
                    
                    else => break,
                }
            }
        });

        // Wait for either task to complete
        tokio::select! {
            _ = incoming_task => {},
            _ = outgoing_task => {},
        }

        // Clean up connection
        self.connections.remove(&connection_id);
        if let Some(uid) = user_id {
            self.user_data_txs.remove(&uid);
        }
        info!("WebSocket connection cleaned up: {}", connection_id);

        Ok(())
    }

    /// Handle incoming WebSocket message
    async fn handle_incoming_message(
        connections: &DashMap<Uuid, ConnectionInfo>,
        connection_id: Uuid,
        text: &str,
    ) -> FlowExResult<()> {
        let message: WsMessage = serde_json::from_str(text)
            .map_err(|e| FlowExError::Validation(format!("Invalid message format: {}", e)))?;

        match message {
            WsMessage::Subscribe { channels } => {
                if let Some(mut conn) = connections.get_mut(&connection_id) {
                    for channel in channels {
                        if !conn.subscriptions.contains(&channel) {
                            conn.subscriptions.push(channel.clone());
                            debug!("Connection {} subscribed to {}", connection_id, channel);
                        }
                    }
                }
            }
            WsMessage::Unsubscribe { channels } => {
                if let Some(mut conn) = connections.get_mut(&connection_id) {
                    for channel in channels {
                        conn.subscriptions.retain(|c| c != &channel);
                        debug!("Connection {} unsubscribed from {}", connection_id, channel);
                    }
                }
            }
            WsMessage::Ping => {
                // Ping will be handled by the message loop
            }
            _ => {
                warn!("Unexpected message type from client: {:?}", message);
            }
        }

        Ok(())
    }

    /// Check if a message should be sent to a connection
    fn should_send_message(
        connections: &DashMap<Uuid, ConnectionInfo>,
        connection_id: Uuid,
        message: &WsMessage,
    ) -> bool {
        if let Some(conn) = connections.get(&connection_id) {
            match message {
                WsMessage::OrderBookUpdate(order_book) => {
                    conn.subscriptions.contains(&format!("orderbook.{}", order_book.symbol))
                }
                WsMessage::TickerUpdate(ticker) => {
                    conn.subscriptions.contains(&format!("ticker.{}", ticker.symbol))
                        || conn.subscriptions.contains("ticker.all")
                }
                WsMessage::TradeUpdate(trade) => {
                    conn.subscriptions.contains(&format!("trades.{}", trade.symbol))
                        || conn.subscriptions.contains("trades.all")
                }
                WsMessage::OrderUpdate(_) | WsMessage::BalanceUpdate { .. } => {
                    // User-specific messages are always sent if user is authenticated
                    conn.user_id.is_some()
                }
                _ => true, // System messages are always sent
            }
        } else {
            false
        }
    }

    /// Broadcast market data to all subscribed connections
    pub async fn broadcast_market_data(&self, message: WsMessage) -> FlowExResult<()> {
        if self.market_data_tx.send(message).is_err() {
            warn!("No active market data subscribers");
        }
        Ok(())
    }

    /// Send user-specific data
    pub async fn send_user_data(&self, user_id: Uuid, message: WsMessage) -> FlowExResult<()> {
        if let Some(tx) = self.user_data_txs.get(&user_id) {
            if tx.send(message).is_err() {
                warn!("Failed to send user data to user: {}", user_id);
            }
        }
        Ok(())
    }

    /// Get connection statistics
    pub fn get_stats(&self) -> ConnectionStats {
        let total_connections = self.connections.len();
        let authenticated_connections = self.connections
            .iter()
            .filter(|entry| entry.value().user_id.is_some())
            .count();

        ConnectionStats {
            total_connections,
            authenticated_connections,
            anonymous_connections: total_connections - authenticated_connections,
            max_connections: self.max_connections,
        }
    }

    /// Clean up stale connections
    pub async fn cleanup_stale_connections(&self, timeout_minutes: i64) {
        let cutoff = chrono::Utc::now() - chrono::Duration::minutes(timeout_minutes);
        let mut stale_connections = Vec::new();

        for entry in self.connections.iter() {
            if entry.value().last_ping < cutoff {
                stale_connections.push(*entry.key());
            }
        }

        for connection_id in stale_connections {
            self.connections.remove(&connection_id);
            info!("Removed stale WebSocket connection: {}", connection_id);
        }
    }
}

/// WebSocket connection statistics
#[derive(Debug, Clone, Serialize)]
pub struct ConnectionStats {
    pub total_connections: usize,
    pub authenticated_connections: usize,
    pub anonymous_connections: usize,
    pub max_connections: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_websocket_manager_creation() {
        let manager = WebSocketManager::new(100);
        let stats = manager.get_stats();
        
        assert_eq!(stats.total_connections, 0);
        assert_eq!(stats.max_connections, 100);
    }

    #[tokio::test]
    async fn test_message_serialization() {
        let message = WsMessage::Subscribe {
            channels: vec!["ticker.BTCUSDT".to_string()],
        };
        
        let json = serde_json::to_string(&message).unwrap();
        let deserialized: WsMessage = serde_json::from_str(&json).unwrap();
        
        match deserialized {
            WsMessage::Subscribe { channels } => {
                assert_eq!(channels, vec!["ticker.BTCUSDT".to_string()]);
            }
            _ => panic!("Unexpected message type"),
        }
    }
}
