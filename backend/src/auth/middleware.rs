use axum::{
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    Json,
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::Serialize;

use super::session::SessionClaims;

/// Extracted user identity from a valid session token.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SessionUser {
    pub user_id: String,
    pub external_id: String,
}

#[allow(dead_code)]
#[derive(Serialize)]
pub struct AuthError {
    pub error: String,
}

/// Axum extractor that verifies the session JWT from the `Authorization: Bearer <token>`
/// header and returns a `SessionUser`.
///
/// Usage in a handler:
/// ```ignore
/// async fn protected_route(user: SessionUser) -> impl IntoResponse { ... }
/// ```
impl<S> FromRequestParts<S> for SessionUser
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, Json<AuthError>);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get("Authorization")
            .and_then(|v| v.to_str().ok())
            .ok_or_else(|| {
                (
                    StatusCode::UNAUTHORIZED,
                    Json(AuthError {
                        error: "Missing Authorization header".into(),
                    }),
                )
            })?;

        let token = auth_header.strip_prefix("Bearer ").ok_or_else(|| {
            (
                StatusCode::UNAUTHORIZED,
                Json(AuthError {
                    error: "Authorization header must start with 'Bearer '".into(),
                }),
            )
        })?;

        let secret = std::env::var("JWT_SECRET")
            .unwrap_or_else(|_| "vuxe-dev-secret-change-in-production".into());

        let token_data = decode::<SessionClaims>(
            token,
            &DecodingKey::from_secret(secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(|e| {
            (
                StatusCode::UNAUTHORIZED,
                Json(AuthError {
                    error: format!("Invalid session token: {e}"),
                }),
            )
        })?;

        Ok(SessionUser {
            user_id: token_data.claims.sub,
            external_id: token_data.claims.ext,
        })
    }
}
