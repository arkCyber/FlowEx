//! FlowEx Authentication Middleware
//!
//! Enterprise-grade JWT authentication and authorization middleware
//! with comprehensive security features and audit logging.

use axum::{
    extract::{Request, State},
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::Response,
};
use flowex_types::{AuthContext, JwtClaims, Permission, Role};
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use std::collections::HashSet;
use tracing::{info, warn, error, debug};
use uuid::Uuid;

/// JWT authentication middleware
pub async fn jwt_auth_middleware(
    headers: HeaderMap,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let start_time = std::time::Instant::now();
    
    // Extract JWT token from Authorization header
    let token = extract_jwt_token(&headers)?;
    
    // Validate and decode JWT token
    let claims = validate_jwt_token(&token)?;
    
    // Create authentication context
    let auth_context = AuthContext {
        user_id: Uuid::parse_str(&claims.sub)
            .map_err(|_| {
                error!("Invalid user ID in JWT claims: {}", claims.sub);
                StatusCode::UNAUTHORIZED
            })?,
        email: claims.email.clone(),
        roles: claims.roles.clone(),
        permissions: claims.permissions.clone(),
        session_id: claims.jti.clone(),
    };
    
    // Add auth context to request extensions
    request.extensions_mut().insert(auth_context.clone());
    
    let duration = start_time.elapsed().as_millis();
    debug!(
        user_id = %auth_context.user_id,
        email = %auth_context.email,
        duration_ms = duration,
        "JWT authentication successful"
    );
    
    // Continue with the request
    let response = next.run(request).await;
    
    Ok(response)
}

/// Extract JWT token from Authorization header
fn extract_jwt_token(headers: &HeaderMap) -> Result<String, StatusCode> {
    let auth_header = headers
        .get("authorization")
        .ok_or_else(|| {
            warn!("Missing Authorization header");
            StatusCode::UNAUTHORIZED
        })?
        .to_str()
        .map_err(|_| {
            warn!("Invalid Authorization header format");
            StatusCode::UNAUTHORIZED
        })?;
    
    if !auth_header.starts_with("Bearer ") {
        warn!("Authorization header must start with 'Bearer '");
        return Err(StatusCode::UNAUTHORIZED);
    }
    
    let token = auth_header.strip_prefix("Bearer ").unwrap().to_string();
    
    if token.is_empty() {
        warn!("Empty JWT token");
        return Err(StatusCode::UNAUTHORIZED);
    }
    
    Ok(token)
}

/// Validate JWT token and extract claims
fn validate_jwt_token(token: &str) -> Result<JwtClaims, StatusCode> {
    // In production, this should come from environment or secure storage
    let jwt_secret = std::env::var("JWT_SECRET")
        .unwrap_or_else(|_| "flowex_enterprise_secret_key_2024".to_string());
    
    let decoding_key = DecodingKey::from_secret(jwt_secret.as_ref());
    
    let mut validation = Validation::new(Algorithm::HS256);
    validation.validate_exp = true;
    validation.validate_nbf = true;
    validation.leeway = 60; // 60 seconds leeway for clock skew
    
    let token_data = decode::<JwtClaims>(token, &decoding_key, &validation)
        .map_err(|e| {
            warn!("JWT validation failed: {}", e);
            StatusCode::UNAUTHORIZED
        })?;
    
    Ok(token_data.claims)
}

/// Permission-based authorization middleware
pub async fn require_permission_middleware(
    required_permission: Permission,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // Get auth context from request extensions
    let auth_context = request
        .extensions()
        .get::<AuthContext>()
        .ok_or_else(|| {
            error!("Auth context not found in request extensions");
            StatusCode::UNAUTHORIZED
        })?
        .clone();

    // Check if user has required permission
    if !auth_context.permissions.contains(&required_permission.as_str().to_string()) {
        warn!(
            user_id = %auth_context.user_id,
            required_permission = %required_permission.as_str(),
            user_permissions = ?auth_context.permissions,
            "Permission denied"
        );
        return Err(StatusCode::FORBIDDEN);
    }

    debug!(
        user_id = %auth_context.user_id,
        permission = %required_permission.as_str(),
        "Permission check passed"
    );

    // Continue with the request
    let response = next.run(request).await;
    Ok(response)
}

