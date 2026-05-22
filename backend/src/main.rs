mod config;
mod db;
mod models;
mod auth;
mod routes;

use std::sync::Arc;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load .env file (silently skip if missing)
    let _ = dotenvy::dotenv();

    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| "info,vuxe_backend=debug".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration
    let config = Arc::new(config::Config::from_env()?);
    tracing::info!("Starting Vuxe backend with auth provider: {:?}", config.auth_provider);

    // Connect to database
    let pool = db::create_pool(&config.database_url).await?;
    tracing::info!("Connected to Postgres");

    // Run migrations
    db::run_migrations(&pool).await?;
    tracing::info!("Migrations complete");

    // Build router
    let app = routes::build_router(pool, config.clone());

    // Start server
    let addr = format!("{}:{}", config.host, config.port);
    tracing::info!("Listening on {addr}");

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
