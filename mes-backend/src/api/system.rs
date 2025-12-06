use crate::api::ApiContext;
use crate::db::dao;
use crate::model::system::*;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use sea_orm::ActiveValue::Set;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct UserQuery {
    pub dept_id: Option<i64>,
    pub status: Option<i8>,
    pub keyword: Option<String>,
    #[serde(default)]
    pub page: u64,
    #[serde(default = "default_page_size")]
    pub page_size: u64,
}

fn default_page_size() -> u64 {
    20
}

// UserDto / UserPayload 保持不变

#[derive(Debug, Serialize)]
pub struct UserDto {
    pub id: i64,
    pub username: String,
    pub real_name: String,
    pub dept_id: Option<i64>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub status: i8,
}

#[derive(Debug, Deserialize)]
pub struct UserPayload {
    pub username: String,
    pub password: String,
    pub real_name: String,
    pub dept_id: Option<i64>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub status: Option<i8>,
}

pub fn router() -> axum::Router<ApiContext> {
    use axum::routing::{delete, get, post, put};

    axum::Router::new()
        .route("/system/users", get(list_users).post(create_user))
        .route(
            "/system/users/:id",
            get(get_user).put(update_user).delete(delete_user),
        )
        .route(
            "/system/login-logs",
            get(list_login_logs),
        )
        .route(
            "/system/operation-logs",
            get(list_operation_logs),
        )
}

async fn list_users(
    State(ctx): State<ApiContext>,
    Query(q): Query<UserQuery>,
) -> Result<Json<PageResult<UserDto>>, StatusCode> {
    let filter = dao::user_dao::UserFilter {
        dept_id: q.dept_id,
        status: q.status,
        keyword: q.keyword.clone(),
    };
    let (items, total) = dao::user_dao::list(ctx.db.conn(), filter, q.page, q.page_size)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mapped = items
        .into_iter()
        .map(|u| UserDto {
            id: u.id,
            username: u.username,
            real_name: u.real_name,
            dept_id: u.dept_id,
            email: u.email,
            phone: u.phone,
            status: u.status,
        })
        .collect();

    Ok(Json(PageResult { items: mapped, total }))
}

async fn get_user(
    State(ctx): State<ApiContext>,
    Path(id): Path<i64>,
) -> Result<Json<UserDto>, StatusCode> {
    let Some(u) = dao::user_dao::get_by_id(ctx.db.conn(), id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    else {
        return Err(StatusCode::NOT_FOUND);
    };

    Ok(Json(UserDto {
        id: u.id,
        username: u.username,
        real_name: u.real_name,
        dept_id: u.dept_id,
        email: u.email,
        phone: u.phone,
        status: u.status,
    }))
}

async fn create_user(
    State(ctx): State<ApiContext>,
    Json(body): Json<UserPayload>,
) -> Result<Json<UserDto>, StatusCode> {
    let hashed = body.password; // TODO: hash with argon2
    let active = crate::db::entity::users::ActiveModel {
        username: Set(body.username),
        password: Set(hashed),
        real_name: Set(body.real_name),
        dept_id: Set(body.dept_id),
        email: Set(body.email),
        phone: Set(body.phone),
        status: Set(body.status.unwrap_or(1)),
        is_locked: Set(0),
        ..Default::default()
    };
    let u = dao::user_dao::create(ctx.db.conn(), active)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(UserDto {
        id: u.id,
        username: u.username,
        real_name: u.real_name,
        dept_id: u.dept_id,
        email: u.email,
        phone: u.phone,
        status: u.status,
    }))
}

async fn update_user(
    State(ctx): State<ApiContext>,
    Path(id): Path<i64>,
    Json(body): Json<UserPayload>,
) -> Result<Json<UserDto>, StatusCode> {
    let hashed = body.password; // TODO: hash with argon2
    let active = crate::db::entity::users::ActiveModel {
        username: Set(body.username),
        password: Set(hashed),
        real_name: Set(body.real_name),
        dept_id: Set(body.dept_id),
        email: Set(body.email),
        phone: Set(body.phone),
        status: Set(body.status.unwrap_or(1)),
        ..Default::default()
    };
    let Some(u) = dao::user_dao::update(ctx.db.conn(), id, active)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    else {
        return Err(StatusCode::NOT_FOUND);
    };

    Ok(Json(UserDto {
        id: u.id,
        username: u.username,
        real_name: u.real_name,
        dept_id: u.dept_id,
        email: u.email,
        phone: u.phone,
        status: u.status,
    }))
}

async fn delete_user(
    State(ctx): State<ApiContext>,
    Path(id): Path<i64>,
) -> Result<StatusCode, StatusCode> {
    let rows = dao::user_dao::delete(ctx.db.conn(), id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    if rows == 0 {
        return Err(StatusCode::NOT_FOUND);
    }
    Ok(StatusCode::NO_CONTENT)
}

// ---- Login logs ----

async fn list_login_logs(
    State(ctx): State<ApiContext>,
    Query(q): Query<LoginLogQuery>,
) -> Result<Json<PageResult<LoginLogDto>>, StatusCode> {
    let filter = dao::login_log_dao::LoginLogFilter {
        user_id: q.user_id,
        username: q.username.clone(),
        result: q.result,
    };
    let (items, total) =
        dao::login_log_dao::list(ctx.db.conn(), filter, q.page, q.page_size)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mapped = items
        .into_iter()
        .map(|m| LoginLogDto {
            id: m.id,
            user_id: m.user_id,
            username: m.username,
            login_time: m.login_time.into(),
            login_ip: m.login_ip,
            user_agent: m.user_agent,
            result: m.result,
            fail_reason: m.fail_reason,
        })
        .collect();

    Ok(Json(PageResult { items: mapped, total }))
}

// ---- Operation logs ----

async fn list_operation_logs(
    State(ctx): State<ApiContext>,
    Query(q): Query<OperationLogQuery>,
) -> Result<Json<PageResult<OperationLogDto>>, StatusCode> {
    let filter = dao::operation_log_dao::OperationLogFilter {
        user_id: q.user_id,
        module: q.module.clone(),
        action: q.action.clone(),
        success: q.success,
    };
    let (items, total) =
        dao::operation_log_dao::list(ctx.db.conn(), filter, q.page, q.page_size)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mapped = items
        .into_iter()
        .map(|m| OperationLogDto {
            id: m.id,
            user_id: m.user_id,
            username: m.username,
            module: m.module,
            action: m.action,
            request_path: m.request_path,
            method: m.method,
            request_time: m.request_time.into(),
            success: m.success,
            client_ip: m.client_ip,
            payload: m.payload,
            error_message: m.error_message,
        })
        .collect();

    Ok(Json(PageResult { items: mapped, total }))
}


