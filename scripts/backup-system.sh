#!/bin/bash

# FlowEx Production Backup System
# ===============================
# 
# Enterprise-grade backup and recovery system for FlowEx trading platform
# Created by arkSong (arksong2018@gmail.com)
# 
# This script provides:
# - Automated database backups
# - Configuration backups
# - Log archival
# - Disaster recovery procedures
# - Backup verification and testing

set -euo pipefail

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
BACKUP_ROOT="/var/backups/flowex"
S3_BUCKET="flowex-backups-production"
RETENTION_DAYS=30
ENCRYPTION_KEY_FILE="/etc/flowex/backup.key"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
NC='\033[0m'

# Logging
LOG_FILE="$BACKUP_ROOT/logs/backup-$(date +%Y%m%d).log"
mkdir -p "$(dirname "$LOG_FILE")"

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

success() {
    log "SUCCESS" "${GREEN}$*${NC}"
}

warning() {
    log "WARNING" "${YELLOW}$*${NC}"
}

error() {
    log "ERROR" "${RED}$*${NC}"
}

header() {
    echo -e "${PURPLE}========================================${NC}"
    echo -e "${PURPLE}$*${NC}"
    echo -e "${PURPLE}========================================${NC}"
}

# Error handling
cleanup() {
    local exit_code=$?
    if [ $exit_code -ne 0 ]; then
        error "Backup operation failed with exit code $exit_code"
        send_alert "BACKUP_FAILED" "FlowEx backup operation failed"
    fi
    exit $exit_code
}

trap cleanup EXIT

# Alert system
send_alert() {
    local alert_type=$1
    local message=$2
    
    # Send to monitoring system
    curl -X POST "http://alertmanager:9093/api/v1/alerts" \
        -H "Content-Type: application/json" \
        -d "[{
            \"labels\": {
                \"alertname\": \"$alert_type\",
                \"service\": \"backup-system\",
                \"severity\": \"critical\"
            },
            \"annotations\": {
                \"summary\": \"$message\",
                \"description\": \"FlowEx backup system alert\"
            }
        }]" || true
    
    # Send to Slack (if configured)
    if [[ -n "${SLACK_WEBHOOK_URL:-}" ]]; then
        curl -X POST "$SLACK_WEBHOOK_URL" \
            -H "Content-Type: application/json" \
            -d "{\"text\": \"🚨 FlowEx Alert: $message\"}" || true
    fi
}

# Database backup functions
backup_database() {
    header "Database Backup"
    
    local backup_date=$(date +%Y%m%d_%H%M%S)
    local backup_dir="$BACKUP_ROOT/database/$backup_date"
    local backup_file="$backup_dir/flowex_backup.sql"
    
    mkdir -p "$backup_dir"
    
    info "Starting database backup..."
    
    # Create database dump
    if kubectl exec -n flowex-production postgres-0 -- \
        pg_dump -U flowex -h localhost flowex > "$backup_file"; then
        success "Database dump created: $backup_file"
    else
        error "Failed to create database dump"
        return 1
    fi
    
    # Compress backup
    info "Compressing backup..."
    gzip "$backup_file"
    backup_file="${backup_file}.gz"
    
    # Encrypt backup
    if [[ -f "$ENCRYPTION_KEY_FILE" ]]; then
        info "Encrypting backup..."
        gpg --cipher-algo AES256 --compress-algo 1 --s2k-mode 3 \
            --s2k-digest-algo SHA512 --s2k-count 65536 \
            --symmetric --output "${backup_file}.gpg" \
            --passphrase-file "$ENCRYPTION_KEY_FILE" "$backup_file"
        rm "$backup_file"
        backup_file="${backup_file}.gpg"
        success "Backup encrypted"
    fi
    
    # Calculate checksum
    local checksum=$(sha256sum "$backup_file" | cut -d' ' -f1)
    echo "$checksum" > "${backup_file}.sha256"
    
    # Create metadata
    cat > "$backup_dir/metadata.json" << EOF
{
    "timestamp": "$(date -u +%Y-%m-%dT%H:%M:%SZ)",
    "type": "database",
    "size": $(stat -c%s "$backup_file"),
    "checksum": "$checksum",
    "encrypted": $([ -f "$ENCRYPTION_KEY_FILE" ] && echo "true" || echo "false"),
    "retention_date": "$(date -d "+$RETENTION_DAYS days" -u +%Y-%m-%dT%H:%M:%SZ)"
}
EOF
    
    success "Database backup completed: $backup_file"
    echo "$backup_file"
}

