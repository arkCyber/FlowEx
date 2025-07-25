//! FlowEx Metrics Library
//!
//! Comprehensive metrics collection and monitoring for FlowEx services.
//! Provides Prometheus-compatible metrics, custom business metrics, and health monitoring.

use metrics::{counter, gauge, histogram, describe_counter, describe_gauge, describe_histogram};
use std::time::{Instant, Duration, SystemTime, UNIX_EPOCH};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Serialize, Deserialize};
use tracing::{info, error, debug};

/// Enterprise metrics collector for FlowEx services
#[derive(Clone)]
pub struct MetricsCollector {
    start_time: Instant,
    business_metrics: Arc<RwLock<HashMap<String, f64>>>,
}

impl MetricsCollector {
    /// Create a new metrics collector
    pub fn new() -> Self {
        // Describe all metrics for Prometheus
        Self::describe_metrics();

        Self {
            start_time: Instant::now(),
            business_metrics: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Describe all metrics for Prometheus compatibility
    fn describe_metrics() {
        // HTTP metrics
        describe_counter!("flowex_http_requests_total", "Total number of HTTP requests");
        describe_histogram!("flowex_http_request_duration_seconds", "HTTP request duration in seconds");
        describe_histogram!("flowex_http_response_size_bytes", "HTTP response size in bytes");

        // Database metrics
        describe_gauge!("flowex_db_connections_active", "Number of active database connections");
        describe_gauge!("flowex_db_connections_idle", "Number of idle database connections");
        describe_histogram!("flowex_db_query_duration_seconds", "Database query duration in seconds");
        describe_counter!("flowex_db_queries_total", "Total number of database queries");

        // Trading metrics
        describe_counter!("flowex_orders_total", "Total number of orders");
        describe_counter!("flowex_trades_total", "Total number of trades");
        describe_counter!("flowex_trade_volume_total", "Total trading volume");
        describe_gauge!("flowex_order_book_depth", "Order book depth");

        // WebSocket metrics
        describe_gauge!("flowex_websocket_connections", "Number of active WebSocket connections");
        describe_counter!("flowex_websocket_messages_sent_total", "Total WebSocket messages sent");
        describe_counter!("flowex_websocket_messages_received_total", "Total WebSocket messages received");

        // Cache metrics
        describe_counter!("flowex_cache_hits_total", "Total cache hits");
        describe_counter!("flowex_cache_misses_total", "Total cache misses");
        describe_histogram!("flowex_cache_operation_duration_seconds", "Cache operation duration");

        // System metrics
        describe_gauge!("flowex_memory_usage_bytes", "Memory usage in bytes");
        describe_gauge!("flowex_cpu_usage_percent", "CPU usage percentage");
        describe_gauge!("flowex_uptime_seconds", "Service uptime in seconds");
    }

    // HTTP Metrics
    pub fn record_http_request(&self, method: &str, endpoint: &str, status: u16) {
        counter!("flowex_http_requests_total",
                "method" => method.to_string(),
                "endpoint" => endpoint.to_string(),
                "status" => status.to_string())
            .increment(1);
    }

    pub fn record_http_request_duration(&self, method: &str, endpoint: &str, duration: Duration) {
        histogram!("flowex_http_request_duration_seconds",
                  "method" => method.to_string(),
                  "endpoint" => endpoint.to_string())
            .record(duration.as_secs_f64());
    }

    pub fn record_http_response_size(&self, method: &str, endpoint: &str, size_bytes: u64) {
        histogram!("flowex_http_response_size_bytes",
                  "method" => method.to_string(),
                  "endpoint" => endpoint.to_string())
            .record(size_bytes as f64);
    }

    // Database Metrics
    pub fn record_db_connections(&self, active: u32, idle: u32) {
        gauge!("flowex_db_connections_active").set(active as f64);
        gauge!("flowex_db_connections_idle").set(idle as f64);
    }

    pub fn record_db_query(&self, query_type: &str, table: &str, duration: Duration, success: bool) {
        histogram!("flowex_db_query_duration_seconds",
                  "query_type" => query_type.to_string(),
                  "table" => table.to_string())
            .record(duration.as_secs_f64());

        counter!("flowex_db_queries_total",
                "query_type" => query_type.to_string(),
                "table" => table.to_string(),
                "status" => if success { "success" } else { "error" }.to_string())
            .increment(1);
    }

    // Trading Metrics
    pub fn record_order(&self, order_type: &str, side: &str, symbol: &str) {
        counter!("flowex_orders_total",
                "type" => order_type.to_string(),
                "side" => side.to_string(),
                "symbol" => symbol.to_string())
            .increment(1);
    }

    pub fn record_trade(&self, symbol: &str, volume: f64, price: f64) {
        counter!("flowex_trades_total", "symbol" => symbol.to_string()).increment(1);
        counter!("flowex_trade_volume_total", "symbol" => symbol.to_string()).increment(volume);
    }

    pub fn record_order_book_depth(&self, symbol: &str, bid_depth: u32, ask_depth: u32) {
        gauge!("flowex_order_book_depth",
               "symbol" => symbol.to_string(),
               "side" => "bid".to_string())
            .set(bid_depth as f64);
        gauge!("flowex_order_book_depth",
               "symbol" => symbol.to_string(),
               "side" => "ask".to_string())
            .set(ask_depth as f64);
    }

    // WebSocket Metrics
    pub fn record_websocket_connections(&self, count: u32) {
        gauge!("flowex_websocket_connections").set(count as f64);
    }

    pub fn record_websocket_message_sent(&self, message_type: &str) {
        counter!("flowex_websocket_messages_sent_total",
                "type" => message_type.to_string())
            .increment(1);
    }

    pub fn record_websocket_message_received(&self, message_type: &str) {
        counter!("flowex_websocket_messages_received_total",
                "type" => message_type.to_string())
            .increment(1);
    }

    // Cache Metrics
    pub fn record_cache_hit(&self, cache_type: &str) {
        counter!("flowex_cache_hits_total", "type" => cache_type.to_string()).increment(1);
    }

    pub fn record_cache_miss(&self, cache_type: &str) {
        counter!("flowex_cache_misses_total", "type" => cache_type.to_string()).increment(1);
    }

    pub fn record_cache_operation(&self, operation: &str, duration: Duration) {
        histogram!("flowex_cache_operation_duration_seconds",
                  "operation" => operation.to_string())
            .record(duration.as_secs_f64());
    }

    // System Metrics
    pub fn record_memory_usage(&self, bytes: u64) {
        gauge!("flowex_memory_usage_bytes").set(bytes as f64);
    }

    pub fn record_cpu_usage(&self, percent: f64) {
        gauge!("flowex_cpu_usage_percent").set(percent);
    }

    pub fn update_uptime(&self) {
        let uptime = self.start_time.elapsed().as_secs() as f64;
        gauge!("flowex_uptime_seconds").set(uptime);
    }
}

    // Business Metrics
    pub async fn set_business_metric(&self, name: &str, value: f64) {
        let mut metrics = self.business_metrics.write().await;
        metrics.insert(name.to_string(), value);
        gauge!("flowex_business_metric", "name" => name.to_string()).set(value);
        debug!("Set business metric: {} = {}", name, value);
    }

    pub async fn increment_business_metric(&self, name: &str, delta: f64) {
        let mut metrics = self.business_metrics.write().await;
        let current = metrics.get(name).unwrap_or(&0.0);
        let new_value = current + delta;
        metrics.insert(name.to_string(), new_value);
        gauge!("flowex_business_metric", "name" => name.to_string()).set(new_value);
        debug!("Incremented business metric: {} by {} = {}", name, delta, new_value);
    }

    pub async fn get_business_metric(&self, name: &str) -> Option<f64> {
        let metrics = self.business_metrics.read().await;
        metrics.get(name).copied()
    }

    pub async fn get_all_business_metrics(&self) -> HashMap<String, f64> {
        let metrics = self.business_metrics.read().await;
        metrics.clone()
    }

    // Health and Performance Monitoring
    pub fn record_service_health(&self, service: &str, healthy: bool, response_time_ms: f64) {
        gauge!("flowex_service_health", "service" => service.to_string())
            .set(if healthy { 1.0 } else { 0.0 });
        histogram!("flowex_service_response_time_seconds", "service" => service.to_string())
            .record(response_time_ms / 1000.0);
    }

    pub fn record_error(&self, service: &str, error_type: &str) {
        counter!("flowex_errors_total",
                "service" => service.to_string(),
                "type" => error_type.to_string())
            .increment(1);
    }

    // Performance timing helper
    pub fn start_timer(&self) -> MetricsTimer {
        MetricsTimer::new()
    }

    // Batch metrics update for efficiency
    pub async fn update_system_metrics(&self) {
        self.update_uptime();

        // Update memory usage (simplified - in production use proper system metrics)
        if let Ok(memory) = self.get_memory_usage() {
            self.record_memory_usage(memory);
        }

        // Update CPU usage (simplified - in production use proper system metrics)
        if let Ok(cpu) = self.get_cpu_usage() {
            self.record_cpu_usage(cpu);
        }
    }

    // Helper methods for system metrics (simplified implementations)
    fn get_memory_usage(&self) -> Result<u64, std::io::Error> {
        // In production, use proper system metrics library like sysinfo
        // This is a placeholder implementation
        Ok(1024 * 1024 * 100) // 100MB placeholder
    }

    fn get_cpu_usage(&self) -> Result<f64, std::io::Error> {
        // In production, use proper system metrics library like sysinfo
        // This is a placeholder implementation
        Ok(25.0) // 25% placeholder
    }
}

/// Timer for measuring operation duration
pub struct MetricsTimer {
    start: Instant,
}

impl MetricsTimer {
    pub fn new() -> Self {
        Self {
            start: Instant::now(),
        }
    }