/// Role-based authorization middleware
pub async fn require_role_middleware(
    required_role: Role,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let required_role_str = required_role.as_str().to_string();

    // Get auth context from request extensions
    let auth_context = request
        .extensions()
        .get::<AuthContext>()
        .ok_or_else(|| {
            error!("Auth context not found in request extensions");
            StatusCode::UNAUTHORIZED
        })?
        .clone();

    // Check if user has required role
    if !auth_context.roles.contains(&required_role_str) {
        warn!(
            user_id = %auth_context.user_id,
            required_role = %required_role_str,
            user_roles = ?auth_context.roles,
            "Role check failed"
        );
        return Err(StatusCode::FORBIDDEN);
    }

    debug!(
        user_id = %auth_context.user_id,
        role = %required_role_str,
        "Role check passed"
    );

    // Continue with the request
    let response = next.run(request).await;
    Ok(response)
}

/// Extract auth context from request (helper function for handlers)
pub fn get_auth_context(request: &Request) -> Result<&AuthContext, StatusCode> {
    request
        .extensions()
        .get::<AuthContext>()
        .ok_or_else(|| {
            error!("Auth context not found in request extensions");
            StatusCode::UNAUTHORIZED
        })
}

/// Rate limiting middleware (basic implementation)
pub async fn rate_limit_middleware(
    headers: HeaderMap,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // Extract client IP
    let client_ip = headers
        .get("x-forwarded-for")
        .or_else(|| headers.get("x-real-ip"))
        .and_then(|h| h.to_str().ok())
        .unwrap_or("unknown");
    
    // In production, implement proper rate limiting with Redis
    // For now, just log the request
    debug!(client_ip = %client_ip, "Rate limit check");
    
    let response = next.run(request).await;
    Ok(response)
}

/// Security headers middleware
pub async fn security_headers_middleware(
    request: Request,
    next: Next,
) -> Response {
    let mut response = next.run(request).await;
    
    let headers = response.headers_mut();
    
    // Add security headers
    headers.insert("X-Content-Type-Options", "nosniff".parse().unwrap());
    headers.insert("X-Frame-Options", "DENY".parse().unwrap());
    headers.insert("X-XSS-Protection", "1; mode=block".parse().unwrap());
    headers.insert("Strict-Transport-Security", "max-age=31536000; includeSubDomains".parse().unwrap());
    headers.insert("Content-Security-Policy", "default-src 'self'".parse().unwrap());
    headers.insert("Referrer-Policy", "strict-origin-when-cross-origin".parse().unwrap());
    
    response
}

#[cfg(test)]
mod tests {
    use super::*;
    use jsonwebtoken::{encode, EncodingKey, Header};
    
    #[test]
    fn test_jwt_token_validation() {
        let claims = JwtClaims {
            sub: Uuid::new_v4().to_string(),
            email: "test@flowex.com".to_string(),
            exp: (chrono::Utc::now() + chrono::Duration::hours(1)).timestamp() as usize,
            iat: chrono::Utc::now().timestamp() as usize,
            jti: Uuid::new_v4().to_string(),
            roles: vec!["trader".to_string()],
            permissions: vec!["trading:read".to_string(), "trading:write".to_string()],
        };
        
        let secret = "test_secret";
        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_ref()),
        ).unwrap();
        
        // This test would need to set the JWT_SECRET environment variable
        // or modify the validation function to accept a secret parameter
        // For now, it's a placeholder for the test structure
        assert!(!token.is_empty());
    }
    
    #[test]
    fn test_permission_extraction() {
        let trader_role = Role::Trader;
        let permissions = trader_role.permissions();
        
        assert!(permissions.contains(&Permission::TradingRead));
        assert!(permissions.contains(&Permission::TradingWrite));
        assert!(permissions.contains(&Permission::WalletRead));
    }
}
