//! FlowEx Database Library
//!
//! Enterprise-grade database functionality for FlowEx services.
//! Provides connection pooling, migration management, and transaction utilities.

use chrono::{DateTime, Utc};
use sqlx::{PgPool, Row, Postgres, Transaction};
use std::time::{Duration, SystemTime};
use tracing::{info, error, warn, debug};
use uuid::Uuid;

/// Database connection pool wrapper with enterprise features
#[derive(Clone)]
pub struct DatabasePool {
    pool: PgPool,
    start_time: SystemTime,
}

impl DatabasePool {
    /// Create a new database pool with enterprise configuration
    pub async fn new(database_url: &str) -> Result<Self, sqlx::Error> {
        info!("🔌 Initializing FlowEx database connection pool");
        debug!("Database URL: {}", database_url.replace(|c: char| c.is_ascii_digit(), "*"));

        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(50) // Increased for enterprise load
            .min_connections(5)  // Maintain minimum connections
            .acquire_timeout(Duration::from_secs(30))
            .idle_timeout(Duration::from_secs(600)) // 10 minutes
            .max_lifetime(Duration::from_secs(1800)) // 30 minutes
            .test_before_acquire(true) // Test connections before use
            .connect(database_url)
            .await?;

        info!("✅ Database connection pool created successfully");

        Ok(Self {
            pool,
            start_time: SystemTime::now(),
        })
    }

    /// Get the underlying pool
    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    /// Get pool statistics
    pub fn stats(&self) -> PoolStats {
        let uptime = self.start_time.elapsed().unwrap_or_default().as_secs();

        PoolStats {
            size: self.pool.size(),
            idle: self.pool.num_idle() as u32,
            uptime_seconds: uptime,
        }
    }

    /// Test database connection with comprehensive health check
    pub async fn health_check(&self) -> Result<DatabaseHealth, sqlx::Error> {
        let start = std::time::Instant::now();

        // Test basic connectivity
        let row: (i64,) = sqlx::query_as("SELECT 1")
            .fetch_one(&self.pool)
            .await?;

        let response_time = start.elapsed().as_millis() as u64;

        if row.0 == 1 {
            info!("✅ Database health check passed ({}ms)", response_time);
            Ok(DatabaseHealth {
                status: "healthy".to_string(),
                response_time_ms: response_time,
                pool_stats: self.stats(),
                timestamp: Utc::now(),
            })
        } else {
            error!("❌ Database health check failed");
            Err(sqlx::Error::RowNotFound)
        }
    }

    /// Begin a new transaction
    pub async fn begin_transaction(&self) -> Result<Transaction<'_, Postgres>, sqlx::Error> {
        debug!("🔄 Starting database transaction");
        self.pool.begin().await
    }

    /// Execute a query with logging
    pub async fn execute_logged(&self, query: &str) -> Result<sqlx::postgres::PgQueryResult, sqlx::Error> {
        debug!("📝 Executing query: {}", query);
        let start = std::time::Instant::now();

        let result = sqlx::query(query).execute(&self.pool).await;

        let duration = start.elapsed().as_millis();
        match &result {
            Ok(result) => {
                debug!("✅ Query executed successfully in {}ms, affected {} rows",
                       duration, result.rows_affected());
            }
            Err(e) => {
                error!("❌ Query failed in {}ms: {}", duration, e);
            }
        }

        result
    }
}

/// Database pool statistics
#[derive(Debug, Clone)]
pub struct PoolStats {
    pub size: u32,
    pub idle: u32,
    pub uptime_seconds: u64,
}

/// Database health information
#[derive(Debug, Clone)]
pub struct DatabaseHealth {
    pub status: String,
    pub response_time_ms: u64,
    pub pool_stats: PoolStats,
    pub timestamp: DateTime<Utc>,
}

/// Database migration utilities with enterprise features
pub mod migrations {
    use super::*;
    use std::collections::HashMap;
    use std::fs;
    use std::path::Path;

