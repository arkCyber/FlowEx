//! FlowEx Authentication Library
//!
//! Enterprise-grade authentication utilities including JWT token management,
//! password hashing, session management, and security features.

use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::{Duration, Utc};
use flowex_types::{JwtClaims, User, Role, Permission, FlowExError, FlowExResult};
use jsonwebtoken::{encode, decode, EncodingKey, DecodingKey, Header, Validation, Algorithm};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use tracing::{info, warn, error, debug};
use uuid::Uuid;

/// JWT token manager for FlowEx authentication
#[derive(Clone)]
pub struct JwtManager {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
    issuer: String,
    audience: String,
    expiration_hours: i64,
    refresh_expiration_days: i64,
}

impl JwtManager {
    /// Create a new JWT manager
    pub fn new(
        secret: &str,
        issuer: String,
        audience: String,
        expiration_hours: i64,
        refresh_expiration_days: i64,
    ) -> Self {
        let encoding_key = EncodingKey::from_secret(secret.as_ref());
        let decoding_key = DecodingKey::from_secret(secret.as_ref());

        Self {
            encoding_key,
            decoding_key,
            issuer,
            audience,
            expiration_hours,
            refresh_expiration_days,
        }
    }

    /// Generate JWT token for user
    pub fn generate_token(&self, user: &User, roles: Vec<String>) -> FlowExResult<String> {
        let now = Utc::now();
        let exp = now + Duration::hours(self.expiration_hours);
        
        // Get permissions based on roles
        let permissions = self.get_permissions_for_roles(&roles);

        let claims = JwtClaims {
            sub: user.id.to_string(),
            email: user.email.clone(),
            exp: exp.timestamp() as usize,
            iat: now.timestamp() as usize,
            jti: Uuid::new_v4().to_string(),
            roles,
            permissions,
        };

        let header = Header::new(Algorithm::HS256);
        
        encode(&header, &claims, &self.encoding_key)
            .map_err(|e| FlowExError::Authentication(format!("Failed to generate token: {}", e)))
    }

    /// Generate refresh token
    pub fn generate_refresh_token(&self, user: &User) -> FlowExResult<String> {
        let now = Utc::now();
        let exp = now + Duration::days(self.refresh_expiration_days);

        let claims = RefreshTokenClaims {
            sub: user.id.to_string(),
            email: user.email.clone(),
            exp: exp.timestamp() as usize,
            iat: now.timestamp() as usize,
            jti: Uuid::new_v4().to_string(),
            token_type: "refresh".to_string(),
        };

        let header = Header::new(Algorithm::HS256);
        
        encode(&header, &claims, &self.encoding_key)
            .map_err(|e| FlowExError::Authentication(format!("Failed to generate refresh token: {}", e)))
    }

    /// Validate and decode JWT token
    pub fn validate_token(&self, token: &str) -> FlowExResult<JwtClaims> {
        let mut validation = Validation::new(Algorithm::HS256);
        validation.set_issuer(&[&self.issuer]);
        validation.set_audience(&[&self.audience]);
        validation.validate_exp = true;
        validation.validate_nbf = true;
        validation.leeway = 60; // 60 seconds leeway for clock skew

        let token_data = decode::<JwtClaims>(token, &self.decoding_key, &validation)
            .map_err(|e| {
                warn!("JWT validation failed: {}", e);
                FlowExError::Authentication("Invalid or expired token".to_string())
            })?;

        Ok(token_data.claims)
    }

    /// Validate refresh token
    pub fn validate_refresh_token(&self, token: &str) -> FlowExResult<RefreshTokenClaims> {
        let mut validation = Validation::new(Algorithm::HS256);
        validation.set_issuer(&[&self.issuer]);
        validation.set_audience(&[&self.audience]);
        validation.validate_exp = true;

        let token_data = decode::<RefreshTokenClaims>(token, &self.decoding_key, &validation)
            .map_err(|e| {
                warn!("Refresh token validation failed: {}", e);
                FlowExError::Authentication("Invalid or expired refresh token".to_string())
            })?;

        Ok(token_data.claims)
    }

    /// Get permissions for roles
    fn get_permissions_for_roles(&self, roles: &[String]) -> Vec<String> {
        let mut permissions = HashSet::new();

        for role_str in roles {
            if let Ok(role) = role_str.parse::<Role>() {
                for permission in role.permissions() {
                    permissions.insert(permission.as_str().to_string());
                }
            }
        }

        permissions.into_iter().collect()
    }
}

/// Refresh token claims
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefreshTokenClaims {
    pub sub: String,        // Subject (user ID)
    pub email: String,      // User email
    pub exp: usize,         // Expiration time
    pub iat: usize,         // Issued at
    pub jti: String,        // JWT ID
    pub token_type: String, // Token type
}

/// Password manager for secure password operations
pub struct PasswordManager {
    cost: u32,
}

impl PasswordManager {
    /// Create a new password manager
    pub fn new(cost: Option<u32>) -> Self {
        Self {
            cost: cost.unwrap_or(DEFAULT_COST),
        }
    }

    /// Hash a password
    pub fn hash_password(&self, password: &str) -> FlowExResult<String> {
        // Validate password strength
        self.validate_password_strength(password)?;

        hash(password, self.cost)
            .map_err(|e| FlowExError::Authentication(format!("Failed to hash password: {}", e)))
    }

