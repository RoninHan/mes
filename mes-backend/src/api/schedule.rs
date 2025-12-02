use crate::api::ApiContext;
use crate::db::dao;
use crate::model::schedule::*;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use chrono::{DateTime, Utc};
use sea_orm::ActiveValue::Set;

pub fn router() -> axum::Router<ApiContext> {
    use axum::routing::{delete, get, post, put};

    axum::Router::new()
        .route("/schedule/timeline", get(get_timeline))
        .route("/schedule", post(create_schedule))
        .route("/schedule/:id", put(update_schedule).delete(delete_schedule))
        .route("/schedule/run", post(run_auto_schedule))
}

async fn get_timeline(
    State(ctx): State<ApiContext>,
    Query(q): Query<TimelineQuery>,
) -> Result<Json<PageResult<TimelineItemDto>>, StatusCode> {
    let filter = dao::production_schedule_dao::ScheduleFilter {
        workshop_id: q.workshop_id,
        equipment_id: q.equipment_id,
        from: q.from,
        to: q.to,
    };

    let (schedules, total) =
        dao::production_schedule_dao::list_by_window(ctx.db.conn(), filter, q.page, q.page_size)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // preload work orders
    let wo_ids: Vec<i64> = schedules.iter().map(|s| s.work_order_id).collect();
    let work_orders = crate::db::entity::work_orders::Entity::find()
        .filter(crate::db::entity::work_orders::Column::Id.is_in(wo_ids.clone()))
        .all(ctx.db.conn())
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    use std::collections::HashMap;
    let mut wo_map: HashMap<i64, crate::db::entity::work_orders::Model> = HashMap::new();
    for wo in work_orders {
        wo_map.insert(wo.id, wo);
    }

    let items = schedules
        .into_iter()
        .filter_map(|s| {
            let wo = wo_map.get(&s.work_order_id)?;
            Some(TimelineItemDto {
                id: s.id,
                work_order_id: s.work_order_id,
                work_order_no: wo.work_order_no.clone(),
                material_id: wo.material_id,
                equipment_id: s.equipment_id,
                workshop_id: s.workshop_id,
                start_time: DateTime::<Utc>::from(s.start_time),
                end_time: DateTime::<Utc>::from(s.end_time),
                status: s.status,
                priority: s.priority,
            })
        })
        .collect();

    Ok(Json(PageResult { items, total }))
}

async fn create_schedule(
    State(ctx): State<ApiContext>,
    Json(body): Json<ScheduleCreateRequest>,
) -> Result<Json<TimelineItemDto>, StatusCode> {
    let active = crate::db::entity::production_schedules::ActiveModel {
        work_order_id: Set(body.work_order_id),
        equipment_id: Set(body.equipment_id),
        workshop_id: Set(body.workshop_id),
        start_time: Set(body.start_time.into()),
        end_time: Set(body.end_time.into()),
        status: Set(body.status.unwrap_or(1)),
        priority: Set(body.priority.unwrap_or(3)),
        remark: Set(body.remark),
        ..Default::default()
    };

    let s = dao::production_schedule_dao::create(ctx.db.conn(), active)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let Some(wo) = crate::db::entity::work_orders::Entity::find_by_id(s.work_order_id)
        .one(ctx.db.conn())
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    else {
        return Err(StatusCode::BAD_REQUEST);
    };

    Ok(Json(TimelineItemDto {
        id: s.id,
        work_order_id: s.work_order_id,
        work_order_no: wo.work_order_no,
        material_id: wo.material_id,
        equipment_id: s.equipment_id,
        workshop_id: s.workshop_id,
        start_time: DateTime::<Utc>::from(s.start_time),
        end_time: DateTime::<Utc>::from(s.end_time),
        status: s.status,
        priority: s.priority,
    }))
}