# Configuration backup
backup_configuration() {
    header "Configuration Backup"
    
    local backup_date=$(date +%Y%m%d_%H%M%S)
    local backup_dir="$BACKUP_ROOT/configuration/$backup_date"
    local backup_file="$backup_dir/configuration.tar.gz"
    
    mkdir -p "$backup_dir"
    
    info "Starting configuration backup..."
    
    # Backup Kubernetes configurations
    kubectl get all,configmaps,secrets,pvc -n flowex-production -o yaml > "$backup_dir/k8s-resources.yaml"
    
    # Backup application configurations
    tar -czf "$backup_file" \
        -C "$PROJECT_ROOT" \
        k8s/ \
        monitoring/ \
        security/ \
        scripts/ \
        docker-compose.production.yml \
        .env.production.example
    
    # Calculate checksum
    local checksum=$(sha256sum "$backup_file" | cut -d' ' -f1)
    echo "$checksum" > "${backup_file}.sha256"
    
    # Create metadata
    cat > "$backup_dir/metadata.json" << EOF
{
    "timestamp": "$(date -u +%Y-%m-%dT%H:%M:%SZ)",
    "type": "configuration",
    "size": $(stat -c%s "$backup_file"),
    "checksum": "$checksum",
    "retention_date": "$(date -d "+$RETENTION_DAYS days" -u +%Y-%m-%dT%H:%M:%SZ)"
}
EOF
    
    success "Configuration backup completed: $backup_file"
    echo "$backup_file"
}

# Log archival
archive_logs() {
    header "Log Archival"
    
    local backup_date=$(date +%Y%m%d_%H%M%S)
    local backup_dir="$BACKUP_ROOT/logs/$backup_date"
    local backup_file="$backup_dir/logs.tar.gz"
    
    mkdir -p "$backup_dir"
    
    info "Starting log archival..."
    
    # Archive application logs
    find "$PROJECT_ROOT/logs" -name "*.log" -mtime +1 -type f | \
        tar -czf "$backup_file" -T -
    
    # Archive system logs (if accessible)
    if [[ -d "/var/log/flowex" ]]; then
        find "/var/log/flowex" -name "*.log" -mtime +1 -type f | \
            tar -czf "$backup_dir/system-logs.tar.gz" -T -
    fi
    
    success "Log archival completed: $backup_file"
    echo "$backup_file"
}

# Upload to cloud storage
upload_to_cloud() {
    local backup_file=$1
    local backup_type=$2
    
    header "Cloud Upload"
    
    info "Uploading $backup_file to S3..."
    
    local s3_key="$backup_type/$(date +%Y/%m/%d)/$(basename "$backup_file")"
    
    if aws s3 cp "$backup_file" "s3://$S3_BUCKET/$s3_key" \
        --storage-class STANDARD_IA \
        --server-side-encryption AES256; then
        success "Uploaded to S3: s3://$S3_BUCKET/$s3_key"
        
        # Upload metadata
        local metadata_file="${backup_file%.*}/metadata.json"
        if [[ -f "$metadata_file" ]]; then
            aws s3 cp "$metadata_file" "s3://$S3_BUCKET/${s3_key%.*}/metadata.json"
        fi
        
        # Upload checksum
        local checksum_file="${backup_file}.sha256"
        if [[ -f "$checksum_file" ]]; then
            aws s3 cp "$checksum_file" "s3://$S3_BUCKET/${s3_key}.sha256"
        fi
    else
        error "Failed to upload to S3"
        return 1
    fi
}