    pub fn elapsed(&self) -> Duration {
        self.start.elapsed()
    }

    pub fn elapsed_ms(&self) -> f64 {
        self.start.elapsed().as_secs_f64() * 1000.0
    }

    pub fn record_and_finish(self, metric_name: &str, labels: Vec<(&str, String)>) {
        let duration = self.elapsed();
        let mut histogram_builder = histogram!(metric_name);

        for (key, value) in labels {
            histogram_builder = histogram_builder.with_label(key, value);
        }

        histogram_builder.record(duration.as_secs_f64());
    }
}

/// Health check result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheck {
    pub service: String,
    pub status: HealthStatus,
    pub response_time_ms: f64,
    pub timestamp: u64,
    pub details: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HealthStatus {
    Healthy,
    Degraded,
    Unhealthy,
}

/// Service metrics summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceMetrics {
    pub service_name: String,
    pub uptime_seconds: f64,
    pub total_requests: u64,
    pub error_rate: f64,
    pub avg_response_time_ms: f64,
    pub active_connections: u32,
    pub memory_usage_mb: f64,
    pub cpu_usage_percent: f64,
    pub timestamp: u64,
}

impl Default for MetricsCollector {
    fn default() -> Self {
        Self::new()
    }
}

// Legacy compatibility
pub type MetricsRecorder = MetricsCollector;
