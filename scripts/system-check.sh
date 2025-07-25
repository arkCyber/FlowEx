#!/bin/bash

# FlowEx System Health Check Script
# Comprehensive system validation for FlowEx trading platform
# Checks all services, dependencies, and configurations

set -eo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

# Service endpoints
declare -A SERVICES=(
    ["auth-service"]="http://localhost:8001/health"
    ["trading-service"]="http://localhost:8002/health"
    ["market-data-service"]="http://localhost:8003/health"
    ["wallet-service"]="http://localhost:8004/health"
)

# Infrastructure endpoints
declare -A INFRASTRUCTURE=(
    ["postgres"]="postgresql://flowex:password@localhost:5432/flowex"
    ["redis"]="redis://localhost:6379"
    ["prometheus"]="http://localhost:9090/-/healthy"
    ["grafana"]="http://localhost:3001/api/health"
)

# Counters
TOTAL_CHECKS=0
PASSED_CHECKS=0
FAILED_CHECKS=0

# Function to print colored output
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[✓]${NC} $1"
    ((PASSED_CHECKS++))
}

print_warning() {
    echo -e "${YELLOW}[⚠]${NC} $1"
}

print_error() {
    echo -e "${RED}[✗]${NC} $1"
    ((FAILED_CHECKS++))
}

# Function to increment total checks
check_start() {
    ((TOTAL_CHECKS++))
}

# Function to check if a command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Function to check system prerequisites
check_prerequisites() {
    print_status "Checking system prerequisites..."
    
    # Check Rust installation
    check_start
    if command_exists rustc; then
        local rust_version
        rust_version=$(rustc --version | cut -d' ' -f2)
        print_success "Rust installed: $rust_version"
    else
        print_error "Rust not installed"
    fi
    
    # Check Cargo
    check_start
    if command_exists cargo; then
        local cargo_version
        cargo_version=$(cargo --version | cut -d' ' -f2)
        print_success "Cargo available: $cargo_version"
    else
        print_error "Cargo not available"
    fi
    
    # Check Docker
    check_start
    if command_exists docker; then
        local docker_version
        docker_version=$(docker --version | cut -d' ' -f3 | tr -d ',')
        print_success "Docker available: $docker_version"
    else
        print_warning "Docker not available (optional for development)"
    fi
    
    # Check PostgreSQL client
    check_start
    if command_exists psql; then
        local psql_version
        psql_version=$(psql --version | cut -d' ' -f3)
        print_success "PostgreSQL client available: $psql_version"
    else
        print_warning "PostgreSQL client not available"
    fi
    
    # Check Redis client
    check_start
    if command_exists redis-cli; then
        print_success "Redis client available"
    else
        print_warning "Redis client not available"
    fi
    
    # Check curl
    check_start
    if command_exists curl; then
        print_success "curl available"
    else
        print_error "curl not available (required for health checks)"
    fi
    
    # Check jq
    check_start
    if command_exists jq; then
        print_success "jq available"
    else
        print_warning "jq not available (optional for JSON parsing)"
    fi
}

# Function to check project structure
check_project_structure() {
    print_status "Checking project structure..."
    
    local required_files=(
        "Cargo.toml"
        "docker-compose.yml"
        "Dockerfile"
        ".env.example"
        "migrations/001_initial_schema.sql"
    )
    
    local required_dirs=(
        "backend/services"
        "backend/shared"
        "scripts"
        "tests"
    )
    
    cd "$PROJECT_ROOT"
    
    # Check required files
    for file in "${required_files[@]}"; do
        check_start
        if [[ -f "$file" ]]; then
            print_success "File exists: $file"
        else
            print_error "Missing file: $file"
        fi
    done
    
    # Check required directories
    for dir in "${required_dirs[@]}"; do
        check_start
        if [[ -d "$dir" ]]; then
            print_success "Directory exists: $dir"
        else
            print_error "Missing directory: $dir"
        fi
    done
}