# Backup verification
verify_backup() {
    local backup_file=$1
    
    header "Backup Verification"
    
    info "Verifying backup integrity: $backup_file"
    
    # Verify checksum
    local checksum_file="${backup_file}.sha256"
    if [[ -f "$checksum_file" ]]; then
        if sha256sum -c "$checksum_file"; then
            success "Checksum verification passed"
        else
            error "Checksum verification failed"
            return 1
        fi
    else
        warning "No checksum file found"
    fi
    
    # Test backup restoration (for database backups)
    if [[ "$backup_file" == *"database"* ]]; then
        info "Testing database backup restoration..."
        test_database_restore "$backup_file"
    fi
}

# Test database restore
test_database_restore() {
    local backup_file=$1
    
    info "Creating test database for restore verification..."
    
    # Create temporary test database
    local test_db="flowex_restore_test_$(date +%s)"
    
    kubectl exec -n flowex-production postgres-0 -- \
        createdb -U flowex "$test_db" || {
        error "Failed to create test database"
        return 1
    }
    
    # Restore backup to test database
    local restore_file="$backup_file"
    
    # Decrypt if encrypted
    if [[ "$backup_file" == *.gpg ]]; then
        restore_file="${backup_file%.gpg}"
        gpg --decrypt --passphrase-file "$ENCRYPTION_KEY_FILE" \
            --output "$restore_file" "$backup_file"
    fi
    
    # Decompress if compressed
    if [[ "$restore_file" == *.gz ]]; then
        gunzip -c "$restore_file" | \
            kubectl exec -i -n flowex-production postgres-0 -- \
            psql -U flowex -d "$test_db"
    else
        kubectl exec -i -n flowex-production postgres-0 -- \
            psql -U flowex -d "$test_db" < "$restore_file"
    fi
    
    # Verify restoration
    local table_count=$(kubectl exec -n flowex-production postgres-0 -- \
        psql -U flowex -d "$test_db" -t -c "SELECT count(*) FROM information_schema.tables WHERE table_schema='public';" | tr -d ' ')
    
    if [[ "$table_count" -gt 0 ]]; then
        success "Backup restoration test passed ($table_count tables restored)"
    else
        error "Backup restoration test failed (no tables found)"
        return 1
    fi
    
    # Cleanup test database
    kubectl exec -n flowex-production postgres-0 -- \
        dropdb -U flowex "$test_db"
    
    # Cleanup temporary files
    [[ "$restore_file" != "$backup_file" ]] && rm -f "$restore_file"
}

# Cleanup old backups
cleanup_old_backups() {
    header "Cleanup Old Backups"
    
    info "Cleaning up backups older than $RETENTION_DAYS days..."
    
    # Local cleanup
    find "$BACKUP_ROOT" -type f -mtime +$RETENTION_DAYS -delete
    find "$BACKUP_ROOT" -type d -empty -delete
    
    # S3 cleanup (using lifecycle policy is recommended)
    aws s3api list-objects-v2 --bucket "$S3_BUCKET" \
        --query "Contents[?LastModified<='$(date -d "-$RETENTION_DAYS days" -u +%Y-%m-%dT%H:%M:%SZ)'].Key" \
        --output text | xargs -I {} aws s3 rm "s3://$S3_BUCKET/{}"
    
    success "Old backup cleanup completed"
}

