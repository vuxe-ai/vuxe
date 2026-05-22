pub mod health;
pub mod auth;

use crate::config::Config;
use axum::Router;
use std::sync::Arc;
use sqlx::PgPool;

pub fn build_router(pool: PgPool, config: Arc<Config>) -> Router {
    Router::new()
        .merge(health::router())
        .merge(auth::router(pool, config))
}
