#!/bin/bash

# FlowEx Frontend Build Script
# Enterprise-grade build script for the React frontend

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
FRONTEND_DIR="$PROJECT_ROOT/frontend"

# Default values
BUILD_MODE="production"
RUN_TESTS=true
RUN_LINT=true
GENERATE_BUNDLE_ANALYSIS=false
DOCKER_BUILD=false
VERBOSE=false

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

show_usage() {
    cat << EOF
FlowEx Frontend Build Script

Usage: $0 [OPTIONS]

OPTIONS:
    -h, --help              Show this help message
    -m, --mode MODE         Build mode: development, production (default: production)
    -t, --skip-tests        Skip running tests
    -l, --skip-lint         Skip linting
    -a, --analyze           Generate bundle analysis
    -d, --docker            Build Docker image
    -v, --verbose           Enable verbose output

EXAMPLES:
    $0                      # Standard production build
    $0 -m development       # Development build
    $0 -a -d               # Production build with analysis and Docker
    $0 --skip-tests --skip-lint  # Quick build without checks

EOF
}

parse_args() {
    while [[ $# -gt 0 ]]; do
        case $1 in
            -h|--help)
                show_usage
                exit 0
                ;;
            -m|--mode)
                BUILD_MODE="$2"
                shift 2
                ;;
            -t|--skip-tests)
                RUN_TESTS=false
                shift
                ;;
            -l|--skip-lint)
                RUN_LINT=false
                shift
                ;;
            -a|--analyze)
                GENERATE_BUNDLE_ANALYSIS=true
                shift
                ;;
            -d|--docker)
                DOCKER_BUILD=true
                shift
                ;;
            -v|--verbose)
                VERBOSE=true
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

check_prerequisites() {
    print_status "Checking prerequisites..."
    
    # Check if Node.js is installed
    if ! command -v node &> /dev/null; then
        print_error "Node.js not found. Please install Node.js 18 or later."
        exit 1
    fi
    
    # Check Node.js version
    local node_version
    node_version=$(node --version | cut -d'v' -f2)
    local major_version
    major_version=$(echo "$node_version" | cut -d'.' -f1)
    
    if [[ $major_version -lt 18 ]]; then
        print_error "Node.js version 18 or later is required. Current version: $node_version"
        exit 1
    fi
    
    print_success "Node.js version: $node_version"
    
    # Check if npm is installed
    if ! command -v npm &> /dev/null; then
        print_error "npm not found. Please install npm."
        exit 1
    fi
    
    local npm_version
    npm_version=$(npm --version)
    print_success "npm version: $npm_version"
    
    # Check if Docker is needed and available
    if [[ "$DOCKER_BUILD" == "true" ]]; then
        if ! command -v docker &> /dev/null; then
            print_error "Docker not found but Docker build requested."
            exit 1
        fi
        
        local docker_version
        docker_version=$(docker --version | cut -d' ' -f3 | tr -d ',')
        print_success "Docker version: $docker_version"
    fi
    
    print_success "Prerequisites check completed"
}

install_dependencies() {
    print_status "Installing dependencies..."
    cd "$FRONTEND_DIR"
    
    if [[ "$VERBOSE" == "true" ]]; then
        npm ci
    else
        npm ci --silent
    fi
    
    print_success "Dependencies installed"
}

run_linting() {
    if [[ "$RUN_LINT" != "true" ]]; then
        print_status "Skipping linting"
        return 0
    fi
    
    print_status "Running linting..."
    cd "$FRONTEND_DIR"
    
    # TypeScript type checking
    npm run type-check
    
    # ESLint
    npm run lint
    
    # Prettier format check
    npm run format:check
    
    print_success "Linting completed"
}

run_tests() {
    if [[ "$RUN_TESTS" != "true" ]]; then
        print_status "Skipping tests"
        return 0
    fi
    
    print_status "Running tests..."
    cd "$FRONTEND_DIR"
    
    # Run unit tests with coverage
    npm run test:coverage
    
    print_success "Tests completed"
}

build_application() {
    print_status "Building application for $BUILD_MODE..."
    cd "$FRONTEND_DIR"
    
    # Set environment variables
    export NODE_ENV="$BUILD_MODE"
    export VITE_ENVIRONMENT="$BUILD_MODE"
    
    if [[ "$BUILD_MODE" == "development" ]]; then
        export VITE_ENABLE_DEVTOOLS="true"
    else
        export VITE_ENABLE_DEVTOOLS="false"
    fi
    
    # Build the application
    if [[ "$VERBOSE" == "true" ]]; then
        npm run build
    else
        npm run build --silent
    fi
    
    print_success "Application built successfully"
}

generate_bundle_analysis() {
    if [[ "$GENERATE_BUNDLE_ANALYSIS" != "true" ]]; then
        return 0
    fi
    
    print_status "Generating bundle analysis..."
    cd "$FRONTEND_DIR"
    
    npm run analyze
    
    print_success "Bundle analysis generated"
}

build_docker_image() {
    if [[ "$DOCKER_BUILD" != "true" ]]; then
        return 0
    fi
    
    print_status "Building Docker image..."
    cd "$FRONTEND_DIR"
    
    local image_tag="flowex-frontend:latest"
    
    docker build \
        --build-arg VITE_ENVIRONMENT="$BUILD_MODE" \
        -t "$image_tag" \
        .
    
    print_success "Docker image built: $image_tag"
}

generate_build_report() {
    print_status "Generating build report..."
    
    local report_file="$FRONTEND_DIR/dist/build-report.json"
    local build_size
    build_size=$(du -sh "$FRONTEND_DIR/dist" | cut -f1)
    
    cat > "$report_file" << EOF
{
    "buildInfo": {
        "timestamp": "$(date -u +%Y-%m-%dT%H:%M:%SZ)",
        "mode": "$BUILD_MODE",
        "buildSize": "$build_size",
        "nodeVersion": "$(node --version)",
        "npmVersion": "$(npm --version)"
    },
    "configuration": {
        "runTests": $RUN_TESTS,
        "runLint": $RUN_LINT,
        "generateBundleAnalysis": $GENERATE_BUNDLE_ANALYSIS,
        "dockerBuild": $DOCKER_BUILD
    }
}
EOF
    
    print_success "Build report generated: $report_file"
}

main() {
    print_status "Starting FlowEx Frontend Build"
    print_status "=============================="
    
    parse_args "$@"
    check_prerequisites
    install_dependencies
    run_linting
    run_tests
    build_application
    generate_bundle_analysis
    build_docker_image
    generate_build_report
    
    print_status "=============================="
    print_success "Frontend build completed successfully! 🎉"
    print_status "Build mode: $BUILD_MODE"
    print_status "Build artifacts: $FRONTEND_DIR/dist"
    
    if [[ "$DOCKER_BUILD" == "true" ]]; then
        print_status "Docker image: flowex-frontend:latest"
    fi
}

main "$@"
