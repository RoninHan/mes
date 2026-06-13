//! 系统管理接口（审计日志 + 关联应用地址配置）
//!
//! 注意：用户管理已统一由 SSO 负责，本模块不再提供用户 CRUD。
//! 仅保留 MES 自己产生的登录日志和操作审计日志，以及关联系统地址配置。

use std::sync::{Mutex, OnceLock};

use crate::api::ApiContext;
use crate::db::dao;
use crate::model::system::*;
use axum::{
    extract::{Query, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};

// ── 关联应用地址配置（内存，重启后重置，下次可从数据库扩展）─────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppLinksConfig {
    /// ERP 前端地址
    pub erp_url: String,
    /// SSO 认证中心地址
    pub sso_url: String,
}

static APP_LINKS: OnceLock<Mutex<AppLinksConfig>> = OnceLock::new();

fn get_app_links() -> &'static Mutex<AppLinksConfig> {
    APP_LINKS.get_or_init(|| {
        Mutex::new(AppLinksConfig {
            erp_url: std::env::var("ERP_URL")
                .unwrap_or_else(|_| "http://localhost:3000".to_string()),
            sso_url: std::env::var("SSO_URL")
                .unwrap_or_else(|_| "http://localhost:3001".to_string()),
        })
    })
}

#[derive(Debug, Deserialize)]
pub struct UpdateAppLinksRequest {
    pub erp_url: Option<String>,
    pub sso_url: Option<String>,
}

pub fn router() -> axum::Router<ApiContext> {
    use axum::routing::{get, put};

    axum::Router::new()
        // ── 关联系统地址（GET 无需额外鉴权，已由外层 SSO 中间件保护）──────
        .route("/system/app-links", get(get_app_links_handler).put(update_app_links))
        // ── 审计日志 ─────────────────────────────────────────────────────────
        .route("/system/login-logs", get(list_login_logs))
        .route("/system/operation-logs", get(list_operation_logs))
}

async fn get_app_links_handler() -> Result<Json<AppLinksConfig>, StatusCode> {
    let cfg = get_app_links()
        .lock()
        .map(|g| g.clone())
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(cfg))
}

async fn update_app_links(
    Json(req): Json<UpdateAppLinksRequest>,
) -> Result<Json<AppLinksConfig>, StatusCode> {
    let mut cfg = get_app_links()
        .lock()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    if let Some(v) = req.erp_url {
        if !v.trim().is_empty() {
            cfg.erp_url = v.trim().to_string();
        }
    }
    if let Some(v) = req.sso_url {
        if !v.trim().is_empty() {
            cfg.sso_url = v.trim().to_string();
        }
    }
    Ok(Json(cfg.clone()))
}

// ── 登录日志 ──────────────────────────────────────────────────────────────────

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

// ── 操作审计日志 ──────────────────────────────────────────────────────────────

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
