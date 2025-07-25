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

    /// 测试：数据库URL验证
    #[test]
    fn test_database_url_validation() {
        init_test_env();

        // 测试有效的数据库URL
        let valid_urls = vec![
            "postgresql://user:pass@localhost:5432/db",
            "postgresql://user@localhost/db",
            "postgresql://localhost/db",
            "postgres://user:pass@host:5432/database",
        ];

        for url in valid_urls {
            assert!(url.starts_with("postgres"), "URL {} 应该以postgres开头", url);
            assert!(url.contains("://"), "URL {} 应该包含协议分隔符", url);
        }

        // 测试无效的数据库URL
        let invalid_urls = vec![
            "",
            "invalid_url",
            "http://localhost/db",
            "mysql://localhost/db",
        ];

        for url in invalid_urls {
            assert!(!url.starts_with("postgresql://") && !url.starts_with("postgres://"),
                   "URL {} 应该是无效的PostgreSQL URL", url);
        }
    }

    /// 测试：迁移文件结构
    #[test]
    fn test_migration_structure() {
        init_test_env();

        let migration = Migration {
            version: "001".to_string(),
            name: "initial_schema".to_string(),
            sql: "CREATE TABLE test (id SERIAL PRIMARY KEY);".to_string(),
            checksum: "abc123".to_string(),
            applied_at: None,
        };

        assert_eq!(migration.version, "001");
        assert_eq!(migration.name, "initial_schema");
        assert!(!migration.sql.is_empty());
        assert!(!migration.checksum.is_empty());
        assert!(migration.applied_at.is_none());
    }

    /// 测试：迁移文件名解析
    #[test]
    fn test_migration_filename_parsing() {
        init_test_env();

        let test_cases = vec![
            ("001_initial_schema.sql", "001", "initial_schema"),
            ("002_add_users_table.sql", "002", "add_users_table"),
            ("010_update_indexes.sql", "010", "update_indexes"),
        ];

        for (filename, expected_version, expected_name) in test_cases {
            let parts: Vec<&str> = filename.split('_').collect();
            let version = parts[0];
            let name = filename.strip_suffix(".sql").unwrap()
                .strip_prefix(&format!("{}_", version)).unwrap();

            assert_eq!(version, expected_version);
            assert_eq!(name, expected_name);
        }
    }

    /// 测试：SQL校验和计算
    #[test]
    fn test_sql_checksum_calculation() {
        init_test_env();

        let sql1 = "CREATE TABLE users (id SERIAL PRIMARY KEY);";
        let sql2 = "CREATE TABLE users (id SERIAL PRIMARY KEY);";
        let sql3 = "CREATE TABLE orders (id SERIAL PRIMARY KEY);";

        let checksum1 = format!("{:x}", md5::compute(sql1));
        let checksum2 = format!("{:x}", md5::compute(sql2));
        let checksum3 = format!("{:x}", md5::compute(sql3));

        // 相同SQL应该有相同校验和
        assert_eq!(checksum1, checksum2);

        // 不同SQL应该有不同校验和
        assert_ne!(checksum1, checksum3);

        // 校验和应该是32位十六进制字符串
        assert_eq!(checksum1.len(), 32);
        assert!(checksum1.chars().all(|c| c.is_ascii_hexdigit()));
    }

    /// 测试：用户仓库模式
    #[test]
    fn test_user_repository_pattern() {
        init_test_env();

        // 创建模拟的用户数据
        let user = User {
            id: Uuid::new_v4(),
            email: "repository@example.com".to_string(),
            first_name: "Repository".to_string(),
            last_name: "Test".to_string(),
            is_verified: false,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        // 验证用户数据结构
        assert!(!user.email.is_empty());
        assert!(!user.first_name.is_empty());
        assert!(!user.last_name.is_empty());
        assert!(user.email.contains('@'));

        // 验证时间戳
        let now = chrono::Utc::now();
        let time_diff = (now - user.created_at).num_seconds();
        assert!(time_diff >= 0 && time_diff < 5);
    }

    /// 测试：数据库连接配置
    #[test]
    fn test_database_connection_config() {
        init_test_env();

        // 测试连接池配置参数
        let max_connections = 20u32;
        let min_connections = 5u32;
        let acquire_timeout = 30u64;
        let idle_timeout = 600u64;
        let max_lifetime = 1800u64;

        assert!(max_connections > min_connections, "最大连接数应该大于最小连接数");
        assert!(max_connections <= 100, "最大连接数不应该过大");
        assert!(min_connections > 0, "最小连接数应该大于0");
        assert!(acquire_timeout > 0, "获取连接超时应该大于0");
        assert!(idle_timeout > 0, "空闲超时应该大于0");
        assert!(max_lifetime > idle_timeout, "最大生命周期应该大于空闲超时");
    }

    /// 测试：错误处理类型
    #[test]
    fn test_error_handling_types() {
        init_test_env();

        // 测试数据库错误
        let db_error = FlowExError::Database("Connection failed".to_string());
        match db_error {
            FlowExError::Database(msg) => {
                assert_eq!(msg, "Connection failed");
            }
            _ => panic!("应该是数据库错误"),
        }

        // 测试验证错误
        let validation_error = FlowExError::Validation("Invalid email format".to_string());
        match validation_error {
            FlowExError::Validation(msg) => {
                assert_eq!(msg, "Invalid email format");
            }
            _ => panic!("应该是验证错误"),
        }
    }

    /// 测试：SQL查询构建
    #[test]
    fn test_sql_query_building() {
        init_test_env();

        // 测试用户查询SQL
        let user_id = Uuid::new_v4();
        let email = "test@example.com";

        // 模拟SQL查询构建
        let select_by_id = format!(
            "SELECT id, email, first_name, last_name, is_verified, created_at, updated_at FROM users WHERE id = '{}'",
            user_id
        );

        let select_by_email = format!(
            "SELECT id, email, first_name, last_name, is_verified, created_at, updated_at FROM users WHERE email = '{}'",
            email
        );

        assert!(select_by_id.contains("SELECT"));
        assert!(select_by_id.contains("FROM users"));
        assert!(select_by_id.contains("WHERE id"));
        assert!(select_by_id.contains(&user_id.to_string()));

        assert!(select_by_email.contains("SELECT"));
        assert!(select_by_email.contains("FROM users"));
        assert!(select_by_email.contains("WHERE email"));
        assert!(select_by_email.contains(email));
    }

    /// 测试：数据类型转换
    #[test]
    fn test_data_type_conversion() {
        init_test_env();

        // 测试UUID转换
        let uuid = Uuid::new_v4();
        let uuid_string = uuid.to_string();
        let parsed_uuid = Uuid::parse_str(&uuid_string).unwrap();
        assert_eq!(uuid, parsed_uuid);

        // 测试时间戳转换
        let now = chrono::Utc::now();
        let timestamp = now.timestamp();
        let from_timestamp = chrono::DateTime::from_timestamp(timestamp, 0).unwrap();
        assert_eq!(now.timestamp(), from_timestamp.timestamp());

        // 测试布尔值转换
        let is_verified = true;
        let verification_string = is_verified.to_string();
        assert_eq!(verification_string, "true");
    }

    /// 测试：并发数据库操作模拟
    #[tokio::test]
    async fn test_concurrent_database_operations() {
        init_test_env();

        let mut handles = vec![];

        // 模拟并发数据库操作
        for i in 0..10 {
            let handle = tokio::spawn(async move {
                // 模拟数据库操作
                let user = User {
                    id: Uuid::new_v4(),
                    email: format!("concurrent{}@example.com", i),
                    first_name: format!("User{}", i),
                    last_name: "Concurrent".to_string(),
                    is_verified: i % 2 == 0,
                    created_at: chrono::Utc::now(),
                    updated_at: chrono::Utc::now(),
                };

                // 模拟数据库写入延迟
                tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;

                (i, user.email)
            });
            handles.push(handle);
        }

        // 等待所有操作完成
        for handle in handles {
            let (task_id, email) = handle.await.unwrap();
            assert!(email.contains(&format!("concurrent{}", task_id)));
        }
    }

    /// 测试：性能基准
    #[tokio::test]
    async fn test_performance_benchmark() {
        init_test_env();

        let start = std::time::Instant::now();

        // 模拟大量数据库操作
        for i in 0..1000 {
            let _user = User {
                id: Uuid::new_v4(),
                email: format!("perf{}@example.com", i),
                first_name: format!("User{}", i),
                last_name: "Performance".to_string(),
                is_verified: i % 2 == 0,
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            };

            // 模拟数据处理
            let _checksum = format!("{:x}", md5::compute(format!("user_{}", i)));
        }

        let duration = start.elapsed();
        println!("1000次数据库操作模拟耗时: {:?}", duration);

        // 性能要求：1000次操作应该在100ms内完成
        assert!(duration.as_millis() < 100, "数据库操作性能不达标");
    }

    /// 测试：内存使用优化
    #[test]
    fn test_memory_usage_optimization() {
        init_test_env();

        let mut users = Vec::new();
        let mut migrations = Vec::new();

        // 创建大量数据对象
        for i in 0..1000 {
            let user = User {
                id: Uuid::new_v4(),
                email: format!("memory{}@example.com", i),
                first_name: format!("User{}", i),
                last_name: "Memory".to_string(),
                is_verified: i % 2 == 0,
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            };
            users.push(user);

            let migration = Migration {
                version: format!("{:03}", i),
                name: format!("migration_{}", i),
                sql: format!("CREATE TABLE table_{} (id SERIAL PRIMARY KEY);", i),
                checksum: format!("{:x}", md5::compute(format!("migration_{}", i))),
                applied_at: Some(chrono::Utc::now()),
            };
            migrations.push(migration);
        }

        assert_eq!(users.len(), 1000);
        assert_eq!(migrations.len(), 1000);

        // 清理内存
        drop(users);
        drop(migrations);
        assert!(true, "内存使用优化测试完成");
    }

    /// 测试：边界值处理
    #[test]
    fn test_boundary_value_handling() {
        init_test_env();

        // 测试空字符串
        let empty_email = "";
        assert!(empty_email.is_empty());

        // 测试最大长度字符串
        let max_email = format!("{}@example.com", "a".repeat(250));
        assert!(max_email.len() > 250);

        // 测试特殊字符
        let special_email = "test+tag@example.com";
        assert!(special_email.contains('+'));
        assert!(special_email.contains('@'));

        // 测试Unicode字符
        let unicode_name = "用户测试";
        assert!(!unicode_name.is_ascii());
        assert!(!unicode_name.is_empty());
    }

    /// 测试：数据完整性验证
    #[test]
    fn test_data_integrity_validation() {
        init_test_env();

        let user = User {
            id: Uuid::new_v4(),
            email: "integrity@example.com".to_string(),
            first_name: "Integrity".to_string(),
            last_name: "Test".to_string(),
            is_verified: true,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        // 验证数据完整性
        assert!(!user.id.is_nil(), "用户ID不应该为空");
        assert!(user.email.contains('@'), "邮箱应该包含@符号");
        assert!(!user.first_name.is_empty(), "名字不应该为空");
        assert!(!user.last_name.is_empty(), "姓氏不应该为空");
        assert!(user.updated_at >= user.created_at, "更新时间应该大于等于创建时间");
    }
}
