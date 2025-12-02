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
    let Some(user) = dao::user_dao::find_by_username(ctx.db.conn(), &body.username)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    // 简化：暂不校验密码 hash，直接使用已有加密串占位
    // 实际使用时，可用 argon2::PasswordVerifier 校验 body.password
    let exp = (Utc::now() + Duration::hours(8)).timestamp() as usize;
    let claims = Claims {
        sub: user.id,
        username: user.username.clone(),
        roles: vec![],
        exp,
    };

    let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "secret".to_string());
    let token = encode_token(&claims, &secret).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(LoginResponse {
        token,
        user_id: user.id,
        username: user.username,
    }))
}


