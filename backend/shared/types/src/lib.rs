//! FlowEx Enterprise Types
//! 
//! Comprehensive type definitions for the FlowEx trading platform.
//! Implements enterprise-grade type safety and validation.

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// User account information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub is_verified: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Authentication request
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

/// Authentication response
#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub user: User,
    pub expires_in: i64,
}

/// User registration request
#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
}

/// Trading pair information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TradingPair {
    pub symbol: String,
    pub base_asset: String,
    pub quote_asset: String,
    pub status: TradingStatus,
    pub min_price: Decimal,
    pub max_price: Decimal,
    pub min_qty: Decimal,
    pub max_qty: Decimal,
    pub step_size: Decimal,
    pub tick_size: Decimal,
}

/// Trading status enumeration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub enum TradingStatus {
    Trading,
    Halted,
    Maintenance,
}

/// Order information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Order {
    pub id: Uuid,
    pub user_id: Uuid,
    pub trading_pair: String,
    pub side: OrderSide,
    pub order_type: OrderType,
    pub price: Option<Decimal>,
    pub quantity: Decimal,
    pub filled_quantity: Decimal,
    pub status: OrderStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Order side enumeration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum OrderSide {
    Buy,
    Sell,
}

/// Order type enumeration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum OrderType {
    Market,
    Limit,
    StopLoss,
    TakeProfit,
}

/// Order status enumeration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub enum OrderStatus {
    New,
    PartiallyFilled,
    Filled,
    Cancelled,
    Rejected,
    Expired,
}

/// Create order request
#[derive(Debug, Deserialize)]
pub struct CreateOrderRequest {
    pub trading_pair: String,
    pub side: OrderSide,
    pub order_type: OrderType,
    pub price: Option<Decimal>,
    pub quantity: Decimal,
}

/// Order book level
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderBookLevel {
    pub price: Decimal,
    pub quantity: Decimal,
}

/// Order book snapshot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderBook {
    pub symbol: String,
    pub bids: Vec<OrderBookLevel>,
    pub asks: Vec<OrderBookLevel>,
    pub timestamp: DateTime<Utc>,
}

/// Market ticker information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ticker {
    pub symbol: String,
    pub price: Decimal,
    pub change: Decimal,
    pub change_percent: Decimal,
    pub high: Decimal,
    pub low: Decimal,
    pub volume: Decimal,
    pub timestamp: DateTime<Utc>,
}

/// Trade information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trade {
    pub id: Uuid,
    pub symbol: String,
    pub price: Decimal,
    pub quantity: Decimal,
    pub side: OrderSide,
    pub timestamp: DateTime<Utc>,
}

/// Wallet balance information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Balance {
    pub currency: String,
    pub available: Decimal,
    pub locked: Decimal,
}

/// Transaction information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub id: Uuid,
    pub user_id: Uuid,
    pub transaction_type: TransactionType,
    pub currency: String,
    pub amount: Decimal,
    pub status: TransactionStatus,
    pub created_at: DateTime<Utc>,
}

/// Transaction type enumeration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TransactionType {
    Deposit,
    Withdrawal,
    Trade,
    Fee,
}

/// Transaction status enumeration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TransactionStatus {
    Pending,
    Completed,
    Failed,
    Cancelled,
}

/// API response wrapper
#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
    pub timestamp: DateTime<Utc>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
            timestamp: Utc::now(),
        }
    }

    pub fn error(message: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(message),
            timestamp: Utc::now(),
        }
    }
}

/// Health check response
#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub service: String,
    pub version: String,
    pub timestamp: DateTime<Utc>,
    pub uptime: u64,
}

/// Error types for the application
#[derive(thiserror::Error, Debug)]
pub enum FlowExError {
    #[error("Database error: {0}")]
    Database(String),
    
    #[error("Authentication error: {0}")]
    Authentication(String),
    
    #[error("Authorization error: {0}")]
    Authorization(String),
    
    #[error("Validation error: {0}")]
    Validation(String),
    
    #[error("Trading error: {0}")]
    Trading(String),
    
    #[error("Market data error: {0}")]
    MarketData(String),
    
    #[error("Wallet error: {0}")]
    Wallet(String),
    
    #[error("Internal server error: {0}")]
    Internal(String),
}

/// Result type alias for FlowEx operations
pub type FlowExResult<T> = Result<T, FlowExError>;

/// Configuration for services
#[derive(Debug, Clone)]
pub struct ServiceConfig {
    pub host: String,
    pub port: u16,
    pub database_url: String,
    pub redis_url: String,
    pub jwt_secret: String,
    pub log_level: String,
}

/// JWT Claims structure
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct JwtClaims {
    pub sub: String,        // Subject (user ID)
    pub email: String,      // User email
    pub exp: usize,         // Expiration time
    pub iat: usize,         // Issued at
    pub jti: String,        // JWT ID (for token revocation)
    pub roles: Vec<String>, // User roles
    pub permissions: Vec<String>, // User permissions
}

