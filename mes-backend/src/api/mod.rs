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
pub mod quality;

#[derive(Clone)]
pub struct ApiContext {
    pub db: Database,
    pub cache: RedisCache,
    pub mqtt: Option<MqttClientManager>,
}

pub fn create_router(
    db: Database,
    cache: RedisCache,
    mqtt: Option<MqttClientManager>,
    _config: crate::config::AppConfig,
) -> Router {
    let ctx = ApiContext { db, cache, mqtt };

    Router::new()
        .route("/ping", get(|| async { "api ok" }))
        // /auth/login 已移除（认证由 SSO 统一处理）
        .nest("/", equipment::router())
        .nest("/", master::router())
        .nest("/", production::router())
        .nest("/", warehouse::router())
        .nest("/", schedule::router())
        .nest("/", quality::router())
        // 系统审计日志（用户管理由 SSO 统一处理）
        .nest("/", system::router())
        .with_state(ctx)
}



