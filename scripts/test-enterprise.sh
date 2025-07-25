#!/bin/bash

# FlowEx Enterprise Test Suite
# Comprehensive testing script for FlowEx trading platform
# Supports unit tests, integration tests, performance tests, and security tests

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
TEST_RESULTS_DIR="$PROJECT_ROOT/test-results"
COVERAGE_DIR="$PROJECT_ROOT/coverage"

# Default values
RUN_UNIT_TESTS=true
RUN_INTEGRATION_TESTS=true
RUN_PERFORMANCE_TESTS=false
RUN_SECURITY_TESTS=false
GENERATE_COVERAGE=true
PARALLEL_TESTS=true
VERBOSE=false
FAIL_FAST=false
CLEAN_BEFORE_TEST=false

# Function to print colored output
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Function to show usage
show_usage() {
    cat << EOF
FlowEx Enterprise Test Suite

Usage: $0 [OPTIONS]

OPTIONS:
    -h, --help              Show this help message
    -u, --unit-only         Run only unit tests
    -i, --integration-only  Run only integration tests
    -p, --performance       Run performance tests
    -s, --security          Run security tests
    -c, --coverage          Generate code coverage report
    -n, --no-parallel       Disable parallel test execution
    -v, --verbose           Enable verbose output
    -f, --fail-fast         Stop on first test failure
    --clean                 Clean build artifacts before testing
    --no-coverage           Skip coverage generation

EXAMPLES:
    $0                      # Run all standard tests
    $0 -u -c               # Run unit tests with coverage
    $0 -i -v               # Run integration tests verbosely
    $0 -p -s               # Run performance and security tests
    $0 --clean -f          # Clean build and fail fast

ENVIRONMENT VARIABLES:
    RUST_LOG               Set log level (default: info)
    TEST_THREADS           Number of test threads (default: auto)
    TEST_TIMEOUT           Test timeout in seconds (default: 300)
    SKIP_SERVICE_CHECK     Skip service health checks (default: false)

EOF
}

# Parse command line arguments
parse_args() {
    while [[ $# -gt 0 ]]; do
        case $1 in
            -h|--help)
                show_usage
                exit 0
                ;;
            -u|--unit-only)
                RUN_UNIT_TESTS=true
                RUN_INTEGRATION_TESTS=false
                shift
                ;;
            -i|--integration-only)
                RUN_UNIT_TESTS=false
                RUN_INTEGRATION_TESTS=true
                shift
                ;;
            -p|--performance)
                RUN_PERFORMANCE_TESTS=true
                shift
                ;;
            -s|--security)
                RUN_SECURITY_TESTS=true
                shift
                ;;
            -c|--coverage)
                GENERATE_COVERAGE=true
                shift
                ;;
            --no-coverage)
                GENERATE_COVERAGE=false
                shift
                ;;
            -n|--no-parallel)
                PARALLEL_TESTS=false
                shift
                ;;
            -v|--verbose)
                VERBOSE=true
                shift
                ;;
            -f|--fail-fast)
                FAIL_FAST=true
                shift
                ;;
            --clean)
                CLEAN_BEFORE_TEST=true
                shift
                ;;
            *)
                print_error "Unknown option: $1"
                show_usage
                exit 1
                ;;
        esac
    done
}

# Function to check prerequisites
check_prerequisites() {
    print_status "Checking prerequisites..."
    
    # Check if Rust is installed
    if ! command -v cargo &> /dev/null; then
        print_error "Cargo not found. Please install Rust."
        exit 1
    fi
    
    # Check if required tools are available
    local tools=("curl" "jq")
    for tool in "${tools[@]}"; do
        if ! command -v "$tool" &> /dev/null; then
            print_warning "$tool not found. Some tests may fail."
        fi
    done
    
    # Create test results directory
    mkdir -p "$TEST_RESULTS_DIR"
    mkdir -p "$COVERAGE_DIR"
    
    print_success "Prerequisites check completed"
}

