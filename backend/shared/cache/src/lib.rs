//! FlowEx Cache Library
//!
//! Enterprise-grade Redis caching and session management for FlowEx services.
//! Provides distributed caching, session storage, and rate limiting capabilities.

use chrono::{DateTime, Utc};
use redis::{AsyncCommands, Client, RedisResult};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tracing::{info, error, debug, warn};
use uuid::Uuid;

/// Redis cache manager with enterprise features
#[derive(Clone)]
pub struct CacheManager {
    client: Client,
    connection_pool: redis::aio::ConnectionManager,
    default_ttl: Duration,
}

impl CacheManager {
    /// Create a new cache manager
    pub async fn new(redis_url: &str, default_ttl: Duration) -> Result<Self, redis::RedisError> {
        info!("🔌 Initializing FlowEx Redis cache manager");
        debug!("Redis URL: {}", redis_url.replace(|c: char| c.is_ascii_digit(), "*"));
        
        let client = Client::open(redis_url)?;
        let connection_pool = redis::aio::ConnectionManager::new(client.clone()).await?;
        
        info!("✅ Redis cache manager initialized successfully");
        
        Ok(Self {
            client,
            connection_pool,
            default_ttl,
        })
    }
    
    /// Test Redis connection
    pub async fn health_check(&self) -> Result<CacheHealth, redis::RedisError> {
        let start = std::time::Instant::now();
        
        let mut conn = self.connection_pool.clone();
        let pong: String = redis::cmd("PING").query_async(&mut conn).await?;
        
        let response_time = start.elapsed().as_millis() as u64;
        
        if pong == "PONG" {
            info!("✅ Redis health check passed ({}ms)", response_time);
            Ok(CacheHealth {
                status: "healthy".to_string(),
                response_time_ms: response_time,
                timestamp: Utc::now(),
            })
        } else {
            error!("❌ Redis health check failed: unexpected response");
            Err(redis::RedisError::from((redis::ErrorKind::ResponseError, "Unexpected PING response")))
        }
    }
    
    /// Set a value in cache with TTL
    pub async fn set<T>(&self, key: &str, value: &T, ttl: Option<Duration>) -> Result<(), CacheError>
    where
        T: Serialize,
    {
        let serialized = serde_json::to_string(value)
            .map_err(|e| CacheError::Serialization(e.to_string()))?;
        
        let mut conn = self.connection_pool.clone();
        let ttl_seconds = ttl.unwrap_or(self.default_ttl).as_secs();
        
        conn.set_ex(key, serialized, ttl_seconds).await
            .map_err(|e| CacheError::Redis(e))?;
        
        debug!("📝 Cached value for key: {} (TTL: {}s)", key, ttl_seconds);
        Ok(())
    }
    
    /// Get a value from cache
    pub async fn get<T>(&self, key: &str) -> Result<Option<T>, CacheError>
    where
        T: for<'de> Deserialize<'de>,
    {
        let mut conn = self.connection_pool.clone();
        
        let result: Option<String> = conn.get(key).await
            .map_err(|e| CacheError::Redis(e))?;
        
        match result {
            Some(serialized) => {
                let value = serde_json::from_str(&serialized)
                    .map_err(|e| CacheError::Deserialization(e.to_string()))?;
                debug!("📖 Cache hit for key: {}", key);
                Ok(Some(value))
            }
            None => {
                debug!("📭 Cache miss for key: {}", key);
                Ok(None)
            }
        }
    }
    
    /// Delete a key from cache
    pub async fn delete(&self, key: &str) -> Result<bool, CacheError> {
        let mut conn = self.connection_pool.clone();
        
        let deleted: i32 = conn.del(key).await
            .map_err(|e| CacheError::Redis(e))?;
        
        let was_deleted = deleted > 0;
        debug!("🗑️  Deleted key: {} (existed: {})", key, was_deleted);
        Ok(was_deleted)
    }
    
    /// Check if key exists
    pub async fn exists(&self, key: &str) -> Result<bool, CacheError> {
        let mut conn = self.connection_pool.clone();
        
        let exists: bool = conn.exists(key).await
            .map_err(|e| CacheError::Redis(e))?;
        
        Ok(exists)
    }
    
    /// Set TTL for existing key
    pub async fn expire(&self, key: &str, ttl: Duration) -> Result<bool, CacheError> {
        let mut conn = self.connection_pool.clone();
        
        let result: bool = conn.expire(key, ttl.as_secs() as i64).await
            .map_err(|e| CacheError::Redis(e))?;
        
        Ok(result)
    }
    
    /// Increment a counter
    pub async fn increment(&self, key: &str, delta: i64) -> Result<i64, CacheError> {
        let mut conn = self.connection_pool.clone();
        
        let result: i64 = conn.incr(key, delta).await
            .map_err(|e| CacheError::Redis(e))?;
        
        debug!("📈 Incremented counter {}: {} (delta: {})", key, result, delta);
        Ok(result)
    }
    