/// Authentication context
#[derive(Debug, Clone)]
pub struct AuthContext {
    pub user_id: Uuid,
    pub email: String,
    pub roles: Vec<String>,
    pub permissions: Vec<String>,
    pub session_id: String,
}

/// Permission levels
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Permission {
    // User permissions
    UserRead,
    UserWrite,
    UserDelete,

    // Trading permissions
    TradingRead,
    TradingWrite,
    TradingCancel,

    // Wallet permissions
    WalletRead,
    WalletDeposit,
    WalletWithdraw,

    // Admin permissions
    AdminRead,
    AdminWrite,
    AdminDelete,

    // System permissions
    SystemRead,
    SystemWrite,
    SystemMaintenance,
}

impl Permission {
    pub fn as_str(&self) -> &'static str {
        match self {
            Permission::UserRead => "user:read",
            Permission::UserWrite => "user:write",
            Permission::UserDelete => "user:delete",
            Permission::TradingRead => "trading:read",
            Permission::TradingWrite => "trading:write",
            Permission::TradingCancel => "trading:cancel",
            Permission::WalletRead => "wallet:read",
            Permission::WalletDeposit => "wallet:deposit",
            Permission::WalletWithdraw => "wallet:withdraw",
            Permission::AdminRead => "admin:read",
            Permission::AdminWrite => "admin:write",
            Permission::AdminDelete => "admin:delete",
            Permission::SystemRead => "system:read",
            Permission::SystemWrite => "system:write",
            Permission::SystemMaintenance => "system:maintenance",
        }
    }
}

/// User roles
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Role {
    User,
    Trader,
    VipTrader,
    Admin,
    SuperAdmin,
    System,
}

impl Role {
    pub fn as_str(&self) -> &'static str {
        match self {
            Role::User => "user",
            Role::Trader => "trader",
            Role::VipTrader => "vip_trader",
            Role::Admin => "admin",
            Role::SuperAdmin => "super_admin",
            Role::System => "system",
        }
    }

    pub fn permissions(&self) -> Vec<Permission> {
        match self {
            Role::User => vec![
                Permission::UserRead,
                Permission::WalletRead,
            ],
            Role::Trader => vec![
                Permission::UserRead,
                Permission::UserWrite,
                Permission::TradingRead,
                Permission::TradingWrite,
                Permission::TradingCancel,
                Permission::WalletRead,
                Permission::WalletDeposit,
                Permission::WalletWithdraw,
            ],
            Role::VipTrader => {
                let mut perms = Role::Trader.permissions();
                // VIP traders get same permissions as traders for now
                perms
            },
            Role::Admin => vec![
                Permission::UserRead,
                Permission::UserWrite,
                Permission::UserDelete,
                Permission::TradingRead,
                Permission::TradingWrite,
                Permission::TradingCancel,
                Permission::WalletRead,
                Permission::WalletDeposit,
                Permission::WalletWithdraw,
                Permission::AdminRead,
                Permission::AdminWrite,
                Permission::SystemRead,
            ],
            Role::SuperAdmin => vec![
                Permission::UserRead,
                Permission::UserWrite,
                Permission::UserDelete,
                Permission::TradingRead,
                Permission::TradingWrite,
                Permission::TradingCancel,
                Permission::WalletRead,
                Permission::WalletDeposit,
                Permission::WalletWithdraw,
                Permission::AdminRead,
                Permission::AdminWrite,
                Permission::AdminDelete,
                Permission::SystemRead,
                Permission::SystemWrite,
                Permission::SystemMaintenance,
            ],
            Role::System => vec![
                Permission::SystemRead,
                Permission::SystemWrite,
                Permission::SystemMaintenance,
            ],
        }
    }
}

/// Metrics data structure
#[derive(Debug, Clone, Serialize)]
pub struct ServiceMetrics {
    pub requests_total: u64,
    pub requests_success: u64,
    pub requests_error: u64,
    pub response_time_avg: f64,
    pub active_connections: u32,
    pub uptime_seconds: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_serialization() {
        let user = User {
            id: Uuid::new_v4(),
            email: "test@example.com".to_string(),
            first_name: "Test".to_string(),
            last_name: "User".to_string(),
            is_verified: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let json = serde_json::to_string(&user).unwrap();
        let deserialized: User = serde_json::from_str(&json).unwrap();
        assert_eq!(user, deserialized);
    }

    #[test]
    fn test_api_response() {
        let response = ApiResponse::success("test data");
        assert!(response.success);
        assert_eq!(response.data, Some("test data"));
        assert!(response.error.is_none());

        let error_response: ApiResponse<String> = ApiResponse::error("test error".to_string());
        assert!(!error_response.success);
        assert!(error_response.data.is_none());
        assert_eq!(error_response.error, Some("test error".to_string()));
    }
}
