use crate::api::ApiContext;
use crate::db::dao;
use crate::model::production::*;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use num_traits::cast::{FromPrimitive, ToPrimitive};
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
        .route(
            "/production/material-requirements",
            get(list_material_requirements).post(create_material_requirement),
        )
        .route(
            "/production/material-requirements/:id",
            get(get_material_requirement)
                .put(update_material_requirement)
                .delete(delete_material_requirement),
        )
        .route(
            "/production/picking-orders",
            get(list_picking_orders).post(create_picking_order),
        )
        .route(
            "/production/picking-orders/:id",
            get(get_picking_order)
                .put(update_picking_order)
                .delete(delete_picking_order),
        )
        .route(
            "/production/return-orders",
            get(list_return_orders).post(create_return_order),
        )
        .route(
            "/production/return-orders/:id",
            get(get_return_order)
                .put(update_return_order)
                .delete(delete_return_order),
        )
        .route(
            "/production/receipts",
            get(list_production_receipts).post(create_production_receipt),
        )
        .route(
            "/production/receipts/:id",
            get(get_production_receipt)
                .put(update_production_receipt)
                .delete(delete_production_receipt),
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

// ---- Material Requirements ----

async fn list_material_requirements(
    State(ctx): State<ApiContext>,
    Query(q): Query<MaterialRequirementQuery>,
) -> Result<Json<PageResult<MaterialRequirementDto>>, StatusCode> {
    let filter = dao::material_requirement_dao::MaterialRequirementFilter {
        production_order_id: q.production_order_id,
        material_id: q.material_id,
    };
    let (items, total) =
        dao::material_requirement_dao::list(ctx.db.conn(), filter, q.page, q.page_size)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mapped = items
        .into_iter()
        .map(|m| MaterialRequirementDto {
            id: m.id,
            production_order_id: m.production_order_id,
            material_id: m.material_id,
            required_quantity: m.required_quantity.to_f64().unwrap_or(0.0),
            reserved_quantity: m.reserved_quantity.to_f64().unwrap_or(0.0),
            issued_quantity: m.issued_quantity.to_f64().unwrap_or(0.0),
            unit: m.unit,
            remark: m.remark,
        })
        .collect();

    Ok(Json(PageResult { items: mapped, total }))
}

async fn get_material_requirement(
    State(ctx): State<ApiContext>,
    Path(id): Path<i64>,
) -> Result<Json<MaterialRequirementDto>, StatusCode> {
    let Some(m) = dao::material_requirement_dao::get_by_id(ctx.db.conn(), id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    else {
        return Err(StatusCode::NOT_FOUND);
    };

    Ok(Json(MaterialRequirementDto {
        id: m.id,
        production_order_id: m.production_order_id,
        material_id: m.material_id,
        required_quantity: m.required_quantity.to_f64().unwrap_or(0.0),
        reserved_quantity: m.reserved_quantity.to_f64().unwrap_or(0.0),
        issued_quantity: m.issued_quantity.to_f64().unwrap_or(0.0),
        unit: m.unit,
        remark: m.remark,
    }))
}

async fn create_material_requirement(
    State(ctx): State<ApiContext>,
    Json(body): Json<MaterialRequirementPayload>,
) -> Result<Json<MaterialRequirementDto>, StatusCode> {
    use crate::db::entity::material_requirements;
    let active = material_requirements::ActiveModel {
        production_order_id: Set(body.production_order_id),
        material_id: Set(body.material_id),
        required_quantity: Set(Decimal::from_f64(body.required_quantity).unwrap_or_default()),
        reserved_quantity: Set(
            Decimal::from_f64(body.reserved_quantity.unwrap_or(0.0)).unwrap_or_default(),
        ),
        issued_quantity: Set(
            Decimal::from_f64(body.issued_quantity.unwrap_or(0.0)).unwrap_or_default(),
        ),
        unit: Set(body.unit),
        remark: Set(body.remark),
        ..Default::default()
    };
    let m = dao::material_requirement_dao::create(ctx.db.conn(), active)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(MaterialRequirementDto {
        id: m.id,
        production_order_id: m.production_order_id,
        material_id: m.material_id,
        required_quantity: m.required_quantity.to_f64().unwrap_or(0.0),
        reserved_quantity: m.reserved_quantity.to_f64().unwrap_or(0.0),
        issued_quantity: m.issued_quantity.to_f64().unwrap_or(0.0),
        unit: m.unit,
        remark: m.remark,
    }))
}

async fn update_material_requirement(
    State(ctx): State<ApiContext>,
    Path(id): Path<i64>,
    Json(body): Json<MaterialRequirementPayload>,
) -> Result<Json<MaterialRequirementDto>, StatusCode> {
    use crate::db::entity::material_requirements;
    let active = material_requirements::ActiveModel {
        production_order_id: Set(body.production_order_id),
        material_id: Set(body.material_id),
        required_quantity: Set(Decimal::from_f64(body.required_quantity).unwrap_or_default()),
        reserved_quantity: Set(
            Decimal::from_f64(body.reserved_quantity.unwrap_or(0.0)).unwrap_or_default(),
        ),
        issued_quantity: Set(
            Decimal::from_f64(body.issued_quantity.unwrap_or(0.0)).unwrap_or_default(),
        ),
        unit: Set(body.unit),
        remark: Set(body.remark),
        ..Default::default()
    };
    let Some(m) =
        dao::material_requirement_dao::update(ctx.db.conn(), id, active)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    else {
        return Err(StatusCode::NOT_FOUND);
    };

    Ok(Json(MaterialRequirementDto {
        id: m.id,
        production_order_id: m.production_order_id,
        material_id: m.material_id,
        required_quantity: m.required_quantity.to_f64().unwrap_or(0.0),
        reserved_quantity: m.reserved_quantity.to_f64().unwrap_or(0.0),
        issued_quantity: m.issued_quantity.to_f64().unwrap_or(0.0),
        unit: m.unit,
        remark: m.remark,
    }))
}

async fn delete_material_requirement(
    State(ctx): State<ApiContext>,
    Path(id): Path<i64>,
) -> Result<StatusCode, StatusCode> {
    let rows = dao::material_requirement_dao::delete(ctx.db.conn(), id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    if rows == 0 {
        return Err(StatusCode::NOT_FOUND);
    }
    Ok(StatusCode::NO_CONTENT)
}

// ---- Picking Orders ----

async fn list_picking_orders(
    State(ctx): State<ApiContext>,
    Query(q): Query<PickingOrderQuery>,
) -> Result<Json<PageResult<PickingOrderSummaryDto>>, StatusCode> {
    let filter = dao::picking_order_dao::PickingFilter {
        production_order_id: q.production_order_id,
        warehouse_id: q.warehouse_id,
        order_status: q.order_status,
    };
    let (items, total) =
        dao::picking_order_dao::list(ctx.db.conn(), filter, q.page, q.page_size)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mapped = items
        .into_iter()
        .map(|m| PickingOrderSummaryDto {
            id: m.id,
            picking_no: m.picking_no,
            production_order_id: m.production_order_id,
            warehouse_id: m.warehouse_id,
            work_order_id: m.work_order_id,
            picking_type: m.picking_type,
            plan_picking_date: m.plan_picking_date,
            actual_picking_date: m.actual_picking_date,
            total_quantity: m.total_quantity.to_f64().unwrap_or(0.0),
            order_status: m.order_status,
        })
        .collect();

    Ok(Json(PageResult { items: mapped, total }))
}

async fn get_picking_order(
    State(ctx): State<ApiContext>,
    Path(id): Path<i64>,
) -> Result<Json<PickingOrderWithDetailsDto>, StatusCode> {
    let Some((h, ds)) = dao::picking_order_dao::get_by_id(ctx.db.conn(), id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    else {
        return Err(StatusCode::NOT_FOUND);
    };

    let header = PickingOrderSummaryDto {
        id: h.id,
        picking_no: h.picking_no,
        production_order_id: h.production_order_id,
        warehouse_id: h.warehouse_id,
        work_order_id: h.work_order_id,
        picking_type: h.picking_type,
        plan_picking_date: h.plan_picking_date,
        actual_picking_date: h.actual_picking_date,
        total_quantity: h.total_quantity.to_f64().unwrap_or(0.0),
        order_status: h.order_status,
    };

    let details = ds
        .into_iter()
        .map(|d| PickingOrderDetailDto {
            id: d.id,
            material_id: d.material_id,
            warehouse_id: d.warehouse_id,
            location_id: d.location_id,
            batch_no: d.batch_no,
            plan_quantity: d.plan_quantity.to_f64().unwrap_or(0.0),
            actual_quantity: d.actual_quantity.to_f64().unwrap_or(0.0),
            unit: d.unit,
            line_status: d.line_status,
        })
        .collect();

    Ok(Json(PickingOrderWithDetailsDto { header, details }))
}

async fn create_picking_order(
    State(ctx): State<ApiContext>,
    Json(body): Json<PickingOrderPayload>,
) -> Result<Json<PickingOrderWithDetailsDto>, StatusCode> {
    use crate::db::entity::{picking_order_lines, picking_orders};

    let total_qty: f64 = body.details.iter().map(|d| d.plan_quantity).sum();
    let order_active = picking_orders::ActiveModel {
        picking_no: Set(body.picking_no),
        production_order_id: Set(body.production_order_id),
        warehouse_id: Set(body.warehouse_id),
        work_order_id: Set(body.work_order_id),
        picking_type: Set(body.picking_type),
        plan_picking_date: Set(body.plan_picking_date),
        total_quantity: Set(Decimal::from_f64(total_qty).unwrap_or_default()),
        order_status: Set(1),
        remark: Set(body.remark),
        ..Default::default()
    };

    let details_active = body
        .details
        .into_iter()
        .map(|d| picking_order_lines::ActiveModel {
            material_id: Set(d.material_id),
            warehouse_id: Set(d.warehouse_id),
            location_id: Set(d.location_id),
            batch_no: Set(d.batch_no),
            plan_quantity: Set(Decimal::from_f64(d.plan_quantity).unwrap_or_default()),
            actual_quantity: Set(Decimal::ZERO),
            unit: Set(d.unit),
            line_status: Set(1),
            ..Default::default()
        })
        .collect();

    let (h, ds) = dao::picking_order_dao::create(
        ctx.db.conn(),
        dao::picking_order_dao::PickingWithDetails {
            order: order_active,
            details: details_active,
        },
    )
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let header = PickingOrderSummaryDto {
        id: h.id,
        picking_no: h.picking_no,
        production_order_id: h.production_order_id,
        warehouse_id: h.warehouse_id,
        work_order_id: h.work_order_id,
        picking_type: h.picking_type,
        plan_picking_date: h.plan_picking_date,
        actual_picking_date: h.actual_picking_date,
        total_quantity: h.total_quantity.to_f64().unwrap_or(0.0),
        order_status: h.order_status,
    };

    let details = ds
        .into_iter()
        .map(|d| PickingOrderDetailDto {
            id: d.id,
            material_id: d.material_id,
            warehouse_id: d.warehouse_id,
            location_id: d.location_id,
            batch_no: d.batch_no,
            plan_quantity: d.plan_quantity.to_f64().unwrap_or(0.0),
            actual_quantity: d.actual_quantity.to_f64().unwrap_or(0.0),
            unit: d.unit,
            line_status: d.line_status,
        })
        .collect();

    Ok(Json(PickingOrderWithDetailsDto { header, details }))
}

async fn update_picking_order(
    State(ctx): State<ApiContext>,
    Path(id): Path<i64>,
    Json(body): Json<PickingOrderPayload>,
) -> Result<Json<PickingOrderWithDetailsDto>, StatusCode> {
    use crate::db::entity::{picking_order_lines, picking_orders};

    let total_qty: f64 = body.details.iter().map(|d| d.plan_quantity).sum();
    let order_active = picking_orders::ActiveModel {
        picking_no: Set(body.picking_no),
        production_order_id: Set(body.production_order_id),
        warehouse_id: Set(body.warehouse_id),
        work_order_id: Set(body.work_order_id),
        picking_type: Set(body.picking_type),
        plan_picking_date: Set(body.plan_picking_date),
        total_quantity: Set(Decimal::from_f64(total_qty).unwrap_or_default()),
        remark: Set(body.remark),
        ..Default::default()
    };

    let details_active = body
        .details
        .into_iter()
        .map(|d| picking_order_lines::ActiveModel {
            material_id: Set(d.material_id),
            warehouse_id: Set(d.warehouse_id),
            location_id: Set(d.location_id),
            batch_no: Set(d.batch_no),
            plan_quantity: Set(Decimal::from_f64(d.plan_quantity).unwrap_or_default()),
            actual_quantity: Set(Decimal::ZERO),
            unit: Set(d.unit),
            line_status: Set(1),
            ..Default::default()
        })
        .collect();

    let Some((h, ds)) =
        dao::picking_order_dao::update(
            ctx.db.conn(),
            id,
            dao::picking_order_dao::PickingWithDetails {
                order: order_active,
                details: details_active,
            },
        )
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    else {
        return Err(StatusCode::NOT_FOUND);
    };

    let header = PickingOrderSummaryDto {
        id: h.id,
        picking_no: h.picking_no,
        production_order_id: h.production_order_id,
        warehouse_id: h.warehouse_id,
        work_order_id: h.work_order_id,
        picking_type: h.picking_type,
        plan_picking_date: h.plan_picking_date,
        actual_picking_date: h.actual_picking_date,
        total_quantity: h.total_quantity.to_f64().unwrap_or(0.0),
        order_status: h.order_status,
    };

    let details = ds
        .into_iter()
        .map(|d| PickingOrderDetailDto {
            id: d.id,
            material_id: d.material_id,
            warehouse_id: d.warehouse_id,
            location_id: d.location_id,
            batch_no: d.batch_no,
            plan_quantity: d.plan_quantity.to_f64().unwrap_or(0.0),
            actual_quantity: d.actual_quantity.to_f64().unwrap_or(0.0),
            unit: d.unit,
            line_status: d.line_status,
        })
        .collect();

    Ok(Json(PickingOrderWithDetailsDto { header, details }))
}

async fn delete_picking_order(
    State(ctx): State<ApiContext>,
    Path(id): Path<i64>,
) -> Result<StatusCode, StatusCode> {
    let rows = dao::picking_order_dao::delete(ctx.db.conn(), id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    if rows == 0 {
        return Err(StatusCode::NOT_FOUND);
    }
    Ok(StatusCode::NO_CONTENT)
}

// ---- Return Orders ----

async fn list_return_orders(
    State(ctx): State<ApiContext>,
    Query(q): Query<ReturnOrderQuery>,
) -> Result<Json<PageResult<ReturnOrderSummaryDto>>, StatusCode> {
    let filter = dao::return_order_dao::ReturnFilter {
        production_order_id: q.production_order_id,
        warehouse_id: q.warehouse_id,
        order_status: q.order_status,
    };
    let (items, total) =
        dao::return_order_dao::list(ctx.db.conn(), filter, q.page, q.page_size)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mapped = items
        .into_iter()
        .map(|m| ReturnOrderSummaryDto {
            id: m.id,
            return_no: m.return_no,
            production_order_id: m.production_order_id,
            warehouse_id: m.warehouse_id,
            work_order_id: m.work_order_id,
            return_type: m.return_type,
            plan_return_date: m.plan_return_date,
            actual_return_date: m.actual_return_date,
            total_quantity: m.total_quantity.to_f64().unwrap_or(0.0),
            order_status: m.order_status,
        })
        .collect();

    Ok(Json(PageResult { items: mapped, total }))
}

async fn get_return_order(
    State(ctx): State<ApiContext>,
    Path(id): Path<i64>,
) -> Result<Json<ReturnOrderWithDetailsDto>, StatusCode> {
    let Some((h, ds)) = dao::return_order_dao::get_by_id(ctx.db.conn(), id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    else {
        return Err(StatusCode::NOT_FOUND);
    };

    let header = ReturnOrderSummaryDto {
        id: h.id,
        return_no: h.return_no,
        production_order_id: h.production_order_id,
        warehouse_id: h.warehouse_id,
        work_order_id: h.work_order_id,
        return_type: h.return_type,
        plan_return_date: h.plan_return_date,
        actual_return_date: h.actual_return_date,
        total_quantity: h.total_quantity.to_f64().unwrap_or(0.0),
        order_status: h.order_status,
    };

    let details = ds
        .into_iter()
        .map(|d| ReturnOrderDetailDto {
            id: d.id,
            material_id: d.material_id,
            warehouse_id: d.warehouse_id,
            location_id: d.location_id,
            batch_no: d.batch_no,
            plan_quantity: d.plan_quantity.to_f64().unwrap_or(0.0),
            actual_quantity: d.actual_quantity.to_f64().unwrap_or(0.0),
            unit: d.unit,
            line_status: d.line_status,
        })
        .collect();

    Ok(Json(ReturnOrderWithDetailsDto { header, details }))
}

async fn create_return_order(
    State(ctx): State<ApiContext>,
    Json(body): Json<ReturnOrderPayload>,
) -> Result<Json<ReturnOrderWithDetailsDto>, StatusCode> {
    use crate::db::entity::{return_order_lines, return_orders};

    let total_qty: f64 = body.details.iter().map(|d| d.plan_quantity).sum();
    let order_active = return_orders::ActiveModel {
        return_no: Set(body.return_no),
        production_order_id: Set(body.production_order_id),
        warehouse_id: Set(body.warehouse_id),
        work_order_id: Set(body.work_order_id),
        return_type: Set(body.return_type),
        plan_return_date: Set(body.plan_return_date),
        total_quantity: Set(Decimal::from_f64(total_qty).unwrap_or_default()),
        order_status: Set(1),
        remark: Set(body.remark),
        ..Default::default()
    };

    let details_active = body
        .details
        .into_iter()
        .map(|d| return_order_lines::ActiveModel {
            material_id: Set(d.material_id),
            warehouse_id: Set(d.warehouse_id),
            location_id: Set(d.location_id),
            batch_no: Set(d.batch_no),
            plan_quantity: Set(Decimal::from_f64(d.plan_quantity).unwrap_or_default()),
            actual_quantity: Set(Decimal::ZERO),
            unit: Set(d.unit),
            line_status: Set(1),
            ..Default::default()
        })
        .collect();

    let (h, ds) = dao::return_order_dao::create(
        ctx.db.conn(),
        dao::return_order_dao::ReturnWithDetails {
            order: order_active,
            details: details_active,
        },
    )
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let header = ReturnOrderSummaryDto {
        id: h.id,
        return_no: h.return_no,
        production_order_id: h.production_order_id,
        warehouse_id: h.warehouse_id,
        work_order_id: h.work_order_id,
        return_type: h.return_type,
        plan_return_date: h.plan_return_date,
        actual_return_date: h.actual_return_date,
        total_quantity: h.total_quantity.to_f64().unwrap_or(0.0),
        order_status: h.order_status,
    };

    let details = ds
        .into_iter()
        .map(|d| ReturnOrderDetailDto {
            id: d.id,
            material_id: d.material_id,
            warehouse_id: d.warehouse_id,
            location_id: d.location_id,
            batch_no: d.batch_no,
            plan_quantity: d.plan_quantity.to_f64().unwrap_or(0.0),
            actual_quantity: d.actual_quantity.to_f64().unwrap_or(0.0),
            unit: d.unit,
            line_status: d.line_status,
        })
        .collect();

    Ok(Json(ReturnOrderWithDetailsDto { header, details }))
}

async fn update_return_order(
    State(ctx): State<ApiContext>,
    Path(id): Path<i64>,
    Json(body): Json<ReturnOrderPayload>,
) -> Result<Json<ReturnOrderWithDetailsDto>, StatusCode> {
    use crate::db::entity::{return_order_lines, return_orders};

    let total_qty: f64 = body.details.iter().map(|d| d.plan_quantity).sum();
    let order_active = return_orders::ActiveModel {
        return_no: Set(body.return_no),
        production_order_id: Set(body.production_order_id),
        warehouse_id: Set(body.warehouse_id),
        work_order_id: Set(body.work_order_id),
        return_type: Set(body.return_type),
        plan_return_date: Set(body.plan_return_date),
        total_quantity: Set(Decimal::from_f64(total_qty).unwrap_or_default()),
        remark: Set(body.remark),
        ..Default::default()
    };

    let details_active = body
        .details
        .into_iter()
        .map(|d| return_order_lines::ActiveModel {
            material_id: Set(d.material_id),
            warehouse_id: Set(d.warehouse_id),
            location_id: Set(d.location_id),
            batch_no: Set(d.batch_no),
            plan_quantity: Set(Decimal::from_f64(d.plan_quantity).unwrap_or_default()),
            actual_quantity: Set(Decimal::ZERO),
            unit: Set(d.unit),
            line_status: Set(1),
            ..Default::default()
        })
        .collect();

    let Some((h, ds)) =
        dao::return_order_dao::update(
            ctx.db.conn(),
            id,
            dao::return_order_dao::ReturnWithDetails {
                order: order_active,
                details: details_active,
            },
        )
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    else {
        return Err(StatusCode::NOT_FOUND);
    };

    let header = ReturnOrderSummaryDto {
        id: h.id,
        return_no: h.return_no,
        production_order_id: h.production_order_id,
        warehouse_id: h.warehouse_id,
        work_order_id: h.work_order_id,
        return_type: h.return_type,
        plan_return_date: h.plan_return_date,
        actual_return_date: h.actual_return_date,
        total_quantity: h.total_quantity.to_f64().unwrap_or(0.0),
        order_status: h.order_status,
    };

    let details = ds
        .into_iter()
        .map(|d| ReturnOrderDetailDto {
            id: d.id,
            material_id: d.material_id,
            warehouse_id: d.warehouse_id,
            location_id: d.location_id,
            batch_no: d.batch_no,
            plan_quantity: d.plan_quantity.to_f64().unwrap_or(0.0),
            actual_quantity: d.actual_quantity.to_f64().unwrap_or(0.0),
            unit: d.unit,
            line_status: d.line_status,
        })
        .collect();

    Ok(Json(ReturnOrderWithDetailsDto { header, details }))
}

async fn delete_return_order(
    State(ctx): State<ApiContext>,
    Path(id): Path<i64>,
) -> Result<StatusCode, StatusCode> {
    let rows = dao::return_order_dao::delete(ctx.db.conn(), id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    if rows == 0 {
        return Err(StatusCode::NOT_FOUND);
    }
    Ok(StatusCode::NO_CONTENT)
}

// ---- Production Receipts ----

async fn list_production_receipts(
    State(ctx): State<ApiContext>,
    Query(q): Query<ProductionReceiptQuery>,
) -> Result<Json<PageResult<ProductionReceiptDto>>, StatusCode> {
    let filter = dao::production_receipt_dao::ProductionReceiptFilter {
        production_order_id: q.production_order_id,
        warehouse_id: q.warehouse_id,
    };
    let (items, total) =
        dao::production_receipt_dao::list(ctx.db.conn(), filter, q.page, q.page_size)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mapped = items
        .into_iter()
        .map(|m| ProductionReceiptDto {
            id: m.id,
            receipt_no: m.receipt_no,
            production_order_id: m.production_order_id,
            work_order_id: m.work_order_id,
            material_id: m.material_id,
            warehouse_id: m.warehouse_id,
            location_id: m.location_id,
            receipt_type: m.receipt_type,
            receipt_date: m.receipt_date,
            quantity: m.quantity.to_f64().unwrap_or(0.0),
            qualified_quantity: m.qualified_quantity.to_f64().unwrap_or(0.0),
            unqualified_quantity: m.unqualified_quantity.to_f64().unwrap_or(0.0),
            unit: m.unit,
            remark: m.remark,
        })
        .collect();

    Ok(Json(PageResult { items: mapped, total }))
}

async fn get_production_receipt(
    State(ctx): State<ApiContext>,
    Path(id): Path<i64>,
) -> Result<Json<ProductionReceiptDto>, StatusCode> {
    let Some(m) = dao::production_receipt_dao::get_by_id(ctx.db.conn(), id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    else {
        return Err(StatusCode::NOT_FOUND);
    };

    Ok(Json(ProductionReceiptDto {
        id: m.id,
        receipt_no: m.receipt_no,
        production_order_id: m.production_order_id,
        work_order_id: m.work_order_id,
        material_id: m.material_id,
        warehouse_id: m.warehouse_id,
        location_id: m.location_id,
        receipt_type: m.receipt_type,
        receipt_date: m.receipt_date,
        quantity: m.quantity.to_f64().unwrap_or(0.0),
        qualified_quantity: m.qualified_quantity.to_f64().unwrap_or(0.0),
        unqualified_quantity: m.unqualified_quantity.to_f64().unwrap_or(0.0),
        unit: m.unit,
        remark: m.remark,
    }))
}

async fn create_production_receipt(
    State(ctx): State<ApiContext>,
    Json(body): Json<ProductionReceiptPayload>,
) -> Result<Json<ProductionReceiptDto>, StatusCode> {
    use crate::db::entity::production_receipts;
    let active = production_receipts::ActiveModel {
        receipt_no: Set(body.receipt_no),
        production_order_id: Set(body.production_order_id),
        work_order_id: Set(body.work_order_id),
        material_id: Set(body.material_id),
        warehouse_id: Set(body.warehouse_id),
        location_id: Set(body.location_id),
        receipt_type: Set(body.receipt_type),
        receipt_date: Set(body.receipt_date),
        quantity: Set(Decimal::from_f64(body.quantity).unwrap_or_default()),
        qualified_quantity: Set(Decimal::from_f64(body.qualified_quantity).unwrap_or_default()),
        unqualified_quantity: Set(Decimal::from_f64(body.unqualified_quantity).unwrap_or_default()),
        unit: Set(body.unit),
        remark: Set(body.remark),
        ..Default::default()
    };
    let m = dao::production_receipt_dao::create(ctx.db.conn(), active)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(ProductionReceiptDto {
        id: m.id,
        receipt_no: m.receipt_no,
        production_order_id: m.production_order_id,
        work_order_id: m.work_order_id,
        material_id: m.material_id,
        warehouse_id: m.warehouse_id,
        location_id: m.location_id,
        receipt_type: m.receipt_type,
        receipt_date: m.receipt_date,
        quantity: m.quantity.to_f64().unwrap_or(0.0),
        qualified_quantity: m.qualified_quantity.to_f64().unwrap_or(0.0),
        unqualified_quantity: m.unqualified_quantity.to_f64().unwrap_or(0.0),
        unit: m.unit,
        remark: m.remark,
    }))
}

async fn update_production_receipt(
    State(ctx): State<ApiContext>,
    Path(id): Path<i64>,
    Json(body): Json<ProductionReceiptPayload>,
) -> Result<Json<ProductionReceiptDto>, StatusCode> {
    use crate::db::entity::production_receipts;
    let active = production_receipts::ActiveModel {
        receipt_no: Set(body.receipt_no),
        production_order_id: Set(body.production_order_id),
        work_order_id: Set(body.work_order_id),
        material_id: Set(body.material_id),
        warehouse_id: Set(body.warehouse_id),
        location_id: Set(body.location_id),
        receipt_type: Set(body.receipt_type),
        receipt_date: Set(body.receipt_date),
        quantity: Set(Decimal::from_f64(body.quantity).unwrap_or_default()),
        qualified_quantity: Set(Decimal::from_f64(body.qualified_quantity).unwrap_or_default()),
        unqualified_quantity: Set(Decimal::from_f64(body.unqualified_quantity).unwrap_or_default()),
        unit: Set(body.unit),
        remark: Set(body.remark),
        ..Default::default()
    };
    let Some(m) =
        dao::production_receipt_dao::update(ctx.db.conn(), id, active)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    else {
        return Err(StatusCode::NOT_FOUND);
    };

    Ok(Json(ProductionReceiptDto {
        id: m.id,
        receipt_no: m.receipt_no,
        production_order_id: m.production_order_id,
        work_order_id: m.work_order_id,
        material_id: m.material_id,
        warehouse_id: m.warehouse_id,
        location_id: m.location_id,
        receipt_type: m.receipt_type,
        receipt_date: m.receipt_date,
        quantity: m.quantity.to_f64().unwrap_or(0.0),
        qualified_quantity: m.qualified_quantity.to_f64().unwrap_or(0.0),
        unqualified_quantity: m.unqualified_quantity.to_f64().unwrap_or(0.0),
        unit: m.unit,
        remark: m.remark,
    }))
}

async fn delete_production_receipt(
    State(ctx): State<ApiContext>,
    Path(id): Path<i64>,
) -> Result<StatusCode, StatusCode> {
    let rows = dao::production_receipt_dao::delete(ctx.db.conn(), id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    if rows == 0 {
        return Err(StatusCode::NOT_FOUND);
    }
    Ok(StatusCode::NO_CONTENT)
}


