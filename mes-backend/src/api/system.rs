use crate::api::ApiContext;
use crate::db::dao;
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

#[derive(Debug, Serialize)]
pub struct PageResult<T> {
    pub items: Vec<T>,
    pub total: u64,
}

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


