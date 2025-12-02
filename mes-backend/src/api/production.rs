use crate::api::ApiContext;
use crate::db::dao;
use crate::model::production::*;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use rust_decimal::prelude::FromPrimitive;
use sea_orm::prelude::Decimal;
use sea_orm::ActiveValue::Set;

pub fn router() -> axum::Router<ApiContext> {
    use axum::routing::{delete, get, post, put};

    axum::Router::new()
        .route("/production/plans", get(list_plans).post(create_plan))
        .route(
            "/production/plans/:id",
            get(get_plan).put(update_plan).delete(delete_plan),
        )
        .route("/production/orders", get(list_orders).post(create_order))
        .route(
            "/production/orders/:id",
            get(get_order).put(update_order).delete(delete_order),
        )
        .route("/production/work-orders", get(list_work_orders).post(create_work_order))
        .route(
            "/production/work-orders/:id",
            get(get_work_order).put(update_work_order).delete(delete_work_order),
        )
        .route(
            "/production/reports",
            get(list_reports).post(create_report),
        )
        .route(
            "/production/reports/:id",
            get(get_report).put(update_report).delete(delete_report),
        )
}

// ---- Production Plans ----

async fn list_plans(
    State(ctx): State<ApiContext>,
    Query(q): Query<PlanQuery>,
) -> Result<Json<PageResult<PlanDto>>, StatusCode> {
    let filter = dao::production_plan_dao::PlanFilter {
        plan_status: q.plan_status,
        plan_type: q.plan_type,
        keyword: q.keyword.clone(),
    };
    let (items, total) =
        dao::production_plan_dao::list(ctx.db.conn(), filter, q.page, q.page_size)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mapped = items
        .into_iter()
        .map(|m| PlanDto {
            id: m.id,
            plan_no: m.plan_no,
            plan_name: m.plan_name,
            plan_type: m.plan_type,
            plan_start_date: m.plan_start_date,
            plan_end_date: m.plan_end_date,
            plan_status: m.plan_status,
            completion_rate: m.completion_rate.to_f64().unwrap_or(0.0),
        })
        .collect();

    Ok(Json(PageResult { items: mapped, total }))
}

async fn get_plan(
    State(ctx): State<ApiContext>,
    Path(id): Path<i64>,
) -> Result<Json<PlanDto>, StatusCode> {
    let Some(m) = dao::production_plan_dao::get_by_id(ctx.db.conn(), id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    else {
        return Err(StatusCode::NOT_FOUND);
    };
    Ok(Json(PlanDto {
        id: m.id,
        plan_no: m.plan_no,
        plan_name: m.plan_name,
        plan_type: m.plan_type,
        plan_start_date: m.plan_start_date,
        plan_end_date: m.plan_end_date,
        plan_status: m.plan_status,
        completion_rate: m.completion_rate.to_f64().unwrap_or(0.0),
    }))
}

async fn create_plan(
    State(ctx): State<ApiContext>,
    Json(body): Json<PlanPayload>,
) -> Result<Json<PlanDto>, StatusCode> {
    let active = crate::db::entity::production_plans::ActiveModel {
        plan_no: Set(body.plan_no),
        plan_name: Set(body.plan_name),
        plan_type: Set(body.plan_type),
        plan_period: Set(body.plan_period),
        plan_start_date: Set(body.plan_start_date),
        plan_end_date: Set(body.plan_end_date),
        plan_status: Set(body.plan_status.unwrap_or(1)),
        remark: Set(body.remark),
        ..Default::default()
    };
    let m = dao::production_plan_dao::create(ctx.db.conn(), active)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(PlanDto {
        id: m.id,
        plan_no: m.plan_no,
        plan_name: m.plan_name,
        plan_type: m.plan_type,
        plan_start_date: m.plan_start_date,
        plan_end_date: m.plan_end_date,
        plan_status: m.plan_status,
        completion_rate: m.completion_rate.to_f64().unwrap_or(0.0),
    }))
}

