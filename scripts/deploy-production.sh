#!/bin/bash

# FlowEx Production Deployment Script
# ===================================
# 
# Enterprise-grade deployment script for FlowEx trading platform
# Created by arkSong (arksong2018@gmail.com)
# 
# This script handles:
# - Environment validation
# - Database migrations
# - Service deployment
# - Health checks
# - Rollback capabilities

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
COMPOSE_FILE="$PROJECT_ROOT/docker-compose.production.yml"
ENV_FILE="$PROJECT_ROOT/.env.production"
BACKUP_DIR="$PROJECT_ROOT/backups"
LOG_FILE="$PROJECT_ROOT/logs/deployment.log"

# Ensure log directory exists
mkdir -p "$(dirname "$LOG_FILE")"
mkdir -p "$BACKUP_DIR"

# Logging function
log() {
    local level=$1
    shift
    local message="$*"
    local timestamp=$(date '+%Y-%m-%d %H:%M:%S')
    echo -e "${timestamp} [${level}] ${message}" | tee -a "$LOG_FILE"
}

info() {
    log "INFO" "${BLUE}$*${NC}"
}

warn() {
    log "WARN" "${YELLOW}$*${NC}"
}

error() {
    log "ERROR" "${RED}$*${NC}"
}

success() {
    log "SUCCESS" "${GREEN}$*${NC}"
}

# Error handling
cleanup() {
    local exit_code=$?
    if [ $exit_code -ne 0 ]; then
        error "Deployment failed with exit code $exit_code"
        error "Check logs at $LOG_FILE for details"
    fi
    exit $exit_code
}

trap cleanup EXIT

# Help function
show_help() {
    cat << EOF
FlowEx Production Deployment Script

Usage: $0 [OPTIONS] COMMAND

Commands:
    deploy          Deploy the entire FlowEx platform
    update          Update existing deployment
    rollback        Rollback to previous version
    status          Check deployment status
    logs            Show service logs
    backup          Create database backup
    restore         Restore from backup
    health          Run health checks

Options:
    -h, --help      Show this help message
    -v, --verbose   Enable verbose output
    -f, --force     Force deployment without confirmations
    --dry-run       Show what would be done without executing

Examples:
    $0 deploy                    # Deploy FlowEx platform
    $0 update --force           # Force update without confirmation
    $0 rollback                 # Rollback to previous version
    $0 status                   # Check all services status
    $0 logs trading-service     # Show trading service logs

EOF
}

# Parse command line arguments
VERBOSE=false
FORCE=false
DRY_RUN=false
COMMAND=""
SERVICE=""

while [[ $# -gt 0 ]]; do
    case $1 in
        -h|--help)
            show_help
            exit 0
            ;;
        -v|--verbose)
            VERBOSE=true
            shift
            ;;
        -f|--force)
            FORCE=true
            shift
            ;;
        --dry-run)
            DRY_RUN=true
            shift
            ;;
        deploy|update|rollback|status|logs|backup|restore|health)
            COMMAND=$1
            shift
            ;;
        *)
            if [[ -z "$SERVICE" && "$COMMAND" == "logs" ]]; then
                SERVICE=$1
            fi
            shift
            ;;
    esac
done

# Validation functions
check_prerequisites() {
    info "Checking prerequisites..."
    
    # Check if Docker is installed and running
    if ! command -v docker &> /dev/null; then
        error "Docker is not installed"
        exit 1
    fi
    
    if ! docker info &> /dev/null; then
        error "Docker is not running"
        exit 1
    fi
    
    # Check if Docker Compose is installed
    if ! command -v docker-compose &> /dev/null; then
        error "Docker Compose is not installed"
        exit 1
    fi
    
    # Check if required files exist
    if [[ ! -f "$COMPOSE_FILE" ]]; then
        error "Docker Compose file not found: $COMPOSE_FILE"
        exit 1
    fi
    
    if [[ ! -f "$ENV_FILE" ]]; then
        error "Environment file not found: $ENV_FILE"
        exit 1
    fi
    
    success "Prerequisites check passed"
}

