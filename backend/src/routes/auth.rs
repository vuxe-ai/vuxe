use axum::{extract::State, http::StatusCode, routing::post, Json, Router};
use chrono::Utc;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

use crate::auth::clerk::ClerkVerifier;
use crate::auth::oidc::OidcVerifier;
use crate::auth::session::SessionClaims;
use crate::config::{AuthProvider, Config};

#[derive(Debug, Deserialize)]
struct LoginRequest {
    token: String,
}

#[derive(Debug, Serialize)]
struct LoginResponse {
    user_id: Uuid,
    external_id: String,
    session_token: String,
}

#[derive(Debug, Serialize)]
struct ErrorResponse {
    error: String,
}

pub fn router(pool: PgPool, config: Arc<Config>) -> Router {
    let state = Arc::new(AuthState { pool, config });
    Router::new()
        .route("/auth/login", post(login))
        .with_state(state)
}

#[derive(Clone)]
struct AuthState {
    pool: PgPool,
    config: Arc<Config>,
}

async fn login(
    State(state): State<Arc<AuthState>>,
    Json(body): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, (StatusCode, Json<ErrorResponse>)> {
    let external_id = match &state.config.auth_provider {
        AuthProvider::Clerk => {
            let clerk_config = state.config.clerk_config.clone().expect("clerk config missing");
            let verifier = ClerkVerifier::new(clerk_config);
            verifier.verify(&body.token).await.map_err(|e| {
                (
                    StatusCode::UNAUTHORIZED,
                    Json(ErrorResponse {
                        error: format!("Clerk verification failed: {e}"),
                    }),
                )
            })?
        }
        AuthProvider::Oidc => {
            let oidc_config = state.config.oidc_config.clone().expect("oidc config missing");
            let verifier = OidcVerifier::new(oidc_config);
            verifier.verify(&body.token).await.map_err(|e| {
                (
                    StatusCode::UNAUTHORIZED,
                    Json(ErrorResponse {
                        error: format!("OIDC verification failed: {e}"),
                    }),
                )
            })?
        }
    };

    let provider_tag = match &state.config.auth_provider {
        AuthProvider::Clerk => "clerk",
        AuthProvider::Oidc => "oidc",
    };

    // Upsert user: find or create
    let user = sqlx::query_as::<_, crate::models::user::User>(
        r#"
        INSERT INTO users (external_id, auth_provider)
        VALUES ($1, $2)
        ON CONFLICT (external_id, auth_provider) DO UPDATE
            SET updated_at = NOW()
        RETURNING id, external_id, auth_provider, created_at, updated_at
        "#,
    )
    .bind(&external_id)
    .bind(provider_tag)
    .fetch_one(&state.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: format!("Database error: {e}"),
            }),
        )
    })?;

    // Generate a session JWT for the client to use on subsequent requests
    let now = Utc::now();
    let session_claims = SessionClaims {
        sub: user.id.to_string(),
        ext: user.external_id.clone(),
        iat: now.timestamp() as usize,
        exp: (now.timestamp() + 86400 * 7) as usize, // 7 days
    };

    let session_token = encode(
        &Header::default(),
        &session_claims,
        &EncodingKey::from_secret(state.config.jwt_secret.as_bytes()),
    )
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: format!("Failed to create session: {e}"),
            }),
        )
    })?;

    Ok(Json(LoginResponse {
        user_id: user.id,
        external_id: user.external_id,
        session_token,
    }))
}