# Function to check build status
check_build_status() {
    print_status "Checking build status..."
    
    cd "$PROJECT_ROOT"
    
    # Check if project compiles
    check_start
    if cargo check --quiet 2>/dev/null; then
        print_success "Project compiles successfully"
    else
        print_error "Project has compilation errors"
    fi
    
    # Check for built binaries
    local services=("auth-service" "trading-service" "market-data-service" "wallet-service")
    
    for service in "${services[@]}"; do
        check_start
        if [[ -f "target/debug/$service" ]] || [[ -f "target/release/$service" ]]; then
            print_success "Binary exists: $service"
        else
            print_warning "Binary not found: $service (run 'cargo build' to create)"
        fi
    done
}

# Function to check infrastructure services
check_infrastructure() {
    print_status "Checking infrastructure services..."
    
    # Check PostgreSQL
    check_start
    if command_exists psql; then
        if psql "postgresql://flowex:password@localhost:5432/flowex" -c "SELECT 1;" >/dev/null 2>&1; then
            print_success "PostgreSQL connection successful"
        else
            print_error "PostgreSQL connection failed"
        fi
    else
        print_warning "Cannot check PostgreSQL (psql not available)"
    fi
    
    # Check Redis
    check_start
    if command_exists redis-cli; then
        if redis-cli ping >/dev/null 2>&1; then
            print_success "Redis connection successful"
        else
            print_error "Redis connection failed"
        fi
    else
        print_warning "Cannot check Redis (redis-cli not available)"
    fi
    
    # Check other infrastructure services
    for service in "${!INFRASTRUCTURE[@]}"; do
        if [[ "$service" != "postgres" && "$service" != "redis" ]]; then
            check_start
            local url="${INFRASTRUCTURE[$service]}"
            if curl -s -f "$url" >/dev/null 2>&1; then
                print_success "$service is healthy"
            else
                print_warning "$service is not responding (may not be running)"
            fi
        fi
    done
}

# Function to check FlowEx services
check_flowex_services() {
    print_status "Checking FlowEx services..."
    
    for service in "${!SERVICES[@]}"; do
        check_start
        local url="${SERVICES[$service]}"
        
        if curl -s -f "$url" >/dev/null 2>&1; then
            # Get detailed health info if possible
            local health_info
            health_info=$(curl -s "$url" 2>/dev/null || echo "{}")
            
            if command_exists jq && echo "$health_info" | jq . >/dev/null 2>&1; then
                local status
                status=$(echo "$health_info" | jq -r '.status // "unknown"')
                local uptime
                uptime=$(echo "$health_info" | jq -r '.uptime // "unknown"')
                print_success "$service is healthy (status: $status, uptime: ${uptime}s)"
            else
                print_success "$service is responding"
            fi
        else
            print_error "$service is not responding"
        fi
    done
}

# Function to check API endpoints
check_api_endpoints() {
    print_status "Checking API endpoints..."
    
    # Test auth endpoints
    check_start
    if curl -s -f "http://localhost:8001/api/auth/me" >/dev/null 2>&1; then
        print_success "Auth API endpoints accessible"
    else
        print_warning "Auth API endpoints not accessible (may require authentication)"
    fi
    
    # Test trading endpoints
    check_start
    if curl -s -f "http://localhost:8002/api/trading/pairs" >/dev/null 2>&1; then
        print_success "Trading API endpoints accessible"
    else
        print_error "Trading API endpoints not accessible"
    fi
    
    # Test market data endpoints
    check_start
    if curl -s -f "http://localhost:8003/api/market-data/tickers" >/dev/null 2>&1; then
        print_success "Market data API endpoints accessible"
    else
        print_error "Market data API endpoints not accessible"
    fi
    
    # Test wallet endpoints
    check_start
    if curl -s -f "http://localhost:8004/api/wallet/balances" >/dev/null 2>&1; then
        print_success "Wallet API endpoints accessible"
    else
        print_warning "Wallet API endpoints not accessible (may require authentication)"
    fi
}

