-- FlowEx Initial Database Schema
-- Migration: 001_initial_schema
-- Created: 2024-07-24
-- Description: Initial database schema for FlowEx trading platform

-- Enable required extensions
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS "pgcrypto";

-- Users table
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    first_name VARCHAR(100) NOT NULL,
    last_name VARCHAR(100) NOT NULL,
    is_verified BOOLEAN DEFAULT FALSE,
    is_active BOOLEAN DEFAULT TRUE,
    two_factor_enabled BOOLEAN DEFAULT FALSE,
    two_factor_secret VARCHAR(32),
    last_login_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- User sessions table
CREATE TABLE user_sessions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    token_hash VARCHAR(255) NOT NULL,
    expires_at TIMESTAMPTZ NOT NULL,
    ip_address INET,
    user_agent TEXT,
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Trading pairs table
CREATE TABLE trading_pairs (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    symbol VARCHAR(20) UNIQUE NOT NULL,
    base_currency VARCHAR(10) NOT NULL,
    quote_currency VARCHAR(10) NOT NULL,
    min_order_size DECIMAL(20,8) NOT NULL,
    max_order_size DECIMAL(20,8) NOT NULL,
    price_precision INTEGER NOT NULL DEFAULT 8,
    quantity_precision INTEGER NOT NULL DEFAULT 8,
    status VARCHAR(20) DEFAULT 'active',
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Orders table
CREATE TABLE orders (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id),
    trading_pair_id UUID NOT NULL REFERENCES trading_pairs(id),
    order_type VARCHAR(20) NOT NULL, -- market, limit, stop_loss, take_profit
    side VARCHAR(10) NOT NULL, -- buy, sell
    status VARCHAR(20) NOT NULL DEFAULT 'new', -- new, partially_filled, filled, cancelled, rejected, expired
    price DECIMAL(20,8),
    quantity DECIMAL(20,8) NOT NULL,
    filled_quantity DECIMAL(20,8) DEFAULT 0,
    remaining_quantity DECIMAL(20,8) NOT NULL,
    average_price DECIMAL(20,8),
    total_value DECIMAL(20,8),
    fee DECIMAL(20,8) DEFAULT 0,
    fee_currency VARCHAR(10),
    time_in_force VARCHAR(20) DEFAULT 'GTC', -- GTC, IOC, FOK
    stop_price DECIMAL(20,8),
    client_order_id VARCHAR(64),
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    filled_at TIMESTAMPTZ,
    cancelled_at TIMESTAMPTZ
);

-- Trades table
CREATE TABLE trades (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    trading_pair_id UUID NOT NULL REFERENCES trading_pairs(id),
    buyer_order_id UUID NOT NULL REFERENCES orders(id),
    seller_order_id UUID NOT NULL REFERENCES orders(id),
    buyer_user_id UUID NOT NULL REFERENCES users(id),
    seller_user_id UUID NOT NULL REFERENCES users(id),
    price DECIMAL(20,8) NOT NULL,
    quantity DECIMAL(20,8) NOT NULL,
    total_value DECIMAL(20,8) NOT NULL,
    buyer_fee DECIMAL(20,8) DEFAULT 0,
    seller_fee DECIMAL(20,8) DEFAULT 0,
    fee_currency VARCHAR(10),
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Wallets table
CREATE TABLE wallets (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id),
    currency VARCHAR(10) NOT NULL,
    available_balance DECIMAL(20,8) DEFAULT 0,
    locked_balance DECIMAL(20,8) DEFAULT 0,
    total_balance DECIMAL(20,8) GENERATED ALWAYS AS (available_balance + locked_balance) STORED,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(user_id, currency)
);

-- Transactions table
CREATE TABLE transactions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id),
    wallet_id UUID NOT NULL REFERENCES wallets(id),
    transaction_type VARCHAR(20) NOT NULL, -- deposit, withdrawal, trade, fee
    currency VARCHAR(10) NOT NULL,
    amount DECIMAL(20,8) NOT NULL,
    fee DECIMAL(20,8) DEFAULT 0,
    status VARCHAR(20) NOT NULL DEFAULT 'pending', -- pending, completed, failed, cancelled
    reference_id UUID, -- Reference to order, trade, or external transaction
    external_tx_id VARCHAR(255), -- External transaction ID for deposits/withdrawals
    blockchain_confirmations INTEGER DEFAULT 0,
    required_confirmations INTEGER DEFAULT 6,
    notes TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    completed_at TIMESTAMPTZ
);