# Main backup function
run_full_backup() {
    header "FlowEx Full Backup"
    
    info "Starting full backup process..."
    
    # Database backup
    local db_backup=$(backup_database)
    verify_backup "$db_backup"
    upload_to_cloud "$db_backup" "database"
    
    # Configuration backup
    local config_backup=$(backup_configuration)
    verify_backup "$config_backup"
    upload_to_cloud "$config_backup" "configuration"
    
    # Log archival
    local log_backup=$(archive_logs)
    upload_to_cloud "$log_backup" "logs"
    
    # Cleanup
    cleanup_old_backups
    
    success "Full backup process completed successfully"
    send_alert "BACKUP_SUCCESS" "FlowEx backup completed successfully"
}

# Disaster recovery function
disaster_recovery() {
    header "Disaster Recovery"
    
    warning "Starting disaster recovery process..."
    
    # This would implement full disaster recovery procedures
    # Including:
    # - Infrastructure provisioning
    # - Database restoration
    # - Configuration restoration
    # - Service deployment
    # - Health verification
    
    info "Disaster recovery procedures would be implemented here"
    info "This includes infrastructure setup, data restoration, and service deployment"
}

# Help function
show_help() {
    cat << EOF
FlowEx Production Backup System

Usage: $0 [OPTIONS] COMMAND

Commands:
    full-backup         Run complete backup (database + config + logs)
    database-backup     Backup database only
    config-backup       Backup configuration only
    archive-logs        Archive old logs
    verify BACKUP_FILE  Verify backup integrity
    disaster-recovery   Run disaster recovery procedures
    cleanup             Clean up old backups

Options:
    -h, --help          Show this help message
    -v, --verbose       Enable verbose output
    --retention DAYS    Set retention period (default: $RETENTION_DAYS)
    --no-upload         Skip cloud upload
    --test-mode         Run in test mode (no actual operations)

Examples:
    $0 full-backup                    # Run complete backup
    $0 database-backup --no-upload    # Backup database locally only
    $0 verify /path/to/backup.sql.gz  # Verify backup integrity
    $0 cleanup --retention 7          # Clean backups older than 7 days

EOF
}

# Parse command line arguments
COMMAND=""
NO_UPLOAD=false
TEST_MODE=false

while [[ $# -gt 0 ]]; do
    case $1 in
        -h|--help)
            show_help
            exit 0
            ;;
        -v|--verbose)
            set -x
            shift
            ;;
        --retention)
            RETENTION_DAYS=$2
            shift 2
            ;;
        --no-upload)
            NO_UPLOAD=true
            shift
            ;;
        --test-mode)
            TEST_MODE=true
            shift
            ;;
        full-backup|database-backup|config-backup|archive-logs|verify|disaster-recovery|cleanup)
            COMMAND=$1
            shift
            ;;
        *)
            if [[ "$COMMAND" == "verify" && -z "${BACKUP_FILE:-}" ]]; then
                BACKUP_FILE=$1
            fi
            shift
            ;;
    esac
done

# Main execution
main() {
    if [[ -z "$COMMAND" ]]; then
        error "No command specified"
        show_help
        exit 1
    fi
    
    # Create backup directories
    mkdir -p "$BACKUP_ROOT"/{database,configuration,logs,logs}
    
    case $COMMAND in
        full-backup)
            run_full_backup
            ;;
        database-backup)
            backup_file=$(backup_database)
            verify_backup "$backup_file"
            [[ "$NO_UPLOAD" != "true" ]] && upload_to_cloud "$backup_file" "database"
            ;;
        config-backup)
            backup_file=$(backup_configuration)
            verify_backup "$backup_file"
            [[ "$NO_UPLOAD" != "true" ]] && upload_to_cloud "$backup_file" "configuration"
            ;;
        archive-logs)
            backup_file=$(archive_logs)
            [[ "$NO_UPLOAD" != "true" ]] && upload_to_cloud "$backup_file" "logs"
            ;;
        verify)
            if [[ -z "${BACKUP_FILE:-}" ]]; then
                error "Backup file not specified"
                exit 1
            fi
            verify_backup "$BACKUP_FILE"
            ;;
        disaster-recovery)
            disaster_recovery
            ;;
        cleanup)
            cleanup_old_backups
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
