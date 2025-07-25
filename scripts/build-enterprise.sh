#!/bin/bash

# FlowEx Enterprise Build and Deployment Script
# Comprehensive build script for FlowEx trading platform
# Supports development, staging, and production builds

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
BUILD_DIR="$PROJECT_ROOT/build"
DIST_DIR="$PROJECT_ROOT/dist"

# Default values
BUILD_MODE="release"
TARGET_ENV="development"
RUN_TESTS=true
GENERATE_DOCS=true
CREATE_DOCKER_IMAGE=false
PUSH_TO_REGISTRY=false
DEPLOY_TO_K8S=false
CLEAN_BUILD=false
VERBOSE=false

# Docker configuration
DOCKER_REGISTRY="flowex"
DOCKER_TAG="latest"
DOCKER_PLATFORMS="linux/amd64,linux/arm64"

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
FlowEx Enterprise Build Script

Usage: $0 [OPTIONS]

OPTIONS:
    -h, --help              Show this help message
    -m, --mode MODE         Build mode: debug, release (default: release)
    -e, --env ENV           Target environment: development, staging, production (default: development)
    -t, --skip-tests        Skip running tests
    -d, --skip-docs         Skip generating documentation
    --docker                Build Docker image
    --push                  Push Docker image to registry
    --k8s                   Deploy to Kubernetes
    --clean                 Clean build artifacts before building
    -v, --verbose           Enable verbose output
    --tag TAG               Docker tag (default: latest)
    --registry REGISTRY     Docker registry (default: flowex)

EXAMPLES:
    $0                      # Standard development build
    $0 -m release -e production --docker --push
                           # Production build with Docker image
    $0 --clean -v          # Clean verbose build
    $0 --k8s --tag v1.0.0  # Deploy to Kubernetes with specific tag

ENVIRONMENT VARIABLES:
    RUST_LOG               Set log level (default: info)
    CARGO_TARGET_DIR       Cargo target directory
    DOCKER_BUILDKIT        Enable Docker BuildKit (default: 1)

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
            -m|--mode)
                BUILD_MODE="$2"
                shift 2
                ;;
            -e|--env)
                TARGET_ENV="$2"
                shift 2
                ;;
            -t|--skip-tests)
                RUN_TESTS=false
                shift
                ;;
            -d|--skip-docs)
                GENERATE_DOCS=false
                shift
                ;;
            --docker)
                CREATE_DOCKER_IMAGE=true
                shift
                ;;
            --push)
                PUSH_TO_REGISTRY=true
                CREATE_DOCKER_IMAGE=true
                shift
                ;;
            --k8s)
                DEPLOY_TO_K8S=true
                CREATE_DOCKER_IMAGE=true
                shift
                ;;
            --clean)
                CLEAN_BUILD=true
                shift
                ;;
            -v|--verbose)
                VERBOSE=true
                shift
                ;;
            --tag)
                DOCKER_TAG="$2"
                shift 2
                ;;
            --registry)
                DOCKER_REGISTRY="$2"
                shift 2
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
    
    # Check Rust version
    local rust_version
    rust_version=$(rustc --version | cut -d' ' -f2)
    print_status "Rust version: $rust_version"
    
    # Check if Docker is needed and available
    if [[ "$CREATE_DOCKER_IMAGE" == "true" ]]; then
        if ! command -v docker &> /dev/null; then
            print_error "Docker not found but Docker build requested."
            exit 1
        fi
        
        # Enable BuildKit
        export DOCKER_BUILDKIT=1
        print_status "Docker BuildKit enabled"
    fi
    
    # Check if kubectl is needed and available
    if [[ "$DEPLOY_TO_K8S" == "true" ]]; then
        if ! command -v kubectl &> /dev/null; then
            print_error "kubectl not found but Kubernetes deployment requested."
            exit 1
        fi
    fi
    
    # Create build directories
    mkdir -p "$BUILD_DIR"
    mkdir -p "$DIST_DIR"
    
    print_success "Prerequisites check completed"
}