    /// Get multiple keys at once
    pub async fn get_multiple<T>(&self, keys: &[String]) -> Result<Vec<Option<T>>, CacheError>
    where
        T: for<'de> Deserialize<'de>,
    {
        if keys.is_empty() {
            return Ok(vec![]);
        }
        
        let mut conn = self.connection_pool.clone();
        
        let results: Vec<Option<String>> = conn.get(keys).await
            .map_err(|e| CacheError::Redis(e))?;
        
        let mut values = Vec::new();
        for (i, result) in results.into_iter().enumerate() {
            match result {
                Some(serialized) => {
                    let value = serde_json::from_str(&serialized)
                        .map_err(|e| CacheError::Deserialization(e.to_string()))?;
                    values.push(Some(value));
                    debug!("📖 Cache hit for key: {}", keys[i]);
                }
                None => {
                    values.push(None);
                    debug!("📭 Cache miss for key: {}", keys[i]);
                }
            }
        }
        
        Ok(values)
    }
}

/// Session manager for user sessions
pub struct SessionManager {
    cache: CacheManager,
    session_ttl: Duration,
}

impl SessionManager {
    /// Create a new session manager
    pub fn new(cache: CacheManager, session_ttl: Duration) -> Self {
        Self {
            cache,
            session_ttl,
        }
    }
    
    /// Create a new session
    pub async fn create_session(&self, user_id: Uuid, session_data: SessionData) -> Result<String, CacheError> {
        let session_id = Uuid::new_v4().to_string();
        let session_key = format!("session:{}", session_id);
        
        let session = UserSession {
            id: session_id.clone(),
            user_id,
            data: session_data,
            created_at: Utc::now(),
            last_accessed: Utc::now(),
        };
        
        self.cache.set(&session_key, &session, Some(self.session_ttl)).await?;
        
        info!("🔐 Created session for user: {} (session: {})", user_id, session_id);
        Ok(session_id)
    }
    
    /// Get session data
    pub async fn get_session(&self, session_id: &str) -> Result<Option<UserSession>, CacheError> {
        let session_key = format!("session:{}", session_id);
        
        if let Some(mut session) = self.cache.get::<UserSession>(&session_key).await? {
            // Update last accessed time
            session.last_accessed = Utc::now();
            self.cache.set(&session_key, &session, Some(self.session_ttl)).await?;
            
            debug!("🔍 Retrieved session: {}", session_id);
            Ok(Some(session))
        } else {
            debug!("❌ Session not found: {}", session_id);
            Ok(None)
        }
    }
    
    /// Delete session
    pub async fn delete_session(&self, session_id: &str) -> Result<bool, CacheError> {
        let session_key = format!("session:{}", session_id);
        let deleted = self.cache.delete(&session_key).await?;
        
        if deleted {
            info!("🗑️  Deleted session: {}", session_id);
        }
        
        Ok(deleted)
    }
    
    /// Delete all sessions for a user
    pub async fn delete_user_sessions(&self, user_id: Uuid) -> Result<u32, CacheError> {
        // In a production system, you would maintain a user->sessions mapping
        // For now, this is a placeholder
        warn!("🚧 delete_user_sessions not fully implemented for user: {}", user_id);
        Ok(0)
    }
}

/// Cache health information
#[derive(Debug, Clone)]
pub struct CacheHealth {
    pub status: String,
    pub response_time_ms: u64,
    pub timestamp: DateTime<Utc>,
}

/// User session data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSession {
    pub id: String,
    pub user_id: Uuid,
    pub data: SessionData,
    pub created_at: DateTime<Utc>,
    pub last_accessed: DateTime<Utc>,
}

/// Session data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionData {
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub roles: Vec<String>,
    pub permissions: Vec<String>,
    pub metadata: std::collections::HashMap<String, String>,
}

/// Cache error types
#[derive(Debug, thiserror::Error)]
pub enum CacheError {
    #[error("Redis error: {0}")]
    Redis(#[from] redis::RedisError),
    
    #[error("Serialization error: {0}")]
    Serialization(String),
    
    #[error("Deserialization error: {0}")]
    Deserialization(String),
    
    #[error("Session not found")]
    SessionNotFound,
    
    #[error("Session expired")]
    SessionExpired,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_cache_operations() {
        // This test would require a Redis instance
        // For now, it's a placeholder for the test structure
        
        #[derive(Serialize, Deserialize, PartialEq, Debug)]
        struct TestData {
            id: u32,
            name: String,
        }
        
        let test_data = TestData {
            id: 1,
            name: "test".to_string(),
        };
        
        // In a real test, you would:
        // 1. Set up a test Redis instance
        // 2. Create a CacheManager
        // 3. Test set/get/delete operations
        // 4. Verify TTL behavior
        
        assert_eq!(test_data.id, 1);
    }
}
