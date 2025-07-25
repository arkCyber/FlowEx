//! FlowEx Integration Tests Module
//!
//! Comprehensive integration test suite for all FlowEx services
//! with test orchestration and reporting capabilities.

pub mod auth_service_tests;
pub mod trading_service_tests;

use std::time::{Duration, Instant};
use tokio::time::sleep;

/// Test result summary
#[derive(Debug, Clone)]
pub struct TestResult {
    pub service: String,
    pub test_name: String,
    pub passed: bool,
    pub duration: Duration,
    pub error: Option<String>,
}

/// Test suite runner
pub struct TestSuiteRunner {
    results: Vec<TestResult>,
    start_time: Instant,
}

impl TestSuiteRunner {
    pub fn new() -> Self {
        Self {
            results: Vec::new(),
            start_time: Instant::now(),
        }
    }
    
    /// Run a single test with error handling and timing
    pub async fn run_test<F, Fut>(&mut self, service: &str, test_name: &str, test_fn: F)
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = Result<(), Box<dyn std::error::Error>>>,
    {
        println!("🧪 Running test: {} - {}", service, test_name);
        
        let start = Instant::now();
        let result = test_fn().await;
        let duration = start.elapsed();
        
        let test_result = match result {
            Ok(()) => {
                println!("✅ PASSED: {} - {} ({:?})", service, test_name, duration);
                TestResult {
                    service: service.to_string(),
                    test_name: test_name.to_string(),
                    passed: true,
                    duration,
                    error: None,
                }
            }
            Err(e) => {
                println!("❌ FAILED: {} - {} ({:?}): {}", service, test_name, duration, e);
                TestResult {
                    service: service.to_string(),
                    test_name: test_name.to_string(),
                    passed: false,
                    duration,
                    error: Some(e.to_string()),
                }
            }
        };
        
        self.results.push(test_result);
    }
    
    /// Generate test report
    pub fn generate_report(&self) -> TestReport {
        let total_tests = self.results.len();
        let passed_tests = self.results.iter().filter(|r| r.passed).count();
        let failed_tests = total_tests - passed_tests;
        let total_duration = self.start_time.elapsed();
        
        let mut services = std::collections::HashMap::new();
        for result in &self.results {
            let service_stats = services.entry(result.service.clone()).or_insert(ServiceStats {
                total: 0,
                passed: 0,
                failed: 0,
                duration: Duration::from_secs(0),
            });
            
            service_stats.total += 1;
            service_stats.duration += result.duration;
            
            if result.passed {
                service_stats.passed += 1;
            } else {
                service_stats.failed += 1;
            }
        }
        
        TestReport {
            total_tests,
            passed_tests,
            failed_tests,
            total_duration,
            services,
            results: self.results.clone(),
        }
    }
    
    /// Print summary report
    pub fn print_summary(&self) {
        let report = self.generate_report();
        
        println!("\n📊 TEST SUMMARY");
        println!("================");
        println!("Total Tests: {}", report.total_tests);
        println!("Passed: {} ✅", report.passed_tests);
        println!("Failed: {} ❌", report.failed_tests);
        println!("Success Rate: {:.1}%", 
                 if report.total_tests > 0 { 
                     (report.passed_tests as f64 / report.total_tests as f64) * 100.0 
                 } else { 
                     0.0 
                 });
        println!("Total Duration: {:?}", report.total_duration);
        
        println!("\n📋 BY SERVICE");
        println!("==============");
        for (service, stats) in &report.services {
            println!("{}: {}/{} passed ({:.1}%) - {:?}", 
                     service, 
                     stats.passed, 
                     stats.total,
                     if stats.total > 0 { 
                         (stats.passed as f64 / stats.total as f64) * 100.0 
                     } else { 
                         0.0 
                     },
                     stats.duration);
        }
        
        if report.failed_tests > 0 {
            println!("\n❌ FAILED TESTS");
            println!("================");
            for result in &report.results {
                if !result.passed {
                    println!("{} - {}: {}", 
                             result.service, 
                             result.test_name, 
                             result.error.as_ref().unwrap_or(&"Unknown error".to_string()));
                }
            }
        }
    }
}

