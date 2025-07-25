//! FlowEx Test Runner
//!
//! Enterprise-grade test runner for FlowEx services with comprehensive
//! reporting, parallel execution, and CI/CD integration.

use std::env;
use std::process;
use std::time::Instant;
use tokio::time::{sleep, Duration};

mod integration;

use integration::{run_all_integration_tests, TestReport};

/// Test runner configuration
#[derive(Debug, Clone)]
pub struct TestRunnerConfig {
    pub parallel: bool,
    pub verbose: bool,
    pub fail_fast: bool,
    pub output_format: OutputFormat,
    pub report_file: Option<String>,
    pub services_startup_timeout: Duration,
}

#[derive(Debug, Clone)]
pub enum OutputFormat {
    Human,
    Json,
    Junit,
}

impl Default for TestRunnerConfig {
    fn default() -> Self {
        Self {
            parallel: true,
            verbose: false,
            fail_fast: false,
            output_format: OutputFormat::Human,
            report_file: None,
            services_startup_timeout: Duration::from_secs(60),
        }
    }
}

/// Load configuration from environment and command line
fn load_config() -> TestRunnerConfig {
    let mut config = TestRunnerConfig::default();
    
    // Check environment variables
    if env::var("TEST_PARALLEL").unwrap_or_default() == "false" {
        config.parallel = false;
    }
    
    if env::var("TEST_VERBOSE").unwrap_or_default() == "true" {
        config.verbose = true;
    }
    
    if env::var("TEST_FAIL_FAST").unwrap_or_default() == "true" {
        config.fail_fast = true;
    }
    
    if let Ok(format) = env::var("TEST_OUTPUT_FORMAT") {
        config.output_format = match format.to_lowercase().as_str() {
            "json" => OutputFormat::Json,
            "junit" => OutputFormat::Junit,
            _ => OutputFormat::Human,
        };
    }
    
    if let Ok(report_file) = env::var("TEST_REPORT_FILE") {
        config.report_file = Some(report_file);
    }
    
    if let Ok(timeout) = env::var("SERVICES_STARTUP_TIMEOUT") {
        if let Ok(seconds) = timeout.parse::<u64>() {
            config.services_startup_timeout = Duration::from_secs(seconds);
        }
    }
    
    config
}

/// Check if services are running
async fn check_services_health() -> Result<(), Box<dyn std::error::Error>> {
    let services = vec![
        ("auth-service", "http://localhost:8001/health"),
        ("trading-service", "http://localhost:8002/health"),
        ("market-data-service", "http://localhost:8003/health"),
        ("wallet-service", "http://localhost:8004/health"),
    ];
    
    let client = reqwest::Client::new();
    let mut all_healthy = true;
    
    println!("🔍 Checking service health...");
    
    for (service_name, health_url) in &services {
        match client.get(*health_url).timeout(Duration::from_secs(5)).send().await {
            Ok(response) if response.status().is_success() => {
                println!("✅ {} is healthy", service_name);
            }
            Ok(response) => {
                println!("⚠️  {} returned status: {}", service_name, response.status());
                all_healthy = false;
            }
            Err(e) => {
                println!("❌ {} is not responding: {}", service_name, e);
                all_healthy = false;
            }
        }
    }
    
    if !all_healthy {
        return Err("Some services are not healthy. Please start all services before running tests.".into());
    }
    
    println!("✅ All services are healthy and ready for testing");
    Ok(())
}

/// Generate JSON report
fn generate_json_report(report: &TestReport) -> serde_json::Value {
    serde_json::json!({
        "summary": {
            "total_tests": report.total_tests,
            "passed_tests": report.passed_tests,
            "failed_tests": report.failed_tests,
            "success_rate": if report.total_tests > 0 { 
                (report.passed_tests as f64 / report.total_tests as f64) * 100.0 
            } else { 
                0.0 
            },
            "total_duration_ms": report.total_duration.as_millis(),
        },
        "services": report.services.iter().map(|(name, stats)| {
            (name.clone(), serde_json::json!({
                "total": stats.total,
                "passed": stats.passed,
                "failed": stats.failed,
                "duration_ms": stats.duration.as_millis(),
                "success_rate": if stats.total > 0 { 
                    (stats.passed as f64 / stats.total as f64) * 100.0 
                } else { 
                    0.0 
                }
            }))
        }).collect::<serde_json::Map<_, _>>(),
        "results": report.results.iter().map(|result| {
            serde_json::json!({
                "service": result.service,
                "test_name": result.test_name,
                "passed": result.passed,
                "duration_ms": result.duration.as_millis(),
                "error": result.error
            })
        }).collect::<Vec<_>>()
    })
}