async fn update_schedule(
    State(ctx): State<ApiContext>,
    Path(id): Path<i32>,
    Json(body): Json<ScheduleUpdateRequest>,
) -> Result<Json<TimelineItemDto>, StatusCode> {
    let Some(existing) = dao::production_schedule_dao::get_by_id(ctx.db.conn(), id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    else {
        return Err(StatusCode::NOT_FOUND);
    };

    let mut active = crate::db::entity::production_schedules::ActiveModel {
        id: Set(existing.id),
        ..Default::default()
    };
    if let Some(eq) = body.equipment_id {
        active.equipment_id = Set(Some(eq));
    }
    if let Some(ws) = body.workshop_id {
        active.workshop_id = Set(Some(ws));
    }
    if let Some(st) = body.start_time {
        active.start_time = Set(st.into());
    }
    if let Some(et) = body.end_time {
        active.end_time = Set(et.into());
    }
    if let Some(status) = body.status {
        active.status = Set(status);
    }
    if let Some(priority) = body.priority {
        active.priority = Set(priority);
    }
    if let Some(r) = body.remark {
        active.remark = Set(Some(r));
    }

    let Some(s) = dao::production_schedule_dao::update(ctx.db.conn(), id, active)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    else {
        return Err(StatusCode::NOT_FOUND);
    };

    let Some(wo) = crate::db::entity::work_orders::Entity::find_by_id(s.work_order_id)
        .one(ctx.db.conn())
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    else {
        return Err(StatusCode::BAD_REQUEST);
    };

    Ok(Json(TimelineItemDto {
        id: s.id,
        work_order_id: s.work_order_id,
        work_order_no: wo.work_order_no,
        material_id: wo.material_id,
        equipment_id: s.equipment_id,
        workshop_id: s.workshop_id,
        start_time: DateTime::<Utc>::from(s.start_time),
        end_time: DateTime::<Utc>::from(s.end_time),
        status: s.status,
        priority: s.priority,
    }))
}

async fn delete_schedule(
    State(ctx): State<ApiContext>,
    Path(id): Path<i32>,
) -> Result<StatusCode, StatusCode> {
    let rows = dao::production_schedule_dao::delete(ctx.db.conn(), id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    if rows == 0 {
        return Err(StatusCode::NOT_FOUND);
    }
    Ok(StatusCode::NO_CONTENT)
}

#[derive(Debug, Deserialize)]
struct AutoScheduleRequest {
    pub workshop_id: Option<i64>,
    pub equipment_id: Option<i64>,
    pub from: DateTime<Utc>,
}

use serde::Deserialize;

async fn run_auto_schedule(
    State(ctx): State<ApiContext>,
    Json(body): Json<AutoScheduleRequest>,
) -> Result<StatusCode, StatusCode> {
    use crate::db::entity::work_orders;
    use sea_orm::EntityTrait;

    // 简单算法示例：按计划开始时间，对未排程工单顺序平铺，每个假设持续 2 小时
    let mut q = work_orders::Entity::find();
    if let Some(ws) = body.workshop_id {
        q = q.filter(work_orders::Column::WorkshopId.eq(ws));
    }
    let work_orders = q
        .filter(work_orders::Column::IsDeleted.eq(0))
        .order_by_asc(work_orders::Column::PlanStartTime)
        .all(ctx.db.conn())
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut current_start = body.from;

    for wo in work_orders {
        // 检查是否已有排程
        let existing = entity::ProductionSchedules::find()
            .filter(production_schedules::Column::WorkOrderId.eq(wo.id))
            .one(ctx.db.conn())
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        if existing.is_some() {
            continue;
        }

        let duration = chrono::Duration::hours(2);
        let start_time = current_start;
        let end_time = start_time + duration;
        current_start = end_time;

        let active = crate::db::entity::production_schedules::ActiveModel {
            work_order_id: Set(wo.id),
            equipment_id: Set(body.equipment_id),
            workshop_id: Set(wo.workshop_id),
            start_time: Set(start_time.into()),
            end_time: Set(end_time.into()),
            status: Set(1),
            priority: Set(wo.work_order_status.unwrap_or(3)),
            remark: Set(Some("auto scheduled".to_string())),
            ..Default::default()
        };
        let _ = dao::production_schedule_dao::create(ctx.db.conn(), active)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }

    Ok(StatusCode::NO_CONTENT)
}


