#!/bin/bash

# FlowEx Quick System Check
# Simple health check for FlowEx trading platform

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[✓]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[⚠]${NC} $1"
}

print_error() {
    echo -e "${RED}[✗]${NC} $1"
}

# Check if command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

print_status "FlowEx Quick System Check"
print_status "========================="

# Check Rust
if command_exists rustc; then
    rust_version=$(rustc --version | cut -d' ' -f2)
    print_success "Rust installed: $rust_version"
else
    print_error "Rust not installed"
fi

# Check Cargo
if command_exists cargo; then
    print_success "Cargo available"
else
    print_error "Cargo not available"
fi

# Check project compilation
print_status "Checking project compilation..."
if cargo check --quiet 2>/dev/null; then
    print_success "Project compiles successfully"
else
    print_error "Project has compilation errors"
fi

# Check if services can be built
print_status "Checking service binaries..."
services=("auth-service" "trading-service" "market-data-service" "wallet-service")

for service in "${services[@]}"; do
    if [ -f "target/debug/$service" ] || [ -f "target/release/$service" ]; then
        print_success "Binary exists: $service"
    else
        print_warning "Binary not found: $service (run 'cargo build' to create)"
    fi
done

# Check if services are running
print_status "Checking running services..."
service_urls=(
    "http://localhost:8001/health"
    "http://localhost:8002/health" 
    "http://localhost:8003/health"
    "http://localhost:8004/health"
)

running_services=0
for url in "${service_urls[@]}"; do
    if command_exists curl && curl -s -f "$url" >/dev/null 2>&1; then
        service_name=$(echo "$url" | cut -d':' -f2 | cut -d'/' -f3)
        print_success "Service responding: port $service_name"
        ((running_services++))
    fi
done

if [ $running_services -eq 0 ]; then
    print_warning "No services are currently running"
    print_status "To start services: npm run dev"
else
    print_success "$running_services services are running"
fi

# Check project structure
print_status "Checking project structure..."
required_files=("Cargo.toml" "docker-compose.yml" "package.json")
for file in "${required_files[@]}"; do
    if [ -f "$file" ]; then
        print_success "File exists: $file"
    else
        print_error "Missing file: $file"
    fi
done

required_dirs=("backend/services" "backend/shared" "scripts")
for dir in "${required_dirs[@]}"; do
    if [ -d "$dir" ]; then
        print_success "Directory exists: $dir"
    else
        print_error "Missing directory: $dir"
    fi
done

print_status "========================="
print_status "Quick check completed!"

if [ $running_services -gt 0 ]; then
    print_success "System appears to be working! 🎉"
    print_status "Next steps:"
    print_status "  - Run tests: ./scripts/test-enterprise.sh"
    print_status "  - Full check: ./scripts/system-check.sh"
else
    print_warning "Services are not running"
    print_status "Next steps:"
    print_status "  - Start services: npm run dev"
    print_status "  - Build project: cargo build"
    print_status "  - Run full check: ./scripts/system-check.sh"
fi

echo