/// Generate JUnit XML report
fn generate_junit_report(report: &TestReport) -> String {
    let mut xml = String::new();
    xml.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
    xml.push_str(&format!(
        "<testsuites tests=\"{}\" failures=\"{}\" time=\"{:.3}\">\n",
        report.total_tests,
        report.failed_tests,
        report.total_duration.as_secs_f64()
    ));
    
    for (service_name, stats) in &report.services {
        xml.push_str(&format!(
            "  <testsuite name=\"{}\" tests=\"{}\" failures=\"{}\" time=\"{:.3}\">\n",
            service_name,
            stats.total,
            stats.failed,
            stats.duration.as_secs_f64()
        ));
        
        for result in &report.results {
            if result.service == *service_name {
                xml.push_str(&format!(
                    "    <testcase name=\"{}\" time=\"{:.3}\"",
                    result.test_name,
                    result.duration.as_secs_f64()
                ));
                
                if result.passed {
                    xml.push_str(" />\n");
                } else {
                    xml.push_str(">\n");
                    xml.push_str(&format!(
                        "      <failure message=\"{}\">{}</failure>\n",
                        result.error.as_ref().unwrap_or(&"Unknown error".to_string()),
                        result.error.as_ref().unwrap_or(&"Unknown error".to_string())
                    ));
                    xml.push_str("    </testcase>\n");
                }
            }
        }
        
        xml.push_str("  </testsuite>\n");
    }
    
    xml.push_str("</testsuites>\n");
    xml
}

/// Save report to file
async fn save_report(report: &TestReport, config: &TestRunnerConfig) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(report_file) = &config.report_file {
        let content = match config.output_format {
            OutputFormat::Json => generate_json_report(report).to_string(),
            OutputFormat::Junit => generate_junit_report(report),
            OutputFormat::Human => {
                format!("FlowEx Test Report\n==================\n\nTotal Tests: {}\nPassed: {}\nFailed: {}\nSuccess Rate: {:.1}%\nDuration: {:?}\n",
                        report.total_tests,
                        report.passed_tests,
                        report.failed_tests,
                        if report.total_tests > 0 { 
                            (report.passed_tests as f64 / report.total_tests as f64) * 100.0 
                        } else { 
                            0.0 
                        },
                        report.total_duration)
            }
        };
        
        tokio::fs::write(report_file, content).await?;
        println!("📄 Report saved to: {}", report_file);
    }
    
    Ok(())
}

/// Main test runner entry point
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_target(false)
        .with_level(true)
        .compact()
        .init();
    
    let config = load_config();
    
    println!("🚀 FlowEx Enterprise Test Runner");
    println!("=================================");
    println!("Configuration:");
    println!("  Parallel: {}", config.parallel);
    println!("  Verbose: {}", config.verbose);
    println!("  Fail Fast: {}", config.fail_fast);
    println!("  Output Format: {:?}", config.output_format);
    if let Some(ref file) = config.report_file {
        println!("  Report File: {}", file);
    }
    println!();
    
    // Check if we should skip service health checks
    if env::var("SKIP_SERVICE_CHECK").unwrap_or_default() != "true" {
        // Check service health
        if let Err(e) = check_services_health().await {
            eprintln!("❌ Service health check failed: {}", e);
            eprintln!("💡 Tip: Start services with 'npm run dev' or set SKIP_SERVICE_CHECK=true");
            process::exit(1);
        }
    } else {
        println!("⏭️  Skipping service health check (SKIP_SERVICE_CHECK=true)");
    }
    
    // Run tests
    let start_time = Instant::now();
    
    match run_all_integration_tests().await {
        Ok(report) => {
            let total_duration = start_time.elapsed();
            
            // Save report if configured
            save_report(&report, &config).await?;
            
            // Print final summary
            println!("\n🎯 FINAL RESULTS");
            println!("================");
            println!("Total Duration: {:?}", total_duration);
            
            if report.failed_tests == 0 {
                println!("🎉 All tests passed! ✅");
                process::exit(0);
            } else {
                println!("💥 {} test(s) failed! ❌", report.failed_tests);
                process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("💥 Test suite failed to run: {}", e);
            process::exit(1);
        }
    }
}