# Function to check configuration
check_configuration() {
    print_status "Checking configuration..."
    
    cd "$PROJECT_ROOT"
    
    # Check if .env file exists
    check_start
    if [[ -f ".env" ]]; then
        print_success ".env file exists"
    else
        print_warning ".env file not found (using defaults)"
    fi
    
    # Check critical environment variables
    local critical_vars=(
        "DATABASE_URL"
        "REDIS_URL"
        "JWT_SECRET"
    )
    
    for var in "${critical_vars[@]}"; do
        check_start
        if [[ -n "${!var:-}" ]]; then
            print_success "Environment variable set: $var"
        else
            print_warning "Environment variable not set: $var"
        fi
    done
}

# Function to run basic functionality tests
run_basic_tests() {
    print_status "Running basic functionality tests..."
    
    # Test login functionality
    check_start
    local login_response
    login_response=$(curl -s -X POST \
        -H "Content-Type: application/json" \
        -d '{"email":"demo@flowex.com","password":"demo123"}' \
        "http://localhost:8001/api/auth/login" 2>/dev/null || echo "{}")
    
    if echo "$login_response" | grep -q '"success":true'; then
        print_success "Login functionality working"
    else
        print_error "Login functionality not working"
    fi
    
    # Test trading pairs
    check_start
    local pairs_response
    pairs_response=$(curl -s "http://localhost:8002/api/trading/pairs" 2>/dev/null || echo "{}")
    
    if echo "$pairs_response" | grep -q '"success":true'; then
        print_success "Trading pairs endpoint working"
    else
        print_error "Trading pairs endpoint not working"
    fi
}

# Function to generate system report
generate_report() {
    local success_rate
    if [[ $TOTAL_CHECKS -gt 0 ]]; then
        success_rate=$(( (PASSED_CHECKS * 100) / TOTAL_CHECKS ))
    else
        success_rate=0
    fi
    
    echo
    print_status "==================================="
    print_status "FlowEx System Health Check Report"
    print_status "==================================="
    echo
    print_status "Total Checks: $TOTAL_CHECKS"
    print_status "Passed: $PASSED_CHECKS ✓"
    print_status "Failed: $FAILED_CHECKS ✗"
    print_status "Success Rate: $success_rate%"
    echo
    
    if [[ $success_rate -ge 90 ]]; then
        print_success "System is in excellent condition! 🎉"
    elif [[ $success_rate -ge 75 ]]; then
        print_warning "System is in good condition with minor issues ⚠️"
    elif [[ $success_rate -ge 50 ]]; then
        print_warning "System has some issues that need attention 🔧"
    else
        print_error "System has significant issues that require immediate attention 🚨"
    fi
    
    echo
    print_status "Next steps:"
    if [[ $FAILED_CHECKS -gt 0 ]]; then
        echo "  1. Review failed checks above"
        echo "  2. Fix any missing dependencies or services"
        echo "  3. Run 'npm run dev' to start services"
        echo "  4. Re-run this check with: ./scripts/system-check.sh"
    else
        echo "  1. System is ready for development!"
        echo "  2. Run tests with: ./scripts/test-enterprise.sh"
        echo "  3. Start development with: npm run dev"
    fi
    echo
}

# Main execution
main() {
    print_status "Starting FlowEx System Health Check"
    print_status "===================================="
    echo
    
    check_prerequisites
    echo
    check_project_structure
    echo
    check_build_status
    echo
    check_configuration
    echo
    check_infrastructure
    echo
    check_flowex_services
    echo
    check_api_endpoints
    echo
    run_basic_tests
    echo
    
    generate_report
    
    # Exit with appropriate code
    if [[ $FAILED_CHECKS -eq 0 ]]; then
        exit 0
    else
        exit 1
    fi
}

# Run main function
main "$@"