# Function to clean build artifacts
clean_build() {
    if [[ "$CLEAN_BUILD" == "true" ]]; then
        print_status "Cleaning build artifacts..."
        cd "$PROJECT_ROOT"
        cargo clean
        rm -rf "$BUILD_DIR"/*
        rm -rf "$DIST_DIR"/*
        print_success "Build artifacts cleaned"
    fi
}

# Function to set environment variables
set_environment() {
    print_status "Setting up environment for: $TARGET_ENV"
    
    case "$TARGET_ENV" in
        development)
            export RUST_LOG="${RUST_LOG:-debug}"
            export FLOWEX_LOG_LEVEL="debug"
            ;;
        staging)
            export RUST_LOG="${RUST_LOG:-info}"
            export FLOWEX_LOG_LEVEL="info"
            ;;
        production)
            export RUST_LOG="${RUST_LOG:-warn}"
            export FLOWEX_LOG_LEVEL="warn"
            ;;
        *)
            print_error "Unknown environment: $TARGET_ENV"
            exit 1
            ;;
    esac
    
    print_status "Environment configured for $TARGET_ENV"
}

# Function to build the project
build_project() {
    print_status "Building FlowEx project in $BUILD_MODE mode..."
    cd "$PROJECT_ROOT"
    
    local build_args=""
    
    if [[ "$BUILD_MODE" == "release" ]]; then
        build_args="--release"
    fi
    
    if [[ "$VERBOSE" == "true" ]]; then
        build_args="$build_args --verbose"
    fi
    
    # Build all services
    local services=("auth-service" "trading-service" "market-data-service" "wallet-service")
    
    for service in "${services[@]}"; do
        print_status "Building $service..."
        cargo build --bin "$service" $build_args
    done
    
    # Copy binaries to build directory
    local target_dir="target"
    if [[ "$BUILD_MODE" == "release" ]]; then
        target_dir="target/release"
    else
        target_dir="target/debug"
    fi
    
    for service in "${services[@]}"; do
        cp "$target_dir/$service" "$BUILD_DIR/"
    done
    
    print_success "Project built successfully"
}

# Function to run tests
run_tests() {
    if [[ "$RUN_TESTS" != "true" ]]; then
        print_status "Skipping tests"
        return 0
    fi
    
    print_status "Running tests..."
    cd "$PROJECT_ROOT"
    
    # Run unit tests
    cargo test --lib
    
    # Run integration tests if services are available
    if [[ -f "scripts/test-enterprise.sh" ]]; then
        print_status "Running enterprise test suite..."
        bash scripts/test-enterprise.sh -u
    fi
    
    print_success "Tests completed"
}

# Function to generate documentation
generate_docs() {
    if [[ "$GENERATE_DOCS" != "true" ]]; then
        print_status "Skipping documentation generation"
        return 0
    fi
    
    print_status "Generating documentation..."
    cd "$PROJECT_ROOT"
    
    # Generate Rust documentation
    cargo doc --no-deps --document-private-items
    
    # Copy documentation to dist directory
    cp -r target/doc "$DIST_DIR/"
    
    print_success "Documentation generated"
}

# Function to create Docker image
create_docker_image() {
    if [[ "$CREATE_DOCKER_IMAGE" != "true" ]]; then
        return 0
    fi
    
    print_status "Building Docker image..."
    cd "$PROJECT_ROOT"
    
    local image_name="$DOCKER_REGISTRY/flowex:$DOCKER_TAG"
    
    # Build multi-platform image if buildx is available
    if docker buildx version &> /dev/null; then
        print_status "Building multi-platform image with buildx..."
        docker buildx build \
            --platform "$DOCKER_PLATFORMS" \
            --tag "$image_name" \
            --build-arg BUILD_MODE="$BUILD_MODE" \
            --build-arg TARGET_ENV="$TARGET_ENV" \
            .
    else
        print_status "Building single-platform image..."
        docker build \
            --tag "$image_name" \
            --build-arg BUILD_MODE="$BUILD_MODE" \
            --build-arg TARGET_ENV="$TARGET_ENV" \
            .
    fi
    
    print_success "Docker image created: $image_name"
}

# Function to push Docker image
push_docker_image() {
    if [[ "$PUSH_TO_REGISTRY" != "true" ]]; then
        return 0
    fi
    
    print_status "Pushing Docker image to registry..."
    
    local image_name="$DOCKER_REGISTRY/flowex:$DOCKER_TAG"
    
    docker push "$image_name"
    
    print_success "Docker image pushed: $image_name"
}

# Function to deploy to Kubernetes
deploy_to_kubernetes() {
    if [[ "$DEPLOY_TO_K8S" != "true" ]]; then
        return 0
    fi
    
    print_status "Deploying to Kubernetes..."
    
    # Check if Kubernetes manifests exist
    local k8s_dir="$PROJECT_ROOT/k8s"
    if [[ ! -d "$k8s_dir" ]]; then
        print_error "Kubernetes manifests directory not found: $k8s_dir"
        return 1
    fi
    
    # Apply manifests
    kubectl apply -f "$k8s_dir/"
    
    # Update image tag in deployment
    local image_name="$DOCKER_REGISTRY/flowex:$DOCKER_TAG"
    kubectl set image deployment/flowex-auth-service auth-service="$image_name"
    kubectl set image deployment/flowex-trading-service trading-service="$image_name"
    kubectl set image deployment/flowex-market-data-service market-data-service="$image_name"
    kubectl set image deployment/flowex-wallet-service wallet-service="$image_name"
    
    # Wait for rollout to complete
    kubectl rollout status deployment/flowex-auth-service
    kubectl rollout status deployment/flowex-trading-service
    kubectl rollout status deployment/flowex-market-data-service
    kubectl rollout status deployment/flowex-wallet-service
    
    print_success "Deployment to Kubernetes completed"
}

# Function to create release package
create_release_package() {
    print_status "Creating release package..."
    
    local package_name="flowex-$TARGET_ENV-$DOCKER_TAG"
    local package_dir="$DIST_DIR/$package_name"
    
    mkdir -p "$package_dir"
    
    # Copy binaries
    cp -r "$BUILD_DIR"/* "$package_dir/"
    
    # Copy configuration files
    cp .env.example "$package_dir/env.example"
    cp docker-compose.yml "$package_dir/"
    cp -r migrations "$package_dir/"
    
    # Copy documentation
    if [[ -d "$DIST_DIR/doc" ]]; then
        cp -r "$DIST_DIR/doc" "$package_dir/"
    fi
    
    # Create archive
    cd "$DIST_DIR"
    tar -czf "$package_name.tar.gz" "$package_name"
    
    print_success "Release package created: $DIST_DIR/$package_name.tar.gz"
}

# Function to generate build report
generate_build_report() {
    print_status "Generating build report..."
    
    local report_file="$DIST_DIR/build-report-$(date +%Y%m%d-%H%M%S).json"
    
    cat > "$report_file" << EOF
{
    "build_info": {
        "timestamp": "$(date -u +%Y-%m-%dT%H:%M:%SZ)",
        "mode": "$BUILD_MODE",
        "environment": "$TARGET_ENV",
        "version": "$DOCKER_TAG",
        "git_commit": "$(git rev-parse HEAD 2>/dev/null || echo 'unknown')",
        "git_branch": "$(git branch --show-current 2>/dev/null || echo 'unknown')"
    },
    "configuration": {
        "run_tests": $RUN_TESTS,
        "generate_docs": $GENERATE_DOCS,
        "create_docker_image": $CREATE_DOCKER_IMAGE,
        "push_to_registry": $PUSH_TO_REGISTRY,
        "deploy_to_k8s": $DEPLOY_TO_K8S
    },
    "artifacts": {
        "binaries": "$(ls -la $BUILD_DIR 2>/dev/null || echo 'none')",
        "docker_image": "$DOCKER_REGISTRY/flowex:$DOCKER_TAG"
    }
}
EOF
    
    print_success "Build report generated: $report_file"
}

# Main execution function
main() {
    print_status "Starting FlowEx Enterprise Build"
    print_status "================================="
    
    parse_args "$@"
    check_prerequisites
    set_environment
    clean_build
    build_project
    run_tests
    generate_docs
    create_docker_image
    push_docker_image
    deploy_to_kubernetes
    create_release_package
    generate_build_report
    
    # Final summary
    print_status "================================="
    print_success "Build completed successfully! 🎉"
    print_status "Build mode: $BUILD_MODE"
    print_status "Target environment: $TARGET_ENV"
    print_status "Docker tag: $DOCKER_TAG"
    print_status "Artifacts available in: $DIST_DIR"
    
    if [[ "$CREATE_DOCKER_IMAGE" == "true" ]]; then
        print_status "Docker image: $DOCKER_REGISTRY/flowex:$DOCKER_TAG"
    fi
    
    if [[ "$DEPLOY_TO_K8S" == "true" ]]; then
        print_status "Deployed to Kubernetes cluster"
    fi
}

# Run main function with all arguments
main "$@"
