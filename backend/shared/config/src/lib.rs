//! FlowEx Configuration Library
//!
//! Configuration management for FlowEx services.

use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;

/// Base configuration for all FlowEx services
#[derive(Debug, Deserialize, Clone)]
pub struct ServiceConfig {
    pub host: String,
    pub port: u16,
    pub database_url: String,
    pub redis_url: String,
    pub jwt_secret: String,
    pub log_level: String,
}

impl ServiceConfig {
    /// Load configuration from environment and config files
    pub fn load() -> Result<Self, ConfigError> {
        let mut config = Config::builder()
            .add_source(File::with_name("config/default").required(false))
            .add_source(Environment::with_prefix("FLOWEX"))
            .build()?;
        
        config.try_deserialize()
    }
}

impl Default for ServiceConfig {
    fn default() -> Self {
        Self {
            host: "0.0.0.0".to_string(),
            port: 8000,
            database_url: "postgresql://localhost/flowex".to_string(),
            redis_url: "redis://localhost:6379".to_string(),
            jwt_secret: "flowex_secret_key".to_string(),
            log_level: "info".to_string(),
        }
    }
}
