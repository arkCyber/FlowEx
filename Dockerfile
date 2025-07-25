# FlowEx Enterprise Trading Platform - Multi-stage Docker Build
# Optimized for production deployment with security and performance

# Build stage
FROM rust:1.75-slim as builder

# Install system dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libpq-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Create app user
RUN useradd -m -u 1001 flowex

# Set working directory
WORKDIR /app

# Copy dependency files first for better caching
COPY Cargo.toml Cargo.lock ./
COPY backend/services/*/Cargo.toml ./backend/services/
COPY backend/shared/*/Cargo.toml ./backend/shared/

# Create dummy source files to build dependencies
RUN mkdir -p backend/services/auth-service/src \
    backend/services/trading-service/src \
    backend/services/market-data-service/src \
    backend/services/wallet-service/src \
    backend/shared/types/src \
    backend/shared/database/src \
    backend/shared/error-handling/src \
    backend/shared/metrics/src \
    backend/shared/middleware/src \
    backend/shared/config/src \
    backend/shared/cache/src

# Create dummy main.rs files
RUN echo "fn main() {}" > backend/services/auth-service/src/main.rs && \
    echo "fn main() {}" > backend/services/trading-service/src/main.rs && \
    echo "fn main() {}" > backend/services/market-data-service/src/main.rs && \
    echo "fn main() {}" > backend/services/wallet-service/src/main.rs

# Create dummy lib.rs files for shared crates
RUN echo "" > backend/shared/types/src/lib.rs && \
    echo "" > backend/shared/database/src/lib.rs && \
    echo "" > backend/shared/error-handling/src/lib.rs && \
    echo "" > backend/shared/metrics/src/lib.rs && \
    echo "" > backend/shared/middleware/src/lib.rs && \
    echo "" > backend/shared/config/src/lib.rs && \
    echo "" > backend/shared/cache/src/lib.rs

# Build dependencies (this layer will be cached)
RUN cargo build --release

# Remove dummy files
RUN rm -rf backend/services/*/src backend/shared/*/src

# Copy actual source code
COPY backend/ ./backend/
COPY migrations/ ./migrations/

# Build the actual application
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    libpq5 \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Create app user
RUN useradd -m -u 1001 flowex

# Create necessary directories
RUN mkdir -p /app/bin /app/config /app/logs /app/migrations && \
    chown -R flowex:flowex /app

# Copy binaries from builder
COPY --from=builder /app/target/release/auth-service /app/bin/
COPY --from=builder /app/target/release/trading-service /app/bin/
COPY --from=builder /app/target/release/market-data-service /app/bin/
COPY --from=builder /app/target/release/wallet-service /app/bin/

# Copy migrations
COPY --from=builder /app/migrations/ /app/migrations/

# Copy configuration files
COPY docker/config/ /app/config/

# Set permissions
RUN chmod +x /app/bin/* && \
    chown -R flowex:flowex /app

# Switch to non-root user
USER flowex

# Set working directory
WORKDIR /app

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:8001/health || exit 1

# Default environment variables
ENV RUST_LOG=info
ENV FLOWEX_HOST=0.0.0.0
ENV FLOWEX_DATABASE_URL=postgresql://flowex:password@postgres:5432/flowex
ENV FLOWEX_REDIS_URL=redis://redis:6379
ENV FLOWEX_JWT_SECRET=flowex_enterprise_secret_key_2024

# Expose ports
EXPOSE 8001 8002 8003 8004

# Default command (can be overridden)
CMD ["/app/bin/auth-service"]