    /// Verify a password against a hash
    pub fn verify_password(&self, password: &str, hash: &str) -> FlowExResult<bool> {
        verify(password, hash)
            .map_err(|e| FlowExError::Authentication(format!("Failed to verify password: {}", e)))
    }

    /// Validate password strength
    fn validate_password_strength(&self, password: &str) -> FlowExResult<()> {
        if password.len() < 8 {
            return Err(FlowExError::Validation("Password must be at least 8 characters long".to_string()));
        }

        let has_uppercase = password.chars().any(|c| c.is_uppercase());
        let has_lowercase = password.chars().any(|c| c.is_lowercase());
        let has_digit = password.chars().any(|c| c.is_ascii_digit());
        let has_special = password.chars().any(|c| "!@#$%^&*()_+-=[]{}|;:,.<>?".contains(c));

        let strength_score = [has_uppercase, has_lowercase, has_digit, has_special]
            .iter()
            .filter(|&&x| x)
            .count();

        if strength_score < 3 {
            return Err(FlowExError::Validation(
                "Password must contain at least 3 of: uppercase, lowercase, digit, special character".to_string()
            ));
        }

        Ok(())
    }
}

/// Session manager for user sessions
#[derive(Clone)]
pub struct SessionManager {
    cache: flowex_cache::CacheManager,
    session_timeout: Duration,
}

impl SessionManager {
    /// Create a new session manager
    pub fn new(cache: flowex_cache::CacheManager, session_timeout_hours: i64) -> Self {
        Self {
            cache,
            session_timeout: Duration::hours(session_timeout_hours),
        }
    }

    /// Create a new session
    pub async fn create_session(&self, user_id: Uuid, token_id: &str) -> FlowExResult<()> {
        let session_key = format!("session:{}", token_id);
        let session_data = SessionData {
            user_id,
            created_at: Utc::now(),
            last_accessed: Utc::now(),
        };

        self.cache
            .set(&session_key, &session_data, Some(self.session_timeout.to_std().unwrap()))
            .await
            .map_err(|e| FlowExError::Internal(format!("Failed to create session: {}", e)))?;

        debug!("Created session for user: {}", user_id);
        Ok(())
    }

    /// Validate session
    pub async fn validate_session(&self, token_id: &str) -> FlowExResult<SessionData> {
        let session_key = format!("session:{}", token_id);
        
        let session_data: Option<SessionData> = self.cache
            .get(&session_key)
            .await
            .map_err(|e| FlowExError::Internal(format!("Failed to validate session: {}", e)))?;

        match session_data {
            Some(mut data) => {
                // Update last accessed time
                data.last_accessed = Utc::now();
                self.cache
                    .set(&session_key, &data, Some(self.session_timeout.to_std().unwrap()))
                    .await
                    .map_err(|e| FlowExError::Internal(format!("Failed to update session: {}", e)))?;

                Ok(data)
            }
            None => Err(FlowExError::Authentication("Session not found or expired".to_string())),
        }
    }

    /// Revoke session
    pub async fn revoke_session(&self, token_id: &str) -> FlowExResult<()> {
        let session_key = format!("session:{}", token_id);
        
        self.cache
            .delete(&session_key)
            .await
            .map_err(|e| FlowExError::Internal(format!("Failed to revoke session: {}", e)))?;

        debug!("Revoked session: {}", token_id);
        Ok(())
    }
}

/// Session data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionData {
    pub user_id: Uuid,
    pub created_at: chrono::DateTime<Utc>,
    pub last_accessed: chrono::DateTime<Utc>,
}

/// Role parsing implementation
impl std::str::FromStr for Role {
    type Err = FlowExError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "user" => Ok(Role::User),
            "trader" => Ok(Role::Trader),
            "vip_trader" => Ok(Role::VipTrader),
            "admin" => Ok(Role::Admin),
            "super_admin" => Ok(Role::SuperAdmin),
            "system" => Ok(Role::System),
            _ => Err(FlowExError::Validation(format!("Invalid role: {}", s))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_hashing() {
        let password_manager = PasswordManager::new(Some(4)); // Lower cost for testing
        let password = "TestPassword123!";
        
        let hash = password_manager.hash_password(password).unwrap();
        assert!(password_manager.verify_password(password, &hash).unwrap());
        assert!(!password_manager.verify_password("wrong_password", &hash).unwrap());
    }

    #[test]
    fn test_password_strength_validation() {
        let password_manager = PasswordManager::new(None);
        
        // Too short
        assert!(password_manager.hash_password("short").is_err());
        
        // Weak password
        assert!(password_manager.hash_password("password").is_err());
        
        // Strong password
        assert!(password_manager.hash_password("StrongPass123!").is_ok());
    }

    #[tokio::test]
    async fn test_jwt_token_generation() {
        let jwt_manager = JwtManager::new(
            "test_secret",
            "flowex".to_string(),
            "flowex-users".to_string(),
            24,
            30,
        );

        let user = User {
            id: Uuid::new_v4(),
            email: "test@example.com".to_string(),
            first_name: "Test".to_string(),
            last_name: "User".to_string(),
            is_verified: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let roles = vec!["trader".to_string()];
        let token = jwt_manager.generate_token(&user, roles).unwrap();
        
        let claims = jwt_manager.validate_token(&token).unwrap();
        assert_eq!(claims.email, user.email);
        assert_eq!(claims.sub, user.id.to_string());
    }
}