validate_environment() {
    info "Validating environment configuration..."
    
    # Check required environment variables
    local required_vars=(
        "POSTGRES_PASSWORD"
        "REDIS_PASSWORD"
        "JWT_SECRET"
        "GRAFANA_PASSWORD"
    )
    
    source "$ENV_FILE"
    
    for var in "${required_vars[@]}"; do
        if [[ -z "${!var:-}" ]]; then
            error "Required environment variable $var is not set"
            exit 1
        fi
    done
    
    # Validate JWT secret length
    if [[ ${#JWT_SECRET} -lt 32 ]]; then
        error "JWT_SECRET must be at least 32 characters long"
        exit 1
    fi
    
    success "Environment validation passed"
}

# Backup functions
create_backup() {
    info "Creating database backup..."
    
    local backup_file="$BACKUP_DIR/flowex_backup_$(date +%Y%m%d_%H%M%S).sql"
    
    if [[ "$DRY_RUN" == "true" ]]; then
        info "[DRY RUN] Would create backup: $backup_file"
        return 0
    fi
    
    docker-compose -f "$COMPOSE_FILE" exec -T postgres pg_dump -U flowex flowex > "$backup_file"
    
    if [[ $? -eq 0 ]]; then
        success "Backup created: $backup_file"
        
        # Compress backup
        gzip "$backup_file"
        success "Backup compressed: ${backup_file}.gz"
        
        # Clean old backups (keep last 10)
        find "$BACKUP_DIR" -name "flowex_backup_*.sql.gz" -type f | sort -r | tail -n +11 | xargs rm -f
        
    else
        error "Failed to create backup"
        exit 1
    fi
}

# Deployment functions
deploy_services() {
    info "Deploying FlowEx services..."
    
    if [[ "$DRY_RUN" == "true" ]]; then
        info "[DRY RUN] Would deploy services using: $COMPOSE_FILE"
        return 0
    fi
    
    # Pull latest images
    info "Pulling latest images..."
    docker-compose -f "$COMPOSE_FILE" pull
    
    # Build services
    info "Building services..."
    docker-compose -f "$COMPOSE_FILE" build
    
    # Start services
    info "Starting services..."
    docker-compose -f "$COMPOSE_FILE" up -d
    
    success "Services deployed successfully"
}

run_migrations() {
    info "Running database migrations..."
    
    if [[ "$DRY_RUN" == "true" ]]; then
        info "[DRY RUN] Would run database migrations"
        return 0
    fi
    
    # Wait for database to be ready
    info "Waiting for database to be ready..."
    sleep 10
    
    # Run migrations (this would be implemented in the auth service)
    docker-compose -f "$COMPOSE_FILE" exec auth-service /app/migrate || {
        error "Database migration failed"
        exit 1
    }
    
    success "Database migrations completed"
}

health_check() {
    info "Running health checks..."
    
    local services=("api-gateway" "auth-service" "trading-service" "market-data-service" "wallet-service")
    local failed_services=()
    
    for service in "${services[@]}"; do
        info "Checking health of $service..."
        
        local port
        case $service in
            "api-gateway") port=8000 ;;
            "auth-service") port=8001 ;;
            "trading-service") port=8002 ;;
            "market-data-service") port=8003 ;;
            "wallet-service") port=8004 ;;
        esac
        
        local max_attempts=30
        local attempt=1
        
        while [[ $attempt -le $max_attempts ]]; do
            if curl -f -s "http://localhost:$port/health" > /dev/null; then
                success "$service is healthy"
                break
            fi
            
            if [[ $attempt -eq $max_attempts ]]; then
                error "$service health check failed"
                failed_services+=("$service")
                break
            fi
            
            info "Attempt $attempt/$max_attempts for $service..."
            sleep 5
            ((attempt++))
        done
    done
    
    if [[ ${#failed_services[@]} -gt 0 ]]; then
        error "Health check failed for services: ${failed_services[*]}"
        return 1
    fi
    
    success "All services are healthy"
}

# Command implementations
cmd_deploy() {
    info "Starting FlowEx production deployment..."
    
    if [[ "$FORCE" != "true" ]]; then
        read -p "Are you sure you want to deploy to production? (y/N): " -n 1 -r
        echo
        if [[ ! $REPLY =~ ^[Yy]$ ]]; then
            info "Deployment cancelled"
            exit 0
        fi
    fi
    
    check_prerequisites
    validate_environment
    create_backup
    deploy_services
    run_migrations
    health_check
    
    success "FlowEx deployment completed successfully!"
    info "Access the platform at: http://localhost:8000"
    info "Grafana dashboard: http://localhost:3000"
    info "Prometheus metrics: http://localhost:9090"
}

cmd_update() {
    info "Updating FlowEx deployment..."
    
    if [[ "$FORCE" != "true" ]]; then
        read -p "Are you sure you want to update the production deployment? (y/N): " -n 1 -r
        echo
        if [[ ! $REPLY =~ ^[Yy]$ ]]; then
            info "Update cancelled"
            exit 0
        fi
    fi
    
    create_backup
    
    info "Pulling latest changes..."
    docker-compose -f "$COMPOSE_FILE" pull
    docker-compose -f "$COMPOSE_FILE" build
    
    info "Restarting services..."
    docker-compose -f "$COMPOSE_FILE" up -d
    
    health_check
    success "Update completed successfully!"
}

cmd_rollback() {
    warn "Rolling back FlowEx deployment..."
    
    if [[ "$FORCE" != "true" ]]; then
        read -p "Are you sure you want to rollback? This will restore the last backup. (y/N): " -n 1 -r
        echo
        if [[ ! $REPLY =~ ^[Yy]$ ]]; then
            info "Rollback cancelled"
            exit 0
        fi
    fi
    
    # Find latest backup
    local latest_backup=$(find "$BACKUP_DIR" -name "flowex_backup_*.sql.gz" -type f | sort -r | head -n 1)
    
    if [[ -z "$latest_backup" ]]; then
        error "No backup found for rollback"
        exit 1
    fi
    
    info "Rolling back to backup: $latest_backup"
    
    # Stop services
    docker-compose -f "$COMPOSE_FILE" down
    
    # Restore database
    gunzip -c "$latest_backup" | docker-compose -f "$COMPOSE_FILE" exec -T postgres psql -U flowex -d flowex
    
    # Restart services
    docker-compose -f "$COMPOSE_FILE" up -d
    
    health_check
    success "Rollback completed successfully!"
}

cmd_status() {
    info "Checking FlowEx deployment status..."
    
    docker-compose -f "$COMPOSE_FILE" ps
    
    echo
    info "Service health status:"
    health_check
}

cmd_logs() {
    if [[ -n "$SERVICE" ]]; then
        info "Showing logs for $SERVICE..."
        docker-compose -f "$COMPOSE_FILE" logs -f "$SERVICE"
    else
        info "Showing logs for all services..."
        docker-compose -f "$COMPOSE_FILE" logs -f
    fi
}

cmd_backup() {
    create_backup
}

cmd_health() {
    health_check
}

# Main execution
main() {
    if [[ -z "$COMMAND" ]]; then
        error "No command specified"
        show_help
        exit 1
    fi
    
    case $COMMAND in
        deploy)
            cmd_deploy
            ;;
        update)
            cmd_update
            ;;
        rollback)
            cmd_rollback
            ;;
        status)
            cmd_status
            ;;
        logs)
            cmd_logs
            ;;
        backup)
            cmd_backup
            ;;
        health)
            cmd_health
            ;;
        *)
            error "Unknown command: $COMMAND"
            show_help
            exit 1
            ;;
    esac
}

# Run main function
main "$@"