-- Market data tickers table
CREATE TABLE market_tickers (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    trading_pair_id UUID NOT NULL REFERENCES trading_pairs(id),
    symbol VARCHAR(20) NOT NULL,
    last_price DECIMAL(20,8),
    bid_price DECIMAL(20,8),
    ask_price DECIMAL(20,8),
    volume_24h DECIMAL(20,8) DEFAULT 0,
    high_24h DECIMAL(20,8),
    low_24h DECIMAL(20,8),
    price_change_24h DECIMAL(20,8) DEFAULT 0,
    price_change_percent_24h DECIMAL(10,4) DEFAULT 0,
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(trading_pair_id)
);

-- Order book levels table (for maintaining order book state)
CREATE TABLE order_book_levels (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    trading_pair_id UUID NOT NULL REFERENCES trading_pairs(id),
    side VARCHAR(10) NOT NULL, -- buy, sell
    price DECIMAL(20,8) NOT NULL,
    quantity DECIMAL(20,8) NOT NULL,
    order_count INTEGER DEFAULT 1,
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(trading_pair_id, side, price)
);

-- Audit log table
CREATE TABLE audit_logs (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID REFERENCES users(id),
    action VARCHAR(100) NOT NULL,
    resource_type VARCHAR(50) NOT NULL,
    resource_id UUID,
    old_values JSONB,
    new_values JSONB,
    ip_address INET,
    user_agent TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Create indexes for performance
CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_users_created_at ON users(created_at);

CREATE INDEX idx_user_sessions_user_id ON user_sessions(user_id);
CREATE INDEX idx_user_sessions_token_hash ON user_sessions(token_hash);
CREATE INDEX idx_user_sessions_expires_at ON user_sessions(expires_at);

CREATE INDEX idx_trading_pairs_symbol ON trading_pairs(symbol);
CREATE INDEX idx_trading_pairs_status ON trading_pairs(status);

CREATE INDEX idx_orders_user_id ON orders(user_id);
CREATE INDEX idx_orders_trading_pair_id ON orders(trading_pair_id);
CREATE INDEX idx_orders_status ON orders(status);
CREATE INDEX idx_orders_created_at ON orders(created_at);
CREATE INDEX idx_orders_client_order_id ON orders(client_order_id);

CREATE INDEX idx_trades_trading_pair_id ON trades(trading_pair_id);
CREATE INDEX idx_trades_buyer_user_id ON trades(buyer_user_id);
CREATE INDEX idx_trades_seller_user_id ON trades(seller_user_id);
CREATE INDEX idx_trades_created_at ON trades(created_at);

CREATE INDEX idx_wallets_user_id ON wallets(user_id);
CREATE INDEX idx_wallets_currency ON wallets(currency);

CREATE INDEX idx_transactions_user_id ON transactions(user_id);
CREATE INDEX idx_transactions_wallet_id ON transactions(wallet_id);
CREATE INDEX idx_transactions_type ON transactions(transaction_type);
CREATE INDEX idx_transactions_status ON transactions(status);
CREATE INDEX idx_transactions_created_at ON transactions(created_at);
CREATE INDEX idx_transactions_external_tx_id ON transactions(external_tx_id);

CREATE INDEX idx_market_tickers_symbol ON market_tickers(symbol);
CREATE INDEX idx_market_tickers_updated_at ON market_tickers(updated_at);

CREATE INDEX idx_order_book_levels_trading_pair_side ON order_book_levels(trading_pair_id, side);
CREATE INDEX idx_order_book_levels_price ON order_book_levels(price);

CREATE INDEX idx_audit_logs_user_id ON audit_logs(user_id);
CREATE INDEX idx_audit_logs_action ON audit_logs(action);
CREATE INDEX idx_audit_logs_resource_type ON audit_logs(resource_type);
CREATE INDEX idx_audit_logs_created_at ON audit_logs(created_at);

-- Create triggers for updated_at timestamps
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_users_updated_at BEFORE UPDATE ON users
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_trading_pairs_updated_at BEFORE UPDATE ON trading_pairs
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_orders_updated_at BEFORE UPDATE ON orders
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_wallets_updated_at BEFORE UPDATE ON wallets
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_transactions_updated_at BEFORE UPDATE ON transactions
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_order_book_levels_updated_at BEFORE UPDATE ON order_book_levels
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
