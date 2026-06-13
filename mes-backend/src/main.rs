use axum::{
    extract::Request,
    http::{header, StatusCode},
    middleware::Next,
    response::{IntoResponse, Json, Response},
    routing::get,
    Router,
};
use dotenvy::dotenv;
use serde_json::json;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

mod api;
mod cache;
mod config;
mod db;
mod middleware;
mod model;
mod mqtt;
mod service;
mod utils;
use crate::mqtt::manager::MqttClientManager;

/// SSO JWT 认证中间件：验证 Authorization: Bearer <sso_token>
async fn sso_auth_middleware(req: Request, next: Next) -> Response {
    let token = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "));

    let token = match token {
        Some(t) => t.to_string(),
        None => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({"code": 401, "message": "缺少 Authorization Token", "data": null})),
            )
                .into_response();
        }
    };

    match utils::jwt::decode_token(&token) {
        Ok(_claims) => next.run(req).await,
        Err(_) => (
            StatusCode::UNAUTHORIZED,
            Json(json!({"code": 401, "message": "Token 无效或已过期，请重新登录", "data": null})),
        )
            .into_response(),
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("MES Backend starting...");

    let cfg = config::AppConfig::from_env()?;

    let db = db::Database::connect(&cfg.database_url).await?;
    tracing::info!("数据库连接成功");

    let cache = cache::RedisCache::connect(&cfg.redis_url).await?;
    tracing::info!("Redis 连接成功");

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let mqtt_manager = MqttClientManager::new(cfg.mqtt.default_broker.clone());

    // 业务路由（需 SSO JWT 认证）
    let api_router = api::create_router(db, cache, Some(mqtt_manager), cfg.clone())
        .layer(axum::middleware::from_fn(sso_auth_middleware));

    let app = Router::new()
        .route("/health", get(|| async { "MES Backend is running!" }))
        .nest("/api", api_router)
        .layer(cors);

    let listener = TcpListener::bind(&cfg.server_addr).await?;
    tracing::info!("MES 后端已启动: http://{}", cfg.server_addr);
    axum::serve(listener, app).await?;

    Ok(())
}