/// Service statistics
#[derive(Debug, Clone)]
pub struct ServiceStats {
    pub total: usize,
    pub passed: usize,
    pub failed: usize,
    pub duration: Duration,
}

/// Complete test report
#[derive(Debug, Clone)]
pub struct TestReport {
    pub total_tests: usize,
    pub passed_tests: usize,
    pub failed_tests: usize,
    pub total_duration: Duration,
    pub services: std::collections::HashMap<String, ServiceStats>,
    pub results: Vec<TestResult>,
}

/// Wait for services to be ready
pub async fn wait_for_services() -> Result<(), Box<dyn std::error::Error>> {
    println!("⏳ Waiting for services to be ready...");
    
    let services = vec![
        ("auth-service", "http://localhost:8001/health"),
        ("trading-service", "http://localhost:8002/health"),
        ("market-data-service", "http://localhost:8003/health"),
        ("wallet-service", "http://localhost:8004/health"),
    ];
    
    let client = reqwest::Client::new();
    let mut ready_services = 0;
    let max_retries = 60; // 60 seconds total
    
    for retry in 0..max_retries {
        ready_services = 0;
        
        for (service_name, health_url) in &services {
            match client.get(*health_url).send().await {
                Ok(response) if response.status().is_success() => {
                    ready_services += 1;
                }
                _ => {
                    // Service not ready yet
                }
            }
        }
        
        if ready_services == services.len() {
            println!("✅ All {} services are ready!", services.len());
            return Ok(());
        }
        
        if retry % 10 == 0 {
            println!("⏳ {}/{} services ready, waiting... ({}s)", 
                     ready_services, services.len(), retry);
        }
        
        sleep(Duration::from_secs(1)).await;
    }
    
    Err(format!("Only {}/{} services ready after {}s", 
                ready_services, services.len(), max_retries).into())
}

/// Run all integration tests
pub async fn run_all_integration_tests() -> Result<TestReport, Box<dyn std::error::Error>> {
    println!("🚀 Starting FlowEx Integration Test Suite");
    println!("==========================================");
    
    // Wait for services to be ready
    wait_for_services().await?;
    
    let mut runner = TestSuiteRunner::new();
    
    // Run auth service tests
    runner.run_test("auth-service", "health_check", || async {
        auth_service_tests::run_auth_service_tests().await
    }).await;
    
    // Run trading service tests
    runner.run_test("trading-service", "health_check", || async {
        trading_service_tests::run_trading_service_tests().await
    }).await;
    
    // Add more service tests here as they are implemented
    
    // Generate and print report
    let report = runner.generate_report();
    runner.print_summary();
    
    Ok(report)
}

/// Test configuration
pub struct TestConfig {
    pub auth_service_url: String,
    pub trading_service_url: String,
    pub market_data_service_url: String,
    pub wallet_service_url: String,
    pub timeout: Duration,
}

impl Default for TestConfig {
    fn default() -> Self {
        Self {
            auth_service_url: "http://localhost:8001".to_string(),
            trading_service_url: "http://localhost:8002".to_string(),
            market_data_service_url: "http://localhost:8003".to_string(),
            wallet_service_url: "http://localhost:8004".to_string(),
            timeout: Duration::from_secs(30),
        }
    }
}

/// Load test configuration from environment
pub fn load_test_config() -> TestConfig {
    TestConfig {
        auth_service_url: std::env::var("AUTH_SERVICE_URL")
            .unwrap_or_else(|_| "http://localhost:8001".to_string()),
        trading_service_url: std::env::var("TRADING_SERVICE_URL")
            .unwrap_or_else(|_| "http://localhost:8002".to_string()),
        market_data_service_url: std::env::var("MARKET_DATA_SERVICE_URL")
            .unwrap_or_else(|_| "http://localhost:8003".to_string()),
        wallet_service_url: std::env::var("WALLET_SERVICE_URL")
            .unwrap_or_else(|_| "http://localhost:8004".to_string()),
        timeout: Duration::from_secs(
            std::env::var("TEST_TIMEOUT_SECONDS")
                .unwrap_or_else(|_| "30".to_string())
                .parse()
                .unwrap_or(30)
        ),
    }
}