async fn update_plan(
    State(ctx): State<ApiContext>,
    Path(id): Path<i64>,
    Json(body): Json<PlanPayload>,
) -> Result<Json<PlanDto>, StatusCode> {
    let active = crate::db::entity::production_plans::ActiveModel {
        plan_no: Set(body.plan_no),
        plan_name: Set(body.plan_name),
        plan_type: Set(body.plan_type),
        plan_period: Set(body.plan_period),
        plan_start_date: Set(body.plan_start_date),
        plan_end_date: Set(body.plan_end_date),
        plan_status: Set(body.plan_status.unwrap_or(1)),
        remark: Set(body.remark),
        ..Default::default()
    };
    let Some(m) = dao::production_plan_dao::update(ctx.db.conn(), id, active)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    else {
        return Err(StatusCode::NOT_FOUND);
    };
    Ok(Json(PlanDto {
        id: m.id,
        plan_no: m.plan_no,
        plan_name: m.plan_name,
        plan_type: m.plan_type,
        plan_start_date: m.plan_start_date,
        plan_end_date: m.plan_end_date,
        plan_status: m.plan_status,
        completion_rate: m.completion_rate.to_f64().unwrap_or(0.0),
    }))
}

async fn delete_plan(
    State(ctx): State<ApiContext>,
    Path(id): Path<i64>,
) -> Result<StatusCode, StatusCode> {
    let rows = dao::production_plan_dao::delete(ctx.db.conn(), id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    if rows == 0 {
        return Err(StatusCode::NOT_FOUND);
    }
    Ok(StatusCode::NO_CONTENT)
}

// ---- Production Orders ----

async fn list_orders(
    State(ctx): State<ApiContext>,
    Query(q): Query<OrderQuery>,
) -> Result<Json<PageResult<OrderDto>>, StatusCode> {
    let filter = dao::production_order_dao::OrderFilter {
        order_status: q.order_status,
        workshop_id: q.workshop_id,
        keyword: q.keyword.clone(),
    };
    let (items, total) =
        dao::production_order_dao::list(ctx.db.conn(), filter, q.page, q.page_size)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mapped = items
        .into_iter()
        .map(|m| OrderDto {
            id: m.id,
            order_no: m.order_no,
            plan_id: m.plan_id,
            material_id: m.material_id,
            plan_quantity: m.plan_quantity.to_f64().unwrap_or(0.0),
            actual_quantity: m.actual_quantity.to_f64().unwrap_or(0.0),
            order_status: m.order_status,
            plan_start_date: m.plan_start_date,
            plan_end_date: m.plan_end_date,
        })
        .collect();

    Ok(Json(PageResult { items: mapped, total }))
}

async fn get_order(
    State(ctx): State<ApiContext>,
    Path(id): Path<i64>,
) -> Result<Json<OrderDto>, StatusCode> {
    let Some(m) = dao::production_order_dao::get_by_id(ctx.db.conn(), id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    else {
        return Err(StatusCode::NOT_FOUND);
    };
    Ok(Json(OrderDto {
        id: m.id,
        order_no: m.order_no,
        plan_id: m.plan_id,
        material_id: m.material_id,
        plan_quantity: m.plan_quantity.to_f64().unwrap_or(0.0),
        actual_quantity: m.actual_quantity.to_f64().unwrap_or(0.0),
        order_status: m.order_status,
        plan_start_date: m.plan_start_date,
        plan_end_date: m.plan_end_date,
    }))
}

async fn create_order(
    State(ctx): State<ApiContext>,
    Json(body): Json<OrderPayload>,
) -> Result<Json<OrderDto>, StatusCode> {
    let active = crate::db::entity::production_orders::ActiveModel {
        order_no: Set(body.order_no),
        plan_id: Set(body.plan_id),
        material_id: Set(body.material_id),
        plan_quantity: Set(Decimal::from_f64(body.plan_quantity).unwrap_or_default()),
        plan_start_date: Set(body.plan_start_date),
        plan_end_date: Set(body.plan_end_date),
        order_status: Set(body.order_status.unwrap_or(1)),
        workshop_id: Set(body.workshop_id),
        remark: Set(body.remark),
        ..Default::default()
    };
    let m = dao::production_order_dao::create(ctx.db.conn(), active)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(OrderDto {
        id: m.id,
        order_no: m.order_no,
        plan_id: m.plan_id,
        material_id: m.material_id,
        plan_quantity: m.plan_quantity.to_f64().unwrap_or(0.0),
        actual_quantity: m.actual_quantity.to_f64().unwrap_or(0.0),
        order_status: m.order_status,
        plan_start_date: m.plan_start_date,
        plan_end_date: m.plan_end_date,
    }))
}

async fn update_order(
    State(ctx): State<ApiContext>,
    Path(id): Path<i64>,
    Json(body): Json<OrderPayload>,
) -> Result<Json<OrderDto>, StatusCode> {
    let active = crate::db::entity::production_orders::ActiveModel {
        order_no: Set(body.order_no),
        plan_id: Set(body.plan_id),
        material_id: Set(body.material_id),
        plan_quantity: Set(Decimal::from_f64(body.plan_quantity).unwrap_or_default()),
        plan_start_date: Set(body.plan_start_date),
        plan_end_date: Set(body.plan_end_date),
        order_status: Set(body.order_status.unwrap_or(1)),
        workshop_id: Set(body.workshop_id),
        remark: Set(body.remark),
        ..Default::default()
    };
    let Some(m) = dao::production_order_dao::update(ctx.db.conn(), id, active)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    else {
        return Err(StatusCode::NOT_FOUND);
    };
    Ok(Json(OrderDto {
        id: m.id,
        order_no: m.order_no,
        plan_id: m.plan_id,
        material_id: m.material_id,
        plan_quantity: m.plan_quantity.to_f64().unwrap_or(0.0),
        actual_quantity: m.actual_quantity.to_f64().unwrap_or(0.0),
        order_status: m.order_status,
        plan_start_date: m.plan_start_date,
        plan_end_date: m.plan_end_date,
    }))
}

async fn delete_order(
    State(ctx): State<ApiContext>,
    Path(id): Path<i64>,
) -> Result<StatusCode, StatusCode> {
    let rows = dao::production_order_dao::delete(ctx.db.conn(), id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    if rows == 0 {
        return Err(StatusCode::NOT_FOUND);
    }
    Ok(StatusCode::NO_CONTENT)
}

// ---- Work Orders ----

async fn list_work_orders(
    State(ctx): State<ApiContext>,
    Query(q): Query<WorkOrderQuery>,
) -> Result<Json<PageResult<WorkOrderDto>>, StatusCode> {
    let filter = dao::work_order_dao::WorkOrderFilter {
        work_order_status: q.work_order_status,
        equipment_id: q.equipment_id,
        keyword: q.keyword.clone(),
    };
    let (items, total) =
        dao::work_order_dao::list(ctx.db.conn(), filter, q.page, q.page_size)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mapped = items
        .into_iter()
        .map(|m| WorkOrderDto {
            id: m.id,
            work_order_no: m.work_order_no,
            production_order_id: m.production_order_id,
            process_id: m.process_id,
            plan_quantity: m.plan_quantity.to_f64().unwrap_or(0.0),
            actual_quantity: m.actual_quantity.to_f64().unwrap_or(0.0),
            work_order_status: m.work_order_status,
        })
        .collect();

    Ok(Json(PageResult { items: mapped, total }))
}

async fn get_work_order(
    State(ctx): State<ApiContext>,
    Path(id): Path<i64>,
) -> Result<Json<WorkOrderDto>, StatusCode> {
    let Some(m) = dao::work_order_dao::get_by_id(ctx.db.conn(), id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    else {
        return Err(StatusCode::NOT_FOUND);
    };
    Ok(Json(WorkOrderDto {
        id: m.id,
        work_order_no: m.work_order_no,
        production_order_id: m.production_order_id,
        process_id: m.process_id,
        plan_quantity: m.plan_quantity.to_f64().unwrap_or(0.0),
        actual_quantity: m.actual_quantity.to_f64().unwrap_or(0.0),
        work_order_status: m.work_order_status,
    }))
}

async fn create_work_order(
    State(ctx): State<ApiContext>,
    Json(body): Json<WorkOrderPayload>,
) -> Result<Json<WorkOrderDto>, StatusCode> {
    let active = crate::db::entity::work_orders::ActiveModel {
        work_order_no: Set(body.work_order_no),
        production_order_id: Set(body.production_order_id),
        process_id: Set(body.process_id),
        plan_quantity: Set(Decimal::from_f64(body.plan_quantity).unwrap_or_default()),
        work_order_status: Set(body.work_order_status.unwrap_or(1)),
        remark: Set(body.remark),
        ..Default::default()
    };
    let m = dao::work_order_dao::create(ctx.db.conn(), active)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(WorkOrderDto {
        id: m.id,
        work_order_no: m.work_order_no,
        production_order_id: m.production_order_id,
        process_id: m.process_id,
        plan_quantity: m.plan_quantity.to_f64().unwrap_or(0.0),
        actual_quantity: m.actual_quantity.to_f64().unwrap_or(0.0),
        work_order_status: m.work_order_status,
    }))
}

async fn update_work_order(
    State(ctx): State<ApiContext>,
    Path(id): Path<i64>,
    Json(body): Json<WorkOrderPayload>,
) -> Result<Json<WorkOrderDto>, StatusCode> {
    let active = crate::db::entity::work_orders::ActiveModel {
        work_order_no: Set(body.work_order_no),
        production_order_id: Set(body.production_order_id),
        process_id: Set(body.process_id),
        plan_quantity: Set(Decimal::from_f64(body.plan_quantity).unwrap_or_default()),
        work_order_status: Set(body.work_order_status.unwrap_or(1)),
        remark: Set(body.remark),
        ..Default::default()
    };
    let Some(m) = dao::work_order_dao::update(ctx.db.conn(), id, active)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    else {
        return Err(StatusCode::NOT_FOUND);
    };
    Ok(Json(WorkOrderDto {
        id: m.id,
        work_order_no: m.work_order_no,
        production_order_id: m.production_order_id,
        process_id: m.process_id,
        plan_quantity: m.plan_quantity.to_f64().unwrap_or(0.0),
        actual_quantity: m.actual_quantity.to_f64().unwrap_or(0.0),
        work_order_status: m.work_order_status,
    }))
}

async fn delete_work_order(
    State(ctx): State<ApiContext>,
    Path(id): Path<i64>,
) -> Result<StatusCode, StatusCode> {
    let rows = dao::work_order_dao::delete(ctx.db.conn(), id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    if rows == 0 {
        return Err(StatusCode::NOT_FOUND);
    }
    Ok(StatusCode::NO_CONTENT)
}

// ---- Production Reports ----

async fn list_reports(
    State(ctx): State<ApiContext>,
    Query(q): Query<ReportQuery>,
) -> Result<Json<PageResult<ReportDto>>, StatusCode> {
    let filter = dao::production_report_dao::ReportFilter {
        report_type: q.report_type,
        operator_id: q.operator_id,
        start_date: q.start_date,
        end_date: q.end_date,
    };
    let (items, total) =
        dao::production_report_dao::list(ctx.db.conn(), filter, q.page, q.page_size)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mapped = items
        .into_iter()
        .map(|m| ReportDto {
            id: m.id,
            report_no: m.report_no,
            work_order_id: m.work_order_id,
            report_type: m.report_type,
            report_date: m.report_date,
            report_quantity: m.report_quantity.to_f64().unwrap_or(0.0),
            qualified_quantity: m.qualified_quantity.to_f64().unwrap_or(0.0),
            unqualified_quantity: m.unqualified_quantity.to_f64().unwrap_or(0.0),
        })
        .collect();

    Ok(Json(PageResult { items: mapped, total }))
}

async fn get_report(
    State(ctx): State<ApiContext>,
    Path(id): Path<i64>,
) -> Result<Json<ReportDto>, StatusCode> {
    let Some(m) = dao::production_report_dao::get_by_id(ctx.db.conn(), id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    else {
        return Err(StatusCode::NOT_FOUND);
    };
    Ok(Json(ReportDto {
        id: m.id,
        report_no: m.report_no,
        work_order_id: m.work_order_id,
        report_type: m.report_type,
        report_date: m.report_date,
        report_quantity: m.report_quantity.to_f64().unwrap_or(0.0),
        qualified_quantity: m.qualified_quantity.to_f64().unwrap_or(0.0),
        unqualified_quantity: m.unqualified_quantity.to_f64().unwrap_or(0.0),
    }))
}

async fn create_report(
    State(ctx): State<ApiContext>,
    Json(body): Json<ReportPayload>,
) -> Result<Json<ReportDto>, StatusCode> {
    let active = crate::db::entity::production_reports::ActiveModel {
        report_no: Set(body.report_no),
        work_order_id: Set(body.work_order_id),
        production_order_id: Set(body.production_order_id),
        process_id: Set(body.process_id),
        material_id: Set(body.material_id),
        report_type: Set(body.report_type),
        report_date: Set(body.report_date),
        report_quantity: Set(Decimal::from_f64(body.report_quantity).unwrap_or_default()),
        qualified_quantity: Set(Decimal::from_f64(body.qualified_quantity).unwrap_or_default()),
        unqualified_quantity: Set(Decimal::from_f64(body.unqualified_quantity).unwrap_or_default()),
        operator_id: Set(body.operator_id),
        remark: Set(body.remark),
        ..Default::default()
    };
    let m = dao::production_report_dao::create(ctx.db.conn(), active)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(ReportDto {
        id: m.id,
        report_no: m.report_no,
        work_order_id: m.work_order_id,
        report_type: m.report_type,
        report_date: m.report_date,
        report_quantity: m.report_quantity.to_f64().unwrap_or(0.0),
        qualified_quantity: m.qualified_quantity.to_f64().unwrap_or(0.0),
        unqualified_quantity: m.unqualified_quantity.to_f64().unwrap_or(0.0),
    }))
}

async fn update_report(
    State(ctx): State<ApiContext>,
    Path(id): Path<i64>,
    Json(body): Json<ReportPayload>,
) -> Result<Json<ReportDto>, StatusCode> {
    let active = crate::db::entity::production_reports::ActiveModel {
        report_no: Set(body.report_no),
        work_order_id: Set(body.work_order_id),
        production_order_id: Set(body.production_order_id),
        process_id: Set(body.process_id),
        material_id: Set(body.material_id),
        report_type: Set(body.report_type),
        report_date: Set(body.report_date),
        report_quantity: Set(Decimal::from_f64(body.report_quantity).unwrap_or_default()),
        qualified_quantity: Set(Decimal::from_f64(body.qualified_quantity).unwrap_or_default()),
        unqualified_quantity: Set(Decimal::from_f64(body.unqualified_quantity).unwrap_or_default()),
        operator_id: Set(body.operator_id),
        remark: Set(body.remark),
        ..Default::default()
    };
    let Some(m) = dao::production_report_dao::update(ctx.db.conn(), id, active)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    else {
        return Err(StatusCode::NOT_FOUND);
    };
    Ok(Json(ReportDto {
        id: m.id,
        report_no: m.report_no,
        work_order_id: m.work_order_id,
        report_type: m.report_type,
        report_date: m.report_date,
        report_quantity: m.report_quantity.to_f64().unwrap_or(0.0),
        qualified_quantity: m.qualified_quantity.to_f64().unwrap_or(0.0),
        unqualified_quantity: m.unqualified_quantity.to_f64().unwrap_or(0.0),
    }))
}

async fn delete_report(
    State(ctx): State<ApiContext>,
    Path(id): Path<i64>,
) -> Result<StatusCode, StatusCode> {
    let rows = dao::production_report_dao::delete(ctx.db.conn(), id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    if rows == 0 {
        return Err(StatusCode::NOT_FOUND);
    }
    Ok(StatusCode::NO_CONTENT)
}


