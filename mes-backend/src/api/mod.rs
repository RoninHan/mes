use crate::cache::RedisCache;
use crate::db::Database;
use crate::mqtt::manager::MqttClientManager;
use axum::routing::get;
use axum::Router;

pub mod equipment;
pub mod master;
pub mod production;
pub mod warehouse;
pub mod schedule;
pub mod auth;
pub mod system;

#[derive(Clone)]
pub struct ApiContext {
    pub db: Database,
    pub cache: RedisCache,
    pub mqtt: MqttClientManager,
}

pub fn create_router(
    db: Database,
    cache: RedisCache,
    mqtt: MqttClientManager,
    _config: crate::config::AppConfig,
) -> Router {
    let ctx = ApiContext { db, cache, mqtt };

    Router::new()
        .route("/ping", get(|| async { "api ok" }))
        .nest("/auth", auth::router())
        .nest("/", equipment::router())
        .nest("/", master::router())
        .nest("/", production::router())
        .nest("/", warehouse::router())
        .nest("/", schedule::router())
        .with_state(ctx)
}