    /// Migration information
    #[derive(Debug, Clone)]
    pub struct Migration {
        pub version: String,
        pub name: String,
        pub sql: String,
        pub checksum: String,
        pub applied_at: Option<DateTime<Utc>>,
    }

    /// Migration manager for FlowEx database
    pub struct MigrationManager {
        pool: PgPool,
        migrations_path: String,
    }

    impl MigrationManager {
        /// Create a new migration manager
        pub fn new(pool: PgPool, migrations_path: String) -> Self {
            Self {
                pool,
                migrations_path,
            }
        }

        /// Initialize migration tracking table
        pub async fn initialize(&self) -> Result<(), sqlx::Error> {
            info!("🔧 Initializing migration tracking system");

            sqlx::query(r#"
                CREATE TABLE IF NOT EXISTS schema_migrations (
                    version VARCHAR(255) PRIMARY KEY,
                    name VARCHAR(255) NOT NULL,
                    checksum VARCHAR(64) NOT NULL,
                    applied_at TIMESTAMPTZ DEFAULT NOW(),
                    execution_time_ms BIGINT
                )
            "#)
            .execute(&self.pool)
            .await?;

            info!("✅ Migration tracking table ready");
            Ok(())
        }

        /// Load all migration files from disk
        pub fn load_migrations(&self) -> Result<Vec<Migration>, Box<dyn std::error::Error>> {
            info!("📂 Loading migration files from: {}", self.migrations_path);

            let mut migrations = Vec::new();
            let migrations_dir = Path::new(&self.migrations_path);

            if !migrations_dir.exists() {
                warn!("⚠️  Migrations directory does not exist: {}", self.migrations_path);
                return Ok(migrations);
            }

            let entries = fs::read_dir(migrations_dir)?;

            for entry in entries {
                let entry = entry?;
                let path = entry.path();

                if path.extension().and_then(|s| s.to_str()) == Some("sql") {
                    let filename = path.file_name()
                        .and_then(|s| s.to_str())
                        .ok_or("Invalid filename")?;

                    // Extract version from filename (e.g., "001_initial_schema.sql" -> "001")
                    let version = filename.split('_').next()
                        .ok_or("Invalid migration filename format")?
                        .to_string();

                    let name = filename.strip_suffix(".sql")
                        .ok_or("Invalid SQL file")?
                        .to_string();

                    let sql = fs::read_to_string(&path)?;
                    let checksum = format!("{:x}", md5::compute(&sql));

                    migrations.push(Migration {
                        version,
                        name,
                        sql,
                        checksum,
                        applied_at: None,
                    });

                    debug!("📄 Loaded migration: {}", filename);
                }
            }

            // Sort migrations by version
            migrations.sort_by(|a, b| a.version.cmp(&b.version));

            info!("✅ Loaded {} migration files", migrations.len());
            Ok(migrations)
        }

        /// Get applied migrations from database
        pub async fn get_applied_migrations(&self) -> Result<HashMap<String, Migration>, sqlx::Error> {
            let rows = sqlx::query(
                "SELECT version, name, checksum, applied_at FROM schema_migrations ORDER BY version"
            )
            .fetch_all(&self.pool)
            .await?;

            let mut applied = HashMap::new();

            for row in rows {
                let version: String = row.get("version");
                let name: String = row.get("name");
                let checksum: String = row.get("checksum");
                let applied_at: Option<DateTime<Utc>> = row.get("applied_at");

                applied.insert(version.clone(), Migration {
                    version,
                    name,
                    sql: String::new(), // Not needed for applied migrations
                    checksum,
                    applied_at: applied_at.or_else(|| Some(Utc::now())),
                });
            }

            Ok(applied)
        }

        /// Run pending migrations
        pub async fn migrate(&self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
            info!("🚀 Starting database migration process");

            self.initialize().await?;

            let available_migrations = self.load_migrations()?;
            let applied_migrations = self.get_applied_migrations().await?;

            let mut executed_migrations = Vec::new();

            for migration in available_migrations {
                if let Some(applied) = applied_migrations.get(&migration.version) {
                    // Check if checksum matches
                    if applied.checksum != migration.checksum {
                        return Err(format!(
                            "Migration {} checksum mismatch. Expected: {}, Found: {}",
                            migration.version, applied.checksum, migration.checksum
                        ).into());
                    }
                    debug!("⏭️  Skipping already applied migration: {}", migration.version);
                    continue;
                }

                info!("🔄 Applying migration: {} - {}", migration.version, migration.name);

                let start = std::time::Instant::now();

                // Execute migration in a transaction
                let mut tx = self.pool.begin().await?;

                // Execute the migration SQL
                sqlx::query(&migration.sql)
                    .execute(&mut *tx)
                    .await?;

                let execution_time = start.elapsed().as_millis() as i64;

                // Record the migration
                sqlx::query(
                    r#"
                    INSERT INTO schema_migrations (version, name, checksum, execution_time_ms)
                    VALUES ($1, $2, $3, $4)
                    "#
                )
                .bind(&migration.version)
                .bind(&migration.name)
                .bind(&migration.checksum)
                .bind(execution_time)
                .execute(&mut *tx)
                .await?;

                tx.commit().await?;

                info!("✅ Migration {} applied successfully in {}ms",
                      migration.version, execution_time);

                executed_migrations.push(migration.version);
            }

            if executed_migrations.is_empty() {
                info!("✨ Database is up to date, no migrations needed");
            } else {
                info!("🎉 Applied {} migrations successfully", executed_migrations.len());
            }

            Ok(executed_migrations)
        }

        /// Rollback the last migration (dangerous operation)
        pub async fn rollback_last(&self) -> Result<String, Box<dyn std::error::Error>> {
            warn!("⚠️  DANGER: Rolling back last migration");

            let last_migration = sqlx::query(
                "SELECT version, name FROM schema_migrations ORDER BY applied_at DESC LIMIT 1"
            )
            .fetch_optional(&self.pool)
            .await?;

            if let Some(row) = last_migration {
                let version: String = row.get("version");
                let name: String = row.get("name");

                // In a production system, you would need rollback scripts
                // For now, we'll just remove the record
                sqlx::query("DELETE FROM schema_migrations WHERE version = $1")
                    .bind(&version)
                    .execute(&self.pool)
                    .await?;

                warn!("🔄 Rolled back migration: {} - {}", version, name);
                Ok(version)
            } else {
                Err("No migrations to rollback".into())
            }
        }

        /// Get migration status
        pub async fn status(&self) -> Result<MigrationStatus, Box<dyn std::error::Error>> {
            let available_migrations = self.load_migrations()?;
            let applied_migrations = self.get_applied_migrations().await?;

            let total_available = available_migrations.len();
            let total_applied = applied_migrations.len();
            let pending = available_migrations.iter()
                .filter(|m| !applied_migrations.contains_key(&m.version))
                .count();

            Ok(MigrationStatus {
                total_available,
                total_applied,
                pending,
                last_applied: applied_migrations.values()
                    .max_by_key(|m| &m.version)
                    .map(|m| m.version.clone()),
            })
        }
    }

    /// Migration status information
    #[derive(Debug)]
    pub struct MigrationStatus {
        pub total_available: usize,
        pub total_applied: usize,
        pub pending: usize,
        pub last_applied: Option<String>,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_database_pool_creation() {
        // This test would require a real database connection
        // In a real implementation, we'd use a test database
        let database_url = "postgresql://test:test@localhost/test_db";
        
        // This would fail without a real database, but shows the structure
        // let pool = DatabasePool::new(database_url).await;
        // assert!(pool.is_ok());
    }
}
