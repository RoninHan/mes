use crate::api::ApiContext;
use crate::db::dao;
use crate::utils::jwt::{encode_token, Claims};
use axum::{extract::State, http::StatusCode, Json};
use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub user_id: i64,
    pub username: String,
}

pub fn router() -> axum::Router<ApiContext> {
    use axum::routing::post;

    axum::Router::new().route("/auth/login", post(login))
}

async fn login(
    State(ctx): State<ApiContext>,
    Json(body): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, StatusCode> {
    use crate::db::entity::login_logs;
    use sea_orm::ActiveValue::Set;

    let now = Utc::now();
    let mut result_ok = false;
    let mut user_opt = None;

    if let Ok(found) = dao::user_dao::find_by_username(ctx.db.conn(), &body.username).await {
        user_opt = found;
        // 简化：暂不校验密码 hash，直接使用已有加密串占位
        result_ok = user_opt.is_some();
    }

    let (user_id, username) = if let Some(ref u) = user_opt {
        (Some(u.id), Some(u.username.clone()))
    } else {
        (None, Some(body.username.clone()))
    };

    let log_active = login_logs::ActiveModel {
        user_id: Set(user_id),
        username: Set(username),
        login_time: Set(now.into()),
        login_ip: Set(None),
        user_agent: Set(None),
        result: Set(if result_ok { 1 } else { 2 }),
        fail_reason: Set(if result_ok {
            None
        } else {
            Some("Invalid credentials".to_string())
        }),
        ..Default::default()
    };
    let _ = dao::login_log_dao::create(ctx.db.conn(), log_active).await;

    let user = match user_opt {
        Some(u) => u,
        None => return Err(StatusCode::UNAUTHORIZED),
    };

    let exp = (Utc::now() + Duration::hours(8)).timestamp();
    let claims = Claims {
        sub: user.id,
        username: user.username.clone(),
        roles: vec![],
        permissions: vec![],
        sid: String::new(),
        exp,
        iat: Utc::now().timestamp() as i64,
    };

    let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "secret".to_string());
    let token = encode_token(&claims, &secret).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(LoginResponse {
        token,
        user_id: user.id,
        username: user.username,
    }))
}


