//! FlowEx Metrics Library
//!
//! Metrics collection and export for FlowEx services.

use metrics::{counter, gauge, histogram};
use std::time::Instant;

/// Metrics recorder for FlowEx services
pub struct MetricsRecorder {
    start_time: Instant,
}

impl MetricsRecorder {
    pub fn new() -> Self {
        Self {
            start_time: Instant::now(),
        }
    }
    
    pub fn record_request(&self, method: &str, path: &str, status: u16) {
        let method_owned = method.to_string();
        let path_owned = path.to_string();
        let status_str = status.to_string();
        counter!("http_requests_total", "method" => method_owned, "path" => path_owned, "status" => status_str).increment(1);
    }

    pub fn record_response_time(&self, method: &str, path: &str, duration_ms: f64) {
        let method_owned = method.to_string();
        let path_owned = path.to_string();
        histogram!("http_request_duration_ms", "method" => method_owned, "path" => path_owned).record(duration_ms);
    }
    
    pub fn record_active_connections(&self, count: u64) {
        gauge!("active_connections").set(count as f64);
    }
}

impl Default for MetricsRecorder {
    fn default() -> Self {
        Self::new()
    }
}