# Function to clean build artifacts
clean_build() {
    if [[ "$CLEAN_BEFORE_TEST" == "true" ]]; then
        print_status "Cleaning build artifacts..."
        cd "$PROJECT_ROOT"
        cargo clean
        rm -rf "$TEST_RESULTS_DIR"/*
        rm -rf "$COVERAGE_DIR"/*
        print_success "Build artifacts cleaned"
    fi
}

# Function to build the project
build_project() {
    print_status "Building FlowEx project..."
    cd "$PROJECT_ROOT"
    
    local build_args=""
    if [[ "$VERBOSE" == "true" ]]; then
        build_args="--verbose"
    fi
    
    if ! cargo build --release $build_args; then
        print_error "Build failed"
        exit 1
    fi
    
    print_success "Project built successfully"
}

# Function to run unit tests
run_unit_tests() {
    if [[ "$RUN_UNIT_TESTS" != "true" ]]; then
        return 0
    fi
    
    print_status "Running unit tests..."
    cd "$PROJECT_ROOT"
    
    local test_args="--lib"
    
    if [[ "$PARALLEL_TESTS" == "false" ]]; then
        test_args="$test_args -- --test-threads=1"
    fi
    
    if [[ "$FAIL_FAST" == "true" ]]; then
        test_args="$test_args --no-fail-fast"
    fi
    
    if [[ "$VERBOSE" == "true" ]]; then
        test_args="$test_args --verbose"
    fi
    
    # Run tests with coverage if enabled
    if [[ "$GENERATE_COVERAGE" == "true" ]]; then
        print_status "Running unit tests with coverage..."
        
        # Install cargo-tarpaulin if not available
        if ! command -v cargo-tarpaulin &> /dev/null; then
            print_status "Installing cargo-tarpaulin..."
            cargo install cargo-tarpaulin
        fi
        
        cargo tarpaulin \
            --out Html \
            --output-dir "$COVERAGE_DIR" \
            --exclude-files "tests/*" \
            --timeout 300 \
            $test_args
    else
        cargo test $test_args
    fi
    
    local exit_code=$?
    if [[ $exit_code -eq 0 ]]; then
        print_success "Unit tests passed"
    else
        print_error "Unit tests failed with exit code $exit_code"
        if [[ "$FAIL_FAST" == "true" ]]; then
            exit $exit_code
        fi
    fi
    
    return $exit_code
}

# Function to start services for integration tests
start_services() {
    print_status "Starting services for integration tests..."
    
    # Check if services are already running
    if curl -s http://localhost:8001/health &> /dev/null; then
        print_status "Services appear to be already running"
        return 0
    fi
    
    # Start services using the existing script
    if [[ -f "$PROJECT_ROOT/scripts/start-enterprise-environment.js" ]]; then
        print_status "Starting services with Node.js script..."
        cd "$PROJECT_ROOT"
        node scripts/start-enterprise-environment.js &
        local service_pid=$!
        
        # Wait for services to be ready
        local max_wait=60
        local wait_count=0
        
        while [[ $wait_count -lt $max_wait ]]; do
            if curl -s http://localhost:8001/health &> /dev/null; then
                print_success "Services are ready"
                return 0
            fi
            
            sleep 1
            ((wait_count++))
            
            if [[ $((wait_count % 10)) -eq 0 ]]; then
                print_status "Waiting for services... (${wait_count}s)"
            fi
        done
        
        print_error "Services failed to start within ${max_wait} seconds"
        kill $service_pid 2>/dev/null || true
        return 1
    else
        print_warning "Service startup script not found. Please start services manually."
        return 1
    fi
}

# Function to run integration tests
run_integration_tests() {
    if [[ "$RUN_INTEGRATION_TESTS" != "true" ]]; then
        return 0
    fi
    
    print_status "Running integration tests..."
    
    # Start services if needed
    if [[ "${SKIP_SERVICE_CHECK:-false}" != "true" ]]; then
        start_services
    fi
    
    cd "$PROJECT_ROOT"
    
    # Run the Rust test runner
    if [[ -f "tests/test_runner.rs" ]]; then
        print_status "Running Rust integration test runner..."
        cargo run --bin test_runner
    else
        # Fallback to Node.js test runner
        if [[ -f "scripts/run-comprehensive-tests.js" ]]; then
            print_status "Running Node.js test suite..."
            node scripts/run-comprehensive-tests.js
        else
            print_error "No test runner found"
            return 1
        fi
    fi
    
    local exit_code=$?
    if [[ $exit_code -eq 0 ]]; then
        print_success "Integration tests passed"
    else
        print_error "Integration tests failed with exit code $exit_code"
        if [[ "$FAIL_FAST" == "true" ]]; then
            exit $exit_code
        fi
    fi
    
    return $exit_code
}

# Function to run performance tests
run_performance_tests() {
    if [[ "$RUN_PERFORMANCE_TESTS" != "true" ]]; then
        return 0
    fi
    
    print_status "Running performance tests..."
    
    # Check if wrk is available for load testing
    if command -v wrk &> /dev/null; then
        print_status "Running load tests with wrk..."
        
        # Test auth endpoint
        wrk -t4 -c100 -d30s --timeout 10s \
            -s "$SCRIPT_DIR/load-test-auth.lua" \
            http://localhost:8001/api/auth/login
        
        # Test trading endpoint
        wrk -t4 -c100 -d30s --timeout 10s \
            http://localhost:8002/api/trading/pairs
            
    else
        print_warning "wrk not found. Skipping load tests."
    fi
    
    # Run Rust benchmark tests
    if cargo test --benches &> /dev/null; then
        print_status "Running Rust benchmark tests..."
        cargo test --benches
    else
        print_warning "No benchmark tests found"
    fi
    
    print_success "Performance tests completed"
}

# Function to run security tests
run_security_tests() {
    if [[ "$RUN_SECURITY_TESTS" != "true" ]]; then
        return 0
    fi
    
    print_status "Running security tests..."
    
    # Check for common security issues
    if command -v cargo-audit &> /dev/null; then
        print_status "Running cargo audit..."
        cargo audit
    else
        print_warning "cargo-audit not found. Install with: cargo install cargo-audit"
    fi
    
    # Check for unsafe code
    if command -v cargo-geiger &> /dev/null; then
        print_status "Running cargo geiger (unsafe code detection)..."
        cargo geiger
    else
        print_warning "cargo-geiger not found. Install with: cargo install cargo-geiger"
    fi
    
    print_success "Security tests completed"
}

# Function to generate test report
generate_report() {
    print_status "Generating test report..."
    
    local report_file="$TEST_RESULTS_DIR/test-report-$(date +%Y%m%d-%H%M%S).html"
    
    cat > "$report_file" << EOF
<!DOCTYPE html>
<html>
<head>
    <title>FlowEx Test Report</title>
    <style>
        body { font-family: Arial, sans-serif; margin: 20px; }
        .header { background: #f0f0f0; padding: 20px; border-radius: 5px; }
        .section { margin: 20px 0; }
        .success { color: green; }
        .error { color: red; }
        .warning { color: orange; }
    </style>
</head>
<body>
    <div class="header">
        <h1>FlowEx Enterprise Test Report</h1>
        <p>Generated: $(date)</p>
        <p>Project: FlowEx Trading Platform</p>
    </div>
    
    <div class="section">
        <h2>Test Configuration</h2>
        <ul>
            <li>Unit Tests: $RUN_UNIT_TESTS</li>
            <li>Integration Tests: $RUN_INTEGRATION_TESTS</li>
            <li>Performance Tests: $RUN_PERFORMANCE_TESTS</li>
            <li>Security Tests: $RUN_SECURITY_TESTS</li>
            <li>Coverage Generation: $GENERATE_COVERAGE</li>
            <li>Parallel Execution: $PARALLEL_TESTS</li>
        </ul>
    </div>
    
    <div class="section">
        <h2>Results</h2>
        <p>Detailed results are available in the test output above.</p>
        <p>Coverage report: <a href="../coverage/tarpaulin-report.html">Coverage Report</a></p>
    </div>
</body>
</html>
EOF
    
    print_success "Test report generated: $report_file"
}

# Main execution function
main() {
    print_status "Starting FlowEx Enterprise Test Suite"
    print_status "======================================"
    
    parse_args "$@"
    check_prerequisites
    clean_build
    build_project
    
    local overall_exit_code=0
    
    # Run tests
    run_unit_tests || overall_exit_code=$?
    run_integration_tests || overall_exit_code=$?
    run_performance_tests || overall_exit_code=$?
    run_security_tests || overall_exit_code=$?
    
    # Generate report
    generate_report
    
    # Final summary
    print_status "======================================"
    if [[ $overall_exit_code -eq 0 ]]; then
        print_success "All tests completed successfully! 🎉"
    else
        print_error "Some tests failed. Exit code: $overall_exit_code"
    fi
    
    print_status "Test results available in: $TEST_RESULTS_DIR"
    if [[ "$GENERATE_COVERAGE" == "true" ]]; then
        print_status "Coverage report available in: $COVERAGE_DIR"
    fi
    
    exit $overall_exit_code
}

# Run main function with all arguments
main "$@"
