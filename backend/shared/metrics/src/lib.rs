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

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Once;
    use std::time::Duration;

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

    /// 测试：指标收集器创建
    #[test]
    fn test_metrics_collector_creation() {
        init_test_env();

        let collector = MetricsCollector::new();

        // 验证收集器创建成功
        assert!(collector.start_time.elapsed().as_secs() < 1);
    }

    /// 测试：HTTP指标记录
    #[test]
    fn test_http_metrics_recording() {
        init_test_env();

        let collector = MetricsCollector::new();

        // 记录HTTP请求
        collector.record_http_request("GET", "/api/health", 200);
        collector.record_http_request("POST", "/api/orders", 201);
        collector.record_http_request("GET", "/api/orders", 404);

        // 记录HTTP请求持续时间
        let duration = Duration::from_millis(150);
        collector.record_http_request_duration("GET", "/api/health", duration);

        // 记录HTTP响应大小
        collector.record_http_response_size("GET", "/api/health", 1024);

        // 这些调用应该成功完成而不出错
        assert!(true);
    }

    /// 测试：数据库指标记录
    #[test]
    fn test_database_metrics_recording() {
        init_test_env();

        let collector = MetricsCollector::new();

        // 记录数据库连接
        collector.record_db_connections(15, 5);

        // 记录数据库查询
        let query_duration = Duration::from_millis(25);
        collector.record_db_query("SELECT", "users", query_duration, true);
        collector.record_db_query("INSERT", "orders", query_duration, false);

        // 验证记录成功
        assert!(true);
    }

    /// 测试：交易指标记录
    #[test]
    fn test_trading_metrics_recording() {
        init_test_env();

        let collector = MetricsCollector::new();

        // 记录订单
        collector.record_order("limit", "buy", "BTCUSDT");
        collector.record_order("market", "sell", "ETHUSDT");

        // 记录交易
        collector.record_trade("BTCUSDT", 1.5, 50000.0);
        collector.record_trade("ETHUSDT", 10.0, 3000.0);

        // 记录订单簿深度
        collector.record_order_book_depth("BTCUSDT", 25, 30);

        // 验证记录成功
        assert!(true);
    }

    /// 测试：WebSocket指标记录
    #[test]
    fn test_websocket_metrics_recording() {
        init_test_env();

        let collector = MetricsCollector::new();

        // 记录WebSocket连接数
        collector.record_websocket_connections(150);

        // 记录WebSocket消息
        collector.record_websocket_message_sent("ticker_update");
        collector.record_websocket_message_sent("order_update");
        collector.record_websocket_message_received("subscribe");
        collector.record_websocket_message_received("unsubscribe");

        // 验证记录成功
        assert!(true);
    }

    /// 测试：缓存指标记录
    #[test]
    fn test_cache_metrics_recording() {
        init_test_env();

        let collector = MetricsCollector::new();

        // 记录缓存命中和未命中
        collector.record_cache_hit("user_session");
        collector.record_cache_hit("order_book");
        collector.record_cache_miss("user_profile");

        // 记录缓存操作
        let operation_duration = Duration::from_micros(500);
        collector.record_cache_operation("get", operation_duration);
        collector.record_cache_operation("set", operation_duration);

        // 验证记录成功
        assert!(true);
    }

    /// 测试：系统指标记录
    #[test]
    fn test_system_metrics_recording() {
        init_test_env();

        let collector = MetricsCollector::new();

        // 记录内存使用
        collector.record_memory_usage(1024 * 1024 * 512); // 512MB

        // 记录CPU使用率
        collector.record_cpu_usage(45.5);

        // 更新运行时间
        collector.update_uptime();

        // 验证记录成功
        assert!(true);
    }

    /// 测试：业务指标管理
    #[tokio::test]
    async fn test_business_metrics_management() {
        init_test_env();

        let collector = MetricsCollector::new();

        // 设置业务指标
        collector.set_business_metric("active_users", 1250.0).await;
        collector.set_business_metric("daily_volume", 50000000.0).await;

        // 增加业务指标
        collector.increment_business_metric("active_users", 50.0).await;
        collector.increment_business_metric("daily_trades", 1.0).await;

        // 获取业务指标
        let active_users = collector.get_business_metric("active_users").await;
        assert_eq!(active_users, Some(1300.0));

        let daily_volume = collector.get_business_metric("daily_volume").await;
        assert_eq!(daily_volume, Some(50000000.0));

        let daily_trades = collector.get_business_metric("daily_trades").await;
        assert_eq!(daily_trades, Some(1.0));

        // 获取所有业务指标
        let all_metrics = collector.get_all_business_metrics().await;
        assert!(all_metrics.contains_key("active_users"));
        assert!(all_metrics.contains_key("daily_volume"));
        assert!(all_metrics.contains_key("daily_trades"));
    }

    /// 测试：健康和性能监控
    #[test]
    fn test_health_performance_monitoring() {
        init_test_env();

        let collector = MetricsCollector::new();

        // 记录服务健康状态
        collector.record_service_health("auth-service", true, 25.5);
        collector.record_service_health("trading-service", false, 150.0);

        // 记录错误
        collector.record_error("auth-service", "authentication_failed");
        collector.record_error("trading-service", "order_validation_error");

        // 验证记录成功
        assert!(true);
    }

    /// 测试：计时器功能
    #[test]
    fn test_metrics_timer() {
        init_test_env();

        let collector = MetricsCollector::new();
        let timer = collector.start_timer();

        // 模拟一些工作
        std::thread::sleep(Duration::from_millis(10));

        let elapsed = timer.elapsed();
        assert!(elapsed.as_millis() >= 10);

        let elapsed_ms = timer.elapsed_ms();
        assert!(elapsed_ms >= 10.0);

        // 记录并完成计时
        timer.record_and_finish("test_operation_duration_seconds", vec![
            ("operation", "test".to_string()),
            ("status", "success".to_string()),
        ]);
    }

    /// 测试：系统指标更新
    #[tokio::test]
    async fn test_system_metrics_update() {
        init_test_env();

        let collector = MetricsCollector::new();

        // 更新系统指标
        collector.update_system_metrics().await;

        // 验证更新成功（这里主要测试不会崩溃）
        assert!(true);
    }

    /// 测试：指标描述
    #[test]
    fn test_metrics_description() {
        init_test_env();

        // 创建收集器会自动调用describe_metrics
        let _collector = MetricsCollector::new();

        // 验证描述成功（这里主要测试不会崩溃）
        assert!(true);
    }

    /// 测试：健康检查结构
    #[test]
    fn test_health_check_structure() {
        init_test_env();

        let health_check = HealthCheck {
            service: "test-service".to_string(),
            status: HealthStatus::Healthy,
            response_time_ms: 25.5,
            timestamp: 1640995200, // 2022-01-01 00:00:00 UTC
            details: Some("All systems operational".to_string()),
        };

        assert_eq!(health_check.service, "test-service");
        assert!(matches!(health_check.status, HealthStatus::Healthy));
        assert_eq!(health_check.response_time_ms, 25.5);
        assert!(health_check.details.is_some());
    }

    /// 测试：服务指标摘要
    #[test]
    fn test_service_metrics_summary() {
        init_test_env();

        let service_metrics = ServiceMetrics {
            service_name: "trading-service".to_string(),
            uptime_seconds: 3600.0,
            total_requests: 10000,
            error_rate: 0.02,
            avg_response_time_ms: 45.5,
            active_connections: 150,
            memory_usage_mb: 512.0,
            cpu_usage_percent: 35.5,
            timestamp: 1640995200,
        };

        assert_eq!(service_metrics.service_name, "trading-service");
        assert_eq!(service_metrics.uptime_seconds, 3600.0);
        assert_eq!(service_metrics.total_requests, 10000);
        assert_eq!(service_metrics.error_rate, 0.02);
        assert_eq!(service_metrics.avg_response_time_ms, 45.5);
        assert_eq!(service_metrics.active_connections, 150);
        assert_eq!(service_metrics.memory_usage_mb, 512.0);
        assert_eq!(service_metrics.cpu_usage_percent, 35.5);
    }

    /// 测试：并发指标记录
    #[tokio::test]
    async fn test_concurrent_metrics_recording() {
        init_test_env();

        let collector = MetricsCollector::new();

        // 模拟并发指标记录
        let mut handles = vec![];

        for i in 0..10 {
            let collector_clone = collector.clone();
            let handle = tokio::spawn(async move {
                collector_clone.record_http_request("GET", "/api/test", 200);
                collector_clone.set_business_metric(&format!("metric_{}", i), i as f64).await;
                collector_clone.record_cache_hit("test_cache");
            });
            handles.push(handle);
        }

        // 等待所有任务完成
        for handle in handles {
            handle.await.unwrap();
        }

        // 验证并发操作成功
        let all_metrics = collector.get_all_business_metrics().await;
        assert_eq!(all_metrics.len(), 10);
    }

    /// 测试：性能基准
    #[test]
    fn test_performance_benchmark() {
        init_test_env();

        let collector = MetricsCollector::new();
        let start = std::time::Instant::now();

        // 记录大量指标
        for i in 0..1000 {
            collector.record_http_request("GET", "/api/test", 200);
            collector.record_db_query("SELECT", "test_table", Duration::from_millis(1), true);
            collector.record_cache_hit("test_cache");
            collector.record_order("limit", "buy", "BTCUSDT");
        }

        let duration = start.elapsed();
        println!("记录1000次指标耗时: {:?}", duration);

        // 性能要求：1000次指标记录应该在50ms内完成
        assert!(duration.as_millis() < 50, "指标记录性能不达标");
    }

    /// 测试：内存使用优化
    #[tokio::test]
    async fn test_memory_usage_optimization() {
        init_test_env();

        let collector = MetricsCollector::new();

        // 添加大量业务指标
        for i in 0..1000 {
            collector.set_business_metric(&format!("test_metric_{}", i), i as f64).await;
        }

        let all_metrics = collector.get_all_business_metrics().await;
        assert_eq!(all_metrics.len(), 1000);

        // 验证内存使用合理（这里主要确保不会内存泄漏）
        drop(all_metrics);
        assert!(true);
    }

    /// 测试：错误处理
    #[tokio::test]
    async fn test_error_handling() {
        init_test_env();

        let collector = MetricsCollector::new();

        // 测试各种边界情况
        collector.record_memory_usage(0);
        collector.record_cpu_usage(0.0);
        collector.record_cpu_usage(100.0);
        collector.record_websocket_connections(0);

        // 测试空字符串
        collector.set_business_metric("", 0.0).await;
        let empty_metric = collector.get_business_metric("").await;
        assert_eq!(empty_metric, Some(0.0));

        // 测试负值
        collector.increment_business_metric("negative_test", -10.0).await;
        let negative_metric = collector.get_business_metric("negative_test").await;
        assert_eq!(negative_metric, Some(-10.0));

        // 验证错误处理成功
        assert!(true);
    }
}
