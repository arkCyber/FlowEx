-- FlowEx Initial Database Schema
-- Version: 001
-- Description: Create core tables for users, trading, and wallet functionality

-- Enable UUID extension
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Users table
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    first_name VARCHAR(100) NOT NULL,
    last_name VARCHAR(100) NOT NULL,
    is_verified BOOLEAN DEFAULT FALSE,
    is_active BOOLEAN DEFAULT TRUE,
    role VARCHAR(50) DEFAULT 'user',
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- User sessions for JWT token management
CREATE TABLE user_sessions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    token_id VARCHAR(255) UNIQUE NOT NULL,
    expires_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    last_used_at TIMESTAMPTZ DEFAULT NOW(),
    ip_address INET,
    user_agent TEXT
);

-- Trading pairs
CREATE TABLE trading_pairs (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    symbol VARCHAR(20) UNIQUE NOT NULL,
    base_asset VARCHAR(10) NOT NULL,
    quote_asset VARCHAR(10) NOT NULL,
    status VARCHAR(20) DEFAULT 'TRADING',
    min_price DECIMAL(20,8) NOT NULL DEFAULT 0.00000001,
    max_price DECIMAL(20,8) NOT NULL DEFAULT 999999999.99999999,
    min_qty DECIMAL(20,8) NOT NULL DEFAULT 0.00000001,
    max_qty DECIMAL(20,8) NOT NULL DEFAULT 999999999.99999999,
    step_size DECIMAL(20,8) NOT NULL DEFAULT 0.00000001,
    tick_size DECIMAL(20,8) NOT NULL DEFAULT 0.00000001,
    maker_fee DECIMAL(5,4) DEFAULT 0.001,
    taker_fee DECIMAL(5,4) DEFAULT 0.001,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Orders table
CREATE TABLE orders (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    trading_pair VARCHAR(20) NOT NULL,
    side VARCHAR(10) NOT NULL CHECK (side IN ('buy', 'sell')),
    order_type VARCHAR(20) NOT NULL CHECK (order_type IN ('market', 'limit', 'stop_loss', 'take_profit')),
    price DECIMAL(20,8),
    quantity DECIMAL(20,8) NOT NULL,
    filled_quantity DECIMAL(20,8) DEFAULT 0,
    remaining_quantity DECIMAL(20,8) NOT NULL,
    status VARCHAR(20) DEFAULT 'NEW' CHECK (status IN ('NEW', 'PARTIALLY_FILLED', 'FILLED', 'CANCELLED', 'REJECTED', 'EXPIRED')),
    time_in_force VARCHAR(10) DEFAULT 'GTC' CHECK (time_in_force IN ('GTC', 'IOC', 'FOK')),
    stop_price DECIMAL(20,8),
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    expires_at TIMESTAMPTZ
);

-- Order fills/executions
CREATE TABLE order_fills (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    order_id UUID NOT NULL REFERENCES orders(id) ON DELETE CASCADE,
    trade_id UUID NOT NULL,
    price DECIMAL(20,8) NOT NULL,
    quantity DECIMAL(20,8) NOT NULL,
    fee DECIMAL(20,8) NOT NULL DEFAULT 0,
    fee_currency VARCHAR(10) NOT NULL,
    is_maker BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Trades table
CREATE TABLE trades (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    symbol VARCHAR(20) NOT NULL,
    buyer_order_id UUID NOT NULL REFERENCES orders(id),
    seller_order_id UUID NOT NULL REFERENCES orders(id),
    price DECIMAL(20,8) NOT NULL,
    quantity DECIMAL(20,8) NOT NULL,
    buyer_fee DECIMAL(20,8) NOT NULL DEFAULT 0,
    seller_fee DECIMAL(20,8) NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- User balances
CREATE TABLE balances (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    currency VARCHAR(10) NOT NULL,
    available DECIMAL(20,8) NOT NULL DEFAULT 0,
    locked DECIMAL(20,8) NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(user_id, currency)
);

-- Transactions (deposits, withdrawals, trades)
CREATE TABLE transactions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    transaction_type VARCHAR(20) NOT NULL CHECK (transaction_type IN ('deposit', 'withdrawal', 'trade', 'fee')),
    currency VARCHAR(10) NOT NULL,
    amount DECIMAL(20,8) NOT NULL,
    status VARCHAR(20) DEFAULT 'pending' CHECK (status IN ('pending', 'completed', 'failed', 'cancelled')),
    reference_id UUID, -- Can reference order_id, trade_id, etc.
    external_id VARCHAR(255), -- For external transaction tracking
    fee DECIMAL(20,8) DEFAULT 0,
    notes TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Market data - tickers
CREATE TABLE tickers (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    symbol VARCHAR(20) NOT NULL,
    price DECIMAL(20,8) NOT NULL,
    change_24h DECIMAL(20,8) NOT NULL DEFAULT 0,
    change_percent_24h DECIMAL(8,4) NOT NULL DEFAULT 0,
    high_24h DECIMAL(20,8) NOT NULL,
    low_24h DECIMAL(20,8) NOT NULL,
    volume_24h DECIMAL(20,8) NOT NULL DEFAULT 0,
    volume_quote_24h DECIMAL(20,8) NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Create indexes for performance
CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_users_created_at ON users(created_at);

CREATE INDEX idx_user_sessions_user_id ON user_sessions(user_id);
CREATE INDEX idx_user_sessions_token_id ON user_sessions(token_id);
CREATE INDEX idx_user_sessions_expires_at ON user_sessions(expires_at);

CREATE INDEX idx_trading_pairs_symbol ON trading_pairs(symbol);
CREATE INDEX idx_trading_pairs_status ON trading_pairs(status);

CREATE INDEX idx_orders_user_id ON orders(user_id);
CREATE INDEX idx_orders_trading_pair ON orders(trading_pair);
CREATE INDEX idx_orders_status ON orders(status);
CREATE INDEX idx_orders_created_at ON orders(created_at);
CREATE INDEX idx_orders_side_status ON orders(side, status);

CREATE INDEX idx_order_fills_order_id ON order_fills(order_id);
CREATE INDEX idx_order_fills_trade_id ON order_fills(trade_id);
CREATE INDEX idx_order_fills_created_at ON order_fills(created_at);

CREATE INDEX idx_trades_symbol ON trades(symbol);
CREATE INDEX idx_trades_created_at ON trades(created_at);
CREATE INDEX idx_trades_buyer_order_id ON trades(buyer_order_id);
CREATE INDEX idx_trades_seller_order_id ON trades(seller_order_id);

CREATE INDEX idx_balances_user_id ON balances(user_id);
CREATE INDEX idx_balances_currency ON balances(currency);
CREATE INDEX idx_balances_user_currency ON balances(user_id, currency);

CREATE INDEX idx_transactions_user_id ON transactions(user_id);
CREATE INDEX idx_transactions_type ON transactions(transaction_type);
CREATE INDEX idx_transactions_status ON transactions(status);
CREATE INDEX idx_transactions_created_at ON transactions(created_at);
CREATE INDEX idx_transactions_currency ON transactions(currency);

CREATE INDEX idx_tickers_symbol ON tickers(symbol);
CREATE INDEX idx_tickers_created_at ON tickers(created_at);

-- Insert default trading pairs
INSERT INTO trading_pairs (symbol, base_asset, quote_asset, status) VALUES
('BTCUSDT', 'BTC', 'USDT', 'TRADING'),
('ETHUSDT', 'ETH', 'USDT', 'TRADING'),
('BNBUSDT', 'BNB', 'USDT', 'TRADING'),
('ADAUSDT', 'ADA', 'USDT', 'TRADING'),
('DOTUSDT', 'DOT', 'USDT', 'TRADING'),
('LINKUSDT', 'LINK', 'USDT', 'TRADING'),
('LTCUSDT', 'LTC', 'USDT', 'TRADING'),
('BCHUSDT', 'BCH', 'USDT', 'TRADING'),
('XLMUSDT', 'XLM', 'USDT', 'TRADING'),
('EOSUSDT', 'EOS', 'USDT', 'TRADING');

-- Insert demo user (password: demo123 - hashed with bcrypt)
INSERT INTO users (id, email, password_hash, first_name, last_name, is_verified, role) VALUES
('550e8400-e29b-41d4-a716-446655440000', 'demo@flowex.com', '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewdBPj/RK.PJ/..G', 'Demo', 'User', true, 'trader');

-- Insert demo balances
INSERT INTO balances (user_id, currency, available, locked) VALUES
('550e8400-e29b-41d4-a716-446655440000', 'BTC', 0.12345678, 0),
('550e8400-e29b-41d4-a716-446655440000', 'ETH', 2.45678901, 0.1),
('550e8400-e29b-41d4-a716-446655440000', 'USDT', 1000.00000000, 50.00000000),
('550e8400-e29b-41d4-a716-446655440000', 'BNB', 10.50000000, 0),
('550e8400-e29b-41d4-a716-446655440000', 'ADA', 500.00000000, 0),
('550e8400-e29b-41d4-a716-446655440000', 'DOT', 25.75000000, 0);

-- Insert sample market data
INSERT INTO tickers (symbol, price, change_24h, change_percent_24h, high_24h, low_24h, volume_24h) VALUES
('BTCUSDT', 43250.50, 1250.30, 2.98, 44100.00, 42000.00, 15420.75),
('ETHUSDT', 2650.75, -85.25, -3.11, 2750.00, 2580.00, 8750.50),
('BNBUSDT', 315.80, 12.45, 4.10, 320.00, 300.00, 2150.25),
('ADAUSDT', 0.4850, 0.0125, 2.64, 0.4950, 0.4700, 125000.00),
('DOTUSDT', 7.25, -0.35, -4.61, 7.80, 7.10, 5500.75);
