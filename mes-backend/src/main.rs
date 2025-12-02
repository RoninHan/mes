mod config;
mod db;
mod cache;
mod middleware;
mod model;
mod api;
mod mqtt;
mod service;
mod utils;

use axum::{routing::get, Router};
use std::net::SocketAddr;
use tracing_subscriber::EnvFilter;

use crate::config::AppConfig;
use crate::db::Database;
use crate::cache::RedisCache;
use crate::mqtt::manager::MqttClientManager;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let config = AppConfig::from_env()?;
    let db = Database::connect(&config.database_url).await?;
    let cache = RedisCache::connect(&config.redis_url).await?;

    // Initialize MQTT client manager (will load equipment configs later)
    let mqtt_manager = MqttClientManager::new(config.mqtt.default_broker.clone());

    let api_router = api::create_router(db.clone(), cache.clone(), mqtt_manager.clone(), config.clone());

    let app = Router::new()
        .route("/health", get(|| async { "OK" }))
        .nest("/api", api_router);

    let addr: SocketAddr = config.server_addr.parse()?;
    tracing::info!("Starting MES backend on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}


