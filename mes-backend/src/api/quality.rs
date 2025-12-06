use crate::api::ApiContext;
use crate::db::dao;
use crate::model::quality::{self, *};
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
        // Inspection Tasks
        .route("/quality/inspection-tasks", get(list_inspection_tasks).post(create_inspection_task))
        .route(
            "/quality/inspection-tasks/:id",
            get(get_inspection_task).put(update_inspection_task).delete(delete_inspection_task),
        )
        // Inspection Reports
        .route("/quality/inspection-reports", get(list_inspection_reports).post(create_inspection_report))
        .route(
            "/quality/inspection-reports/:id",
            get(get_inspection_report).put(update_inspection_report).delete(delete_inspection_report),
        )
        // NCR
        .route("/quality/ncr", get(list_ncr).post(create_ncr))
        .route("/quality/ncr/:id", get(get_ncr).put(update_ncr).delete(delete_ncr))
        // Customer Complaints
        .route("/quality/complaints", get(list_complaints).post(create_complaint))
        .route("/quality/complaints/:id", get(get_complaint).put(update_complaint).delete(delete_complaint))
        // Rework Orders
        .route("/quality/rework-orders", get(list_rework_orders).post(create_rework_order))
        .route("/quality/rework-orders/:id", get(get_rework_order).put(update_rework_order).delete(delete_rework_order))
        // Measuring Equipment
        .route("/quality/measuring-equipment", get(list_measuring_equipment).post(create_measuring_equipment))
        .route(
            "/quality/measuring-equipment/:id",
            get(get_measuring_equipment).put(update_measuring_equipment).delete(delete_measuring_equipment),
        )
        // Supplier Quality Evaluations
        .route("/quality/supplier-evaluations", get(list_supplier_evaluations).post(create_supplier_evaluation))
        .route(
            "/quality/supplier-evaluations/:id",
            get(get_supplier_evaluation).put(update_supplier_evaluation).delete(delete_supplier_evaluation),
        )
        // Quality Traceability Records
        .route("/quality/traceability", get(list_traceability_records).post(create_traceability_record))
        .route(
            "/quality/traceability/:id",
            get(get_traceability_record).put(update_traceability_record).delete(delete_traceability_record),
        )
        // Quality Costs
        .route("/quality/costs", get(list_quality_costs).post(create_quality_cost))
        .route(
            "/quality/costs/:id",
            get(get_quality_cost).put(update_quality_cost).delete(delete_quality_cost),
        )
        // Quality KPI
        .route("/quality/kpi", get(list_quality_kpi).post(create_quality_kpi))
        .route("/quality/kpi/:id", get(get_quality_kpi).put(update_quality_kpi))
}

// ---- Inspection Tasks ----

async fn list_inspection_tasks(
    State(ctx): State<ApiContext>,
    Query(q): Query<InspectionTaskQuery>,
) -> Result<Json<quality::PageResult<InspectionTaskDto>>, StatusCode> {
    let filter = dao::quality_inspection_task_dao::QualityInspectionTaskFilter {
        inspection_type: q.inspection_type,
        source_type: q.source_type,
        source_order_no: q.source_order_no.clone(),
        material_id: q.material_id,
        batch_no: q.batch_no.clone(),
        task_status: q.task_status,
        inspector_id: q.inspector_id,
    };
    let (items, total) = dao::quality_inspection_task_dao::list(ctx.db.conn(), filter, q.page, q.page_size)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mapped = items
        .into_iter()
        .map(|m| InspectionTaskDto {
            id: m.id,
            task_no: m.task_no,
            inspection_type: m.inspection_type,
            source_type: m.source_type,
            source_order_no: m.source_order_no,
            material_id: m.material_id,
            batch_no: m.batch_no,
            task_status: m.task_status,
            inspection_result: m.inspection_result,
            inspection_quantity: m.inspection_quantity.to_f64().unwrap_or(0.0),
            qualified_quantity: m.qualified_quantity.to_f64().unwrap_or(0.0),
            unqualified_quantity: m.unqualified_quantity.to_f64().unwrap_or(0.0),
            unit: m.unit,
            plan_start_time: m.plan_start_time,
            plan_end_time: m.plan_end_time,
            actual_start_time: m.actual_start_time,
            actual_end_time: m.actual_end_time,
            inspector_id: m.inspector_id,
            priority: m.priority,
        })
        .collect();

    Ok(Json(quality::PageResult { items: mapped, total }))
}

async fn get_inspection_task(
    State(ctx): State<ApiContext>,
    Path(id): Path<i64>,
) -> Result<Json<InspectionTaskDto>, StatusCode> {
    let Some(m) = dao::quality_inspection_task_dao::get_by_id(ctx.db.conn(), id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    else {
        return Err(StatusCode::NOT_FOUND);
    };
    Ok(Json(InspectionTaskDto {
        id: m.id,
        task_no: m.task_no,
        inspection_type: m.inspection_type,
        source_type: m.source_type,
        source_order_no: m.source_order_no,
        material_id: m.material_id,
        batch_no: m.batch_no,
        task_status: m.task_status,
        inspection_result: m.inspection_result,
        inspection_quantity: m.inspection_quantity.to_f64().unwrap_or(0.0),
        qualified_quantity: m.qualified_quantity.to_f64().unwrap_or(0.0),
        unqualified_quantity: m.unqualified_quantity.to_f64().unwrap_or(0.0),
        unit: m.unit,
        plan_start_time: m.plan_start_time,
        plan_end_time: m.plan_end_time,
        actual_start_time: m.actual_start_time,
        actual_end_time: m.actual_end_time,
        inspector_id: m.inspector_id,
        priority: m.priority,
    }))
}

async fn create_inspection_task(
    State(ctx): State<ApiContext>,
    Json(body): Json<InspectionTaskPayload>,
) -> Result<Json<InspectionTaskDto>, StatusCode> {
    use crate::db::entity::quality_inspection_tasks;
    
    let active = quality_inspection_tasks::ActiveModel {
        task_no: Set(body.task_no),
        inspection_type: Set(body.inspection_type),
        source_type: Set(body.source_type),
        source_order_no: Set(body.source_order_no),
        material_id: Set(body.material_id),
        batch_no: Set(body.batch_no),
        supplier_id: Set(body.supplier_id),
        production_order_id: Set(body.production_order_id),
        work_order_id: Set(body.work_order_id),
        inspection_quantity: Set(Decimal::from_f64(body.inspection_quantity).unwrap_or_default()),
        sample_quantity: Set(Decimal::from(0)),
        qualified_quantity: Set(Decimal::from(0)),
        unqualified_quantity: Set(Decimal::from(0)),
        unit: Set(body.unit),
        plan_start_time: Set(body.plan_start_time),
        plan_end_time: Set(body.plan_end_time),
        inspector_id: Set(body.inspector_id),
        task_status: Set(1),
        priority: Set(body.priority.unwrap_or(3)),
        remark: Set(body.remark),
        created_time: Set(chrono::Utc::now().into()),
        updated_time: Set(chrono::Utc::now().into()),
        is_deleted: Set(0),
        ..Default::default()
    };

    let m = dao::quality_inspection_task_dao::create(ctx.db.conn(), active)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(InspectionTaskDto {
        id: m.id,
        task_no: m.task_no,
        inspection_type: m.inspection_type,
        source_type: m.source_type,
        source_order_no: m.source_order_no,
        material_id: m.material_id,
        batch_no: m.batch_no,
        task_status: m.task_status,
        inspection_result: m.inspection_result,
        inspection_quantity: m.inspection_quantity.to_f64().unwrap_or(0.0),
        qualified_quantity: m.qualified_quantity.to_f64().unwrap_or(0.0),
        unqualified_quantity: m.unqualified_quantity.to_f64().unwrap_or(0.0),
        unit: m.unit,
        plan_start_time: m.plan_start_time,
        plan_end_time: m.plan_end_time,
        actual_start_time: m.actual_start_time,
        actual_end_time: m.actual_end_time,
        inspector_id: m.inspector_id,
        priority: m.priority,
    }))
}

async fn update_inspection_task(
    State(ctx): State<ApiContext>,
    Path(id): Path<i64>,
    Json(body): Json<InspectionTaskPayload>,
) -> Result<Json<InspectionTaskDto>, StatusCode> {
    use crate::db::entity::quality_inspection_tasks;
    
    let active = quality_inspection_tasks::ActiveModel {
        task_no: Set(body.task_no),
        inspection_type: Set(body.inspection_type),
        source_type: Set(body.source_type),
        source_order_no: Set(body.source_order_no),
        material_id: Set(body.material_id),
        batch_no: Set(body.batch_no),
        inspection_quantity: Set(Decimal::from_f64(body.inspection_quantity).unwrap_or_default()),
        unit: Set(body.unit),
        plan_start_time: Set(body.plan_start_time),
        plan_end_time: Set(body.plan_end_time),
        inspector_id: Set(body.inspector_id),
        priority: Set(body.priority.unwrap_or(3)),
        remark: Set(body.remark),
        updated_time: Set(chrono::Utc::now().into()),
        ..Default::default()
    };

    let m = dao::quality_inspection_task_dao::update(ctx.db.conn(), id, active)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(InspectionTaskDto {
        id: m.id,
        task_no: m.task_no,
        inspection_type: m.inspection_type,
        source_type: m.source_type,
        source_order_no: m.source_order_no,
        material_id: m.material_id,
        batch_no: m.batch_no,
        task_status: m.task_status,
        inspection_result: m.inspection_result,
        inspection_quantity: m.inspection_quantity.to_f64().unwrap_or(0.0),
        qualified_quantity: m.qualified_quantity.to_f64().unwrap_or(0.0),
        unqualified_quantity: m.unqualified_quantity.to_f64().unwrap_or(0.0),
        unit: m.unit,
        plan_start_time: m.plan_start_time,
        plan_end_time: m.plan_end_time,
        actual_start_time: m.actual_start_time,
        actual_end_time: m.actual_end_time,
        inspector_id: m.inspector_id,
        priority: m.priority,
    }))
}

async fn delete_inspection_task(
    State(ctx): State<ApiContext>,
    Path(id): Path<i64>,
) -> Result<StatusCode, StatusCode> {
    dao::quality_inspection_task_dao::delete(ctx.db.conn(), id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(StatusCode::NO_CONTENT)
}

// ---- Inspection Reports ----

async fn list_inspection_reports(
    State(ctx): State<ApiContext>,
    Query(q): Query<InspectionReportQuery>,
) -> Result<Json<quality::PageResult<InspectionReportDto>>, StatusCode> {
    let filter = dao::quality_inspection_report_dao::QualityInspectionReportFilter {
        task_id: q.task_id,
        inspection_type: q.inspection_type,
        material_id: q.material_id,
        batch_no: q.batch_no.clone(),
        report_status: q.report_status,
        inspection_result: q.inspection_result,
    };
    let (items, total) = dao::quality_inspection_report_dao::list(ctx.db.conn(), filter, q.page, q.page_size)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mapped = items
        .into_iter()
        .map(|m| InspectionReportDto {
            id: m.id,
            report_no: m.report_no,
            task_id: m.task_id,
            inspection_type: m.inspection_type,
            material_id: m.material_id,
            batch_no: m.batch_no,
            inspection_date: m.inspection_date,
            inspection_time: m.inspection_time,
            inspector_id: m.inspector_id,
            inspection_quantity: m.inspection_quantity.to_f64().unwrap_or(0.0),
            sample_quantity: m.sample_quantity.to_f64().unwrap_or(0.0),
            qualified_quantity: m.qualified_quantity.to_f64().unwrap_or(0.0),
            unqualified_quantity: m.unqualified_quantity.to_f64().unwrap_or(0.0),
            qualified_rate: m.qualified_rate.to_f64().unwrap_or(0.0),
            inspection_result: m.inspection_result,
            disposition: m.disposition,
            report_status: m.report_status,
        })
        .collect();

    Ok(Json(quality::PageResult { items: mapped, total }))
}

async fn get_inspection_report(
    State(ctx): State<ApiContext>,
    Path(id): Path<i64>,
) -> Result<Json<InspectionReportDto>, StatusCode> {
    let Some(m) = dao::quality_inspection_report_dao::get_by_id(ctx.db.conn(), id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    else {
        return Err(StatusCode::NOT_FOUND);
    };
    Ok(Json(InspectionReportDto {
        id: m.id,
        report_no: m.report_no,
        task_id: m.task_id,
        inspection_type: m.inspection_type,
        material_id: m.material_id,
        batch_no: m.batch_no,
        inspection_date: m.inspection_date,
        inspection_time: m.inspection_time,
        inspector_id: m.inspector_id,
        inspection_quantity: m.inspection_quantity.to_f64().unwrap_or(0.0),
        sample_quantity: m.sample_quantity.to_f64().unwrap_or(0.0),
        qualified_quantity: m.qualified_quantity.to_f64().unwrap_or(0.0),
        unqualified_quantity: m.unqualified_quantity.to_f64().unwrap_or(0.0),
        qualified_rate: m.qualified_rate.to_f64().unwrap_or(0.0),
        inspection_result: m.inspection_result,
        disposition: m.disposition,
        report_status: m.report_status,
    }))
}

async fn create_inspection_report(
    State(ctx): State<ApiContext>,
    Json(body): Json<InspectionReportPayload>,
) -> Result<Json<InspectionReportDto>, StatusCode> {
    use crate::db::entity::quality_inspection_reports;
    
    let qualified_rate = if body.inspection_quantity > 0.0 {
        (body.qualified_quantity / body.inspection_quantity) * 100.0
    } else {
        0.0
    };

    let active = quality_inspection_reports::ActiveModel {
        report_no: Set(body.report_no),
        task_id: Set(body.task_id),
        inspection_type: Set(body.inspection_type),
        material_id: Set(body.material_id),
        batch_no: Set(body.batch_no),
        inspection_date: Set(body.inspection_date),
        inspection_time: Set(body.inspection_time),
        inspector_id: Set(body.inspector_id),
        inspection_quantity: Set(Decimal::from_f64(body.inspection_quantity).unwrap_or_default()),
        sample_quantity: Set(Decimal::from_f64(body.sample_quantity).unwrap_or_default()),
        qualified_quantity: Set(Decimal::from_f64(body.qualified_quantity).unwrap_or_default()),
        unqualified_quantity: Set(Decimal::from_f64(body.unqualified_quantity).unwrap_or_default()),
        unit: Set(body.unit),
        qualified_rate: Set(Decimal::from_f64(qualified_rate).unwrap_or_default()),
        inspection_result: Set(body.inspection_result),
        disposition: Set(body.disposition),
        major_defects: Set(body.major_defects.unwrap_or(0)),
        minor_defects: Set(body.minor_defects.unwrap_or(0)),
        critical_defects: Set(body.critical_defects.unwrap_or(0)),
        conclusion: Set(body.conclusion),
        improvement_suggestions: Set(body.improvement_suggestions),
        report_status: Set(1),
        remark: Set(body.remark),
        created_time: Set(chrono::Utc::now().into()),
        updated_time: Set(chrono::Utc::now().into()),
        is_deleted: Set(0),
        ..Default::default()
    };

    let m = dao::quality_inspection_report_dao::create(ctx.db.conn(), active)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(InspectionReportDto {
        id: m.id,
        report_no: m.report_no,
        task_id: m.task_id,
        inspection_type: m.inspection_type,
        material_id: m.material_id,
        batch_no: m.batch_no,
        inspection_date: m.inspection_date,
        inspection_time: m.inspection_time,
        inspector_id: m.inspector_id,
        inspection_quantity: m.inspection_quantity.to_f64().unwrap_or(0.0),
        sample_quantity: m.sample_quantity.to_f64().unwrap_or(0.0),
        qualified_quantity: m.qualified_quantity.to_f64().unwrap_or(0.0),
        unqualified_quantity: m.unqualified_quantity.to_f64().unwrap_or(0.0),
        qualified_rate: m.qualified_rate.to_f64().unwrap_or(0.0),
        inspection_result: m.inspection_result,
        disposition: m.disposition,
        report_status: m.report_status,
    }))
}

async fn update_inspection_report(
    State(ctx): State<ApiContext>,
    Path(id): Path<i64>,
    Json(body): Json<InspectionReportPayload>,
) -> Result<Json<InspectionReportDto>, StatusCode> {
    use crate::db::entity::quality_inspection_reports;
    
    let qualified_rate = if body.inspection_quantity > 0.0 {
        (body.qualified_quantity / body.inspection_quantity) * 100.0
    } else {
        0.0
    };

    let active = quality_inspection_reports::ActiveModel {
        report_no: Set(body.report_no),
        task_id: Set(body.task_id),
        inspection_type: Set(body.inspection_type),
        material_id: Set(body.material_id),
        batch_no: Set(body.batch_no),
        inspection_date: Set(body.inspection_date),
        inspection_time: Set(body.inspection_time),
        inspector_id: Set(body.inspector_id),
        inspection_quantity: Set(Decimal::from_f64(body.inspection_quantity).unwrap_or_default()),
        sample_quantity: Set(Decimal::from_f64(body.sample_quantity).unwrap_or_default()),
        qualified_quantity: Set(Decimal::from_f64(body.qualified_quantity).unwrap_or_default()),
        unqualified_quantity: Set(Decimal::from_f64(body.unqualified_quantity).unwrap_or_default()),
        unit: Set(body.unit),
        qualified_rate: Set(Decimal::from_f64(qualified_rate).unwrap_or_default()),
        inspection_result: Set(body.inspection_result),
        disposition: Set(body.disposition),
        major_defects: Set(body.major_defects.unwrap_or(0)),
        minor_defects: Set(body.minor_defects.unwrap_or(0)),
        critical_defects: Set(body.critical_defects.unwrap_or(0)),
        conclusion: Set(body.conclusion),
        improvement_suggestions: Set(body.improvement_suggestions),
        remark: Set(body.remark),
        updated_time: Set(chrono::Utc::now().into()),
        ..Default::default()
    };

    let Some(m) = dao::quality_inspection_report_dao::update(ctx.db.conn(), id, active)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    else {
        return Err(StatusCode::NOT_FOUND);
    };

    Ok(Json(InspectionReportDto {
        id: m.id,
        report_no: m.report_no,
        task_id: m.task_id,
        inspection_type: m.inspection_type,
        material_id: m.material_id,
        batch_no: m.batch_no,
        inspection_date: m.inspection_date,
        inspection_time: m.inspection_time,
        inspector_id: m.inspector_id,
        inspection_quantity: m.inspection_quantity.to_f64().unwrap_or(0.0),
        sample_quantity: m.sample_quantity.to_f64().unwrap_or(0.0),
        qualified_quantity: m.qualified_quantity.to_f64().unwrap_or(0.0),
        unqualified_quantity: m.unqualified_quantity.to_f64().unwrap_or(0.0),
        qualified_rate: m.qualified_rate.to_f64().unwrap_or(0.0),
        inspection_result: m.inspection_result,
        disposition: m.disposition,
        report_status: m.report_status,
    }))
}

async fn delete_inspection_report(
    State(ctx): State<ApiContext>,
    Path(id): Path<i64>,
) -> Result<StatusCode, StatusCode> {
    dao::quality_inspection_report_dao::delete(ctx.db.conn(), id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(StatusCode::NO_CONTENT)
}

// ---- NCR ----

async fn list_ncr(
    State(ctx): State<ApiContext>,
    Query(q): Query<NcrQuery>,
) -> Result<Json<quality::PageResult<NcrDto>>, StatusCode> {
    let filter = dao::nonconforming_product_dao::NonconformingProductFilter {
        ncr_status: q.ncr_status,
        material_id: q.material_id,
        batch_no: q.batch_no.clone(),
        defect_level: q.defect_level,
        source_type: q.source_type,
    };
    let (items, total) = dao::nonconforming_product_dao::list(ctx.db.conn(), filter, q.page, q.page_size)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mapped = items
        .into_iter()
        .map(|m| NcrDto {
            id: m.id,
            ncr_no: m.ncr_no,
            material_id: m.material_id,
            batch_no: m.batch_no,
            defect_quantity: m.defect_quantity.to_f64().unwrap_or(0.0),
            unit: m.unit,
            defect_code: m.defect_code,
            defect_name: m.defect_name,
            defect_level: m.defect_level,
            found_date: m.found_date,
            ncr_status: m.ncr_status,
            disposition: m.disposition,
        })
        .collect();

    Ok(Json(quality::PageResult { items: mapped, total }))
}

async fn get_ncr(
    State(ctx): State<ApiContext>,
    Path(id): Path<i64>,
) -> Result<Json<NcrDto>, StatusCode> {
    let Some(m) = dao::nonconforming_product_dao::get_by_id(ctx.db.conn(), id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    else {
        return Err(StatusCode::NOT_FOUND);
    };
    Ok(Json(NcrDto {
        id: m.id,
        ncr_no: m.ncr_no,
        material_id: m.material_id,
        batch_no: m.batch_no,
        defect_quantity: m.defect_quantity.to_f64().unwrap_or(0.0),
        unit: m.unit,
        defect_code: m.defect_code,
        defect_name: m.defect_name,
        defect_level: m.defect_level,
        found_date: m.found_date,
        ncr_status: m.ncr_status,
        disposition: m.disposition,
    }))
}

async fn create_ncr(
    State(ctx): State<ApiContext>,
    Json(body): Json<NcrPayload>,
) -> Result<Json<NcrDto>, StatusCode> {
    use crate::db::entity::nonconforming_products;
    
    let active = nonconforming_products::ActiveModel {
        ncr_no: Set(body.ncr_no),
        report_id: Set(body.report_id),
        source_type: Set(body.source_type),
        material_id: Set(body.material_id),
        batch_no: Set(body.batch_no),
        defect_quantity: Set(Decimal::from_f64(body.defect_quantity).unwrap_or_default()),
        unit: Set(body.unit),
        defect_code: Set(body.defect_code),
        defect_name: Set(body.defect_name),
        defect_level: Set(body.defect_level.unwrap_or(3)),
        defect_description: Set(body.defect_description),
        found_date: Set(body.found_date),
        found_time: Set(body.found_time),
        finder_id: Set(body.finder_id),
        ncr_status: Set(1),
        remark: Set(body.remark),
        created_time: Set(chrono::Utc::now().into()),
        updated_time: Set(chrono::Utc::now().into()),
        is_deleted: Set(0),
        ..Default::default()
    };

    let m = dao::nonconforming_product_dao::create(ctx.db.conn(), active)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(NcrDto {
        id: m.id,
        ncr_no: m.ncr_no,
        material_id: m.material_id,
        batch_no: m.batch_no,
        defect_quantity: m.defect_quantity.to_f64().unwrap_or(0.0),
        unit: m.unit,
        defect_code: m.defect_code,
        defect_name: m.defect_name,
        defect_level: m.defect_level,
        found_date: m.found_date,
        ncr_status: m.ncr_status,
        disposition: m.disposition,
    }))
}

async fn update_ncr(
    State(ctx): State<ApiContext>,
    Path(id): Path<i64>,
    Json(body): Json<NcrPayload>,
) -> Result<Json<NcrDto>, StatusCode> {
    use crate::db::entity::nonconforming_products;
    
    let active = nonconforming_products::ActiveModel {
        ncr_no: Set(body.ncr_no),
        material_id: Set(body.material_id),
        batch_no: Set(body.batch_no),
        defect_quantity: Set(Decimal::from_f64(body.defect_quantity).unwrap_or_default()),
        unit: Set(body.unit),
        defect_code: Set(body.defect_code),
        defect_name: Set(body.defect_name),
        defect_level: Set(body.defect_level.unwrap_or(3)),
        defect_description: Set(body.defect_description),
        remark: Set(body.remark),
        updated_time: Set(chrono::Utc::now().into()),
        ..Default::default()
    };

    let Some(m) = dao::nonconforming_product_dao::update(ctx.db.conn(), id, active)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    else {
        return Err(StatusCode::NOT_FOUND);
    };

    Ok(Json(NcrDto {
        id: m.id,
        ncr_no: m.ncr_no,
        material_id: m.material_id,
        batch_no: m.batch_no,
        defect_quantity: m.defect_quantity.to_f64().unwrap_or(0.0),
        unit: m.unit,
        defect_code: m.defect_code,
        defect_name: m.defect_name,
        defect_level: m.defect_level,
        found_date: m.found_date,
        ncr_status: m.ncr_status,
        disposition: m.disposition,
    }))
}

async fn delete_ncr(
    State(ctx): State<ApiContext>,
    Path(id): Path<i64>,
) -> Result<StatusCode, StatusCode> {
    dao::nonconforming_product_dao::delete(ctx.db.conn(), id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(StatusCode::NO_CONTENT)
}

// ---- Customer Complaints ----

async fn list_complaints(
    State(ctx): State<ApiContext>,
    Query(q): Query<ComplaintQuery>,
) -> Result<Json<quality::PageResult<ComplaintDto>>, StatusCode> {
    let filter = dao::customer_complaint_dao::CustomerComplaintFilter {
        customer_id: q.customer_id,
        complaint_status: q.complaint_status,
        complaint_type: q.complaint_type,
        material_id: q.material_id,
    };
    let (items, total) = dao::customer_complaint_dao::list(ctx.db.conn(), filter, q.page, q.page_size)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mapped = items
        .into_iter()
        .map(|m| ComplaintDto {
            id: m.id,
            complaint_no: m.complaint_no,
            customer_id: m.customer_id,
            material_id: m.material_id,
            batch_no: m.batch_no,
            complaint_type: m.complaint_type,
            complaint_level: m.complaint_level,
            complaint_date: m.complaint_date,
            complaint_status: m.complaint_status,
            handler_id: m.handler_id,
        })
        .collect();

    Ok(Json(quality::PageResult { items: mapped, total }))
}

async fn get_complaint(
    State(ctx): State<ApiContext>,
    Path(id): Path<i64>,
) -> Result<Json<ComplaintDto>, StatusCode> {
    let Some(m) = dao::customer_complaint_dao::get_by_id(ctx.db.conn(), id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    else {
        return Err(StatusCode::NOT_FOUND);
    };
    Ok(Json(ComplaintDto {
        id: m.id,
        complaint_no: m.complaint_no,
        customer_id: m.customer_id,
        material_id: m.material_id,
        batch_no: m.batch_no,
        complaint_type: m.complaint_type,
        complaint_level: m.complaint_level,
        complaint_date: m.complaint_date,
        complaint_status: m.complaint_status,
        handler_id: m.handler_id,
    }))
}

async fn create_complaint(
    State(ctx): State<ApiContext>,
    Json(body): Json<ComplaintPayload>,
) -> Result<Json<ComplaintDto>, StatusCode> {
    use crate::db::entity::customer_complaints;
    
    let active = customer_complaints::ActiveModel {
        complaint_no: Set(body.complaint_no),
        customer_id: Set(body.customer_id),
        material_id: Set(body.material_id),
        batch_no: Set(body.batch_no),
        complaint_type: Set(body.complaint_type),
        complaint_level: Set(body.complaint_level.unwrap_or(3)),
        complaint_date: Set(body.complaint_date),
        complaint_time: Set(body.complaint_time),
        complaint_quantity: Set(body.complaint_quantity.map(|q| Decimal::from_f64(q).unwrap_or_default())),
        unit: Set(body.unit),
        complaint_description: Set(body.complaint_description),
        defect_description: Set(body.defect_description),
        receiver_id: Set(body.receiver_id),
        complaint_status: Set(1),
        is_valid: Set(1),
        remark: Set(body.remark),
        created_time: Set(chrono::Utc::now().into()),
        updated_time: Set(chrono::Utc::now().into()),
        is_deleted: Set(0),
        ..Default::default()
    };

    let m = dao::customer_complaint_dao::create(ctx.db.conn(), active)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(ComplaintDto {
        id: m.id,
        complaint_no: m.complaint_no,
        customer_id: m.customer_id,
        material_id: m.material_id,
        batch_no: m.batch_no,
        complaint_type: m.complaint_type,
        complaint_level: m.complaint_level,
        complaint_date: m.complaint_date,
        complaint_status: m.complaint_status,
        handler_id: m.handler_id,
    }))
}

async fn update_complaint(
    State(ctx): State<ApiContext>,
    Path(id): Path<i64>,
    Json(body): Json<ComplaintPayload>,
) -> Result<Json<ComplaintDto>, StatusCode> {
    use crate::db::entity::customer_complaints;
    
    let active = customer_complaints::ActiveModel {
        complaint_no: Set(body.complaint_no),
        customer_id: Set(body.customer_id),
        material_id: Set(body.material_id),
        batch_no: Set(body.batch_no),
        complaint_type: Set(body.complaint_type),
        complaint_level: Set(body.complaint_level.unwrap_or(3)),
        complaint_description: Set(body.complaint_description),
        defect_description: Set(body.defect_description),
        remark: Set(body.remark),
        updated_time: Set(chrono::Utc::now().into()),
        ..Default::default()
    };

    let Some(m) = dao::customer_complaint_dao::update(ctx.db.conn(), id, active)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    else {
        return Err(StatusCode::NOT_FOUND);
    };

    Ok(Json(ComplaintDto {
        id: m.id,
        complaint_no: m.complaint_no,
        customer_id: m.customer_id,
        material_id: m.material_id,
        batch_no: m.batch_no,
        complaint_type: m.complaint_type,
        complaint_level: m.complaint_level,
        complaint_date: m.complaint_date,
        complaint_status: m.complaint_status,
        handler_id: m.handler_id,
    }))
}

async fn delete_complaint(
    State(ctx): State<ApiContext>,
    Path(id): Path<i64>,
) -> Result<StatusCode, StatusCode> {
    dao::customer_complaint_dao::delete(ctx.db.conn(), id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(StatusCode::NO_CONTENT)
}

// ---- Rework Orders ----

async fn list_rework_orders(
    State(ctx): State<ApiContext>,
    Query(q): Query<ReworkOrderQuery>,
) -> Result<Json<quality::PageResult<ReworkOrderDto>>, StatusCode> {
    let filter = dao::rework_order_dao::ReworkOrderFilter {
        ncr_id: q.ncr_id,
        rework_status: q.rework_status,
        material_id: q.material_id,
    };
    let (items, total) = dao::rework_order_dao::list(ctx.db.conn(), filter, q.page, q.page_size)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mapped = items
        .into_iter()
        .map(|m| ReworkOrderDto {
            id: m.id,
            rework_no: m.rework_no,
            ncr_id: m.ncr_id,
            material_id: m.material_id,
            batch_no: m.batch_no,
            rework_quantity: m.rework_quantity.to_f64().unwrap_or(0.0),
            completed_quantity: m.completed_quantity.to_f64().unwrap_or(0.0),
            qualified_quantity: m.qualified_quantity.to_f64().unwrap_or(0.0),
            unit: m.unit,
            rework_type: m.rework_type,
            rework_status: m.rework_status,
        })
        .collect();

    Ok(Json(quality::PageResult { items: mapped, total }))
}

async fn get_rework_order(
    State(ctx): State<ApiContext>,
    Path(id): Path<i64>,
) -> Result<Json<ReworkOrderDto>, StatusCode> {
    let Some(m) = dao::rework_order_dao::get_by_id(ctx.db.conn(), id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    else {
        return Err(StatusCode::NOT_FOUND);
    };
    Ok(Json(ReworkOrderDto {
        id: m.id,
        rework_no: m.rework_no,
        ncr_id: m.ncr_id,
        material_id: m.material_id,
        batch_no: m.batch_no,
        rework_quantity: m.rework_quantity.to_f64().unwrap_or(0.0),
        completed_quantity: m.completed_quantity.to_f64().unwrap_or(0.0),
        qualified_quantity: m.qualified_quantity.to_f64().unwrap_or(0.0),
        unit: m.unit,
        rework_type: m.rework_type,
        rework_status: m.rework_status,
    }))
}

async fn create_rework_order(
    State(ctx): State<ApiContext>,
    Json(body): Json<ReworkOrderPayload>,
) -> Result<Json<ReworkOrderDto>, StatusCode> {
    use crate::db::entity::rework_orders;
    
    let active = rework_orders::ActiveModel {
        rework_no: Set(body.rework_no),
        ncr_id: Set(body.ncr_id),
        material_id: Set(body.material_id),
        batch_no: Set(body.batch_no),
        rework_quantity: Set(Decimal::from_f64(body.rework_quantity).unwrap_or_default()),
        completed_quantity: Set(Decimal::from(0)),
        qualified_quantity: Set(Decimal::from(0)),
        scrap_quantity: Set(Decimal::from(0)),
        unit: Set(body.unit),
        rework_type: Set(body.rework_type),
        rework_reason: Set(body.rework_reason),
        rework_plan: Set(body.rework_plan),
        workshop_id: Set(body.workshop_id),
        plan_start_date: Set(body.plan_start_date),
        plan_end_date: Set(body.plan_end_date),
        handler_id: Set(body.handler_id),
        rework_status: Set(1),
        remark: Set(body.remark),
        created_time: Set(chrono::Utc::now().into()),
        updated_time: Set(chrono::Utc::now().into()),
        is_deleted: Set(0),
        ..Default::default()
    };

    let m = dao::rework_order_dao::create(ctx.db.conn(), active)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(ReworkOrderDto {
        id: m.id,
        rework_no: m.rework_no,
        ncr_id: m.ncr_id,
        material_id: m.material_id,
        batch_no: m.batch_no,
        rework_quantity: m.rework_quantity.to_f64().unwrap_or(0.0),
        completed_quantity: m.completed_quantity.to_f64().unwrap_or(0.0),
        qualified_quantity: m.qualified_quantity.to_f64().unwrap_or(0.0),
        unit: m.unit,
        rework_type: m.rework_type,
        rework_status: m.rework_status,
    }))
}

async fn update_rework_order(
    State(ctx): State<ApiContext>,
    Path(id): Path<i64>,
    Json(body): Json<ReworkOrderPayload>,
) -> Result<Json<ReworkOrderDto>, StatusCode> {
    use crate::db::entity::rework_orders;
    
    let active = rework_orders::ActiveModel {
        rework_no: Set(body.rework_no),
        ncr_id: Set(body.ncr_id),
        material_id: Set(body.material_id),
        batch_no: Set(body.batch_no),
        rework_quantity: Set(Decimal::from_f64(body.rework_quantity).unwrap_or_default()),
        unit: Set(body.unit),
        rework_type: Set(body.rework_type),
        rework_reason: Set(body.rework_reason),
        rework_plan: Set(body.rework_plan),
        workshop_id: Set(body.workshop_id),
        plan_start_date: Set(body.plan_start_date),
        plan_end_date: Set(body.plan_end_date),
        handler_id: Set(body.handler_id),
        remark: Set(body.remark),
        updated_time: Set(chrono::Utc::now().into()),
        ..Default::default()
    };

    let Some(m) = dao::rework_order_dao::update(ctx.db.conn(), id, active)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    else {
        return Err(StatusCode::NOT_FOUND);
    };

    Ok(Json(ReworkOrderDto {
        id: m.id,
        rework_no: m.rework_no,
        ncr_id: m.ncr_id,
        material_id: m.material_id,
        batch_no: m.batch_no,
        rework_quantity: m.rework_quantity.to_f64().unwrap_or(0.0),
        completed_quantity: m.completed_quantity.to_f64().unwrap_or(0.0),
        qualified_quantity: m.qualified_quantity.to_f64().unwrap_or(0.0),
        unit: m.unit,
        rework_type: m.rework_type,
        rework_status: m.rework_status,
    }))
}

async fn delete_rework_order(
    State(ctx): State<ApiContext>,
    Path(id): Path<i64>,
) -> Result<StatusCode, StatusCode> {
    dao::rework_order_dao::delete(ctx.db.conn(), id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(StatusCode::NO_CONTENT)
}

// ---- Measuring Equipment ----

async fn list_measuring_equipment(
    State(ctx): State<ApiContext>,
    Query(q): Query<MeasuringEquipmentQuery>,
) -> Result<Json<quality::PageResult<MeasuringEquipmentDto>>, StatusCode> {
    let filter = dao::measuring_equipment_dao::MeasuringEquipmentFilter {
        equipment_type: q.equipment_type,
        equipment_status: q.equipment_status,
        keyword: q.keyword.clone(),
    };
    let (items, total) = dao::measuring_equipment_dao::list(ctx.db.conn(), filter, q.page, q.page_size)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mapped = items
        .into_iter()
        .map(|m| MeasuringEquipmentDto {
            id: m.id,
            equipment_code: m.equipment_code,
            equipment_name: m.equipment_name,
            equipment_model: m.equipment_model,
            equipment_type: m.equipment_type,
            equipment_status: m.equipment_status,
            next_calibration_date: m.next_calibration_date,
            location: m.location,
        })
        .collect();

    Ok(Json(quality::PageResult { items: mapped, total }))
}

async fn get_measuring_equipment(
    State(ctx): State<ApiContext>,
    Path(id): Path<i64>,
) -> Result<Json<MeasuringEquipmentDto>, StatusCode> {
    let Some(m) = dao::measuring_equipment_dao::get_by_id(ctx.db.conn(), id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    else {
        return Err(StatusCode::NOT_FOUND);
    };
    Ok(Json(MeasuringEquipmentDto {
        id: m.id,
        equipment_code: m.equipment_code,
        equipment_name: m.equipment_name,
        equipment_model: m.equipment_model,
        equipment_type: m.equipment_type,
        equipment_status: m.equipment_status,
        next_calibration_date: m.next_calibration_date,
        location: m.location,
    }))
}

async fn create_measuring_equipment(
    State(ctx): State<ApiContext>,
    Json(body): Json<MeasuringEquipmentPayload>,
) -> Result<Json<MeasuringEquipmentDto>, StatusCode> {
    use crate::db::entity::measuring_equipment;
    
    let active = measuring_equipment::ActiveModel {
        equipment_code: Set(body.equipment_code),
        equipment_name: Set(body.equipment_name),
        equipment_model: Set(body.equipment_model),
        equipment_type: Set(body.equipment_type),
        manufacturer: Set(None),
        calibration_cycle: Set(body.calibration_cycle.unwrap_or(365)),
        next_calibration_date: Set(body.next_calibration_date),
        equipment_status: Set(body.equipment_status.unwrap_or(1)),
        location: Set(body.location),
        remark: Set(body.remark),
        created_time: Set(chrono::Utc::now().into()),
        updated_time: Set(chrono::Utc::now().into()),
        is_deleted: Set(0),
        ..Default::default()
    };

    let m = dao::measuring_equipment_dao::create(ctx.db.conn(), active)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(MeasuringEquipmentDto {
        id: m.id,
        equipment_code: m.equipment_code,
        equipment_name: m.equipment_name,
        equipment_model: m.equipment_model,
        equipment_type: m.equipment_type,
        equipment_status: m.equipment_status,
        next_calibration_date: m.next_calibration_date,
        location: m.location,
    }))
}

async fn update_measuring_equipment(
    State(ctx): State<ApiContext>,
    Path(id): Path<i64>,
    Json(body): Json<MeasuringEquipmentPayload>,
) -> Result<Json<MeasuringEquipmentDto>, StatusCode> {
    use crate::db::entity::measuring_equipment;
    
    let active = measuring_equipment::ActiveModel {
        equipment_code: Set(body.equipment_code),
        equipment_name: Set(body.equipment_name),
        equipment_model: Set(body.equipment_model),
        equipment_type: Set(body.equipment_type),
        calibration_cycle: Set(body.calibration_cycle),
        next_calibration_date: Set(body.next_calibration_date),
        equipment_status: Set(body.equipment_status),
        location: Set(body.location),
        remark: Set(body.remark),
        updated_time: Set(chrono::Utc::now().into()),
        ..Default::default()
    };

    let Some(m) = dao::measuring_equipment_dao::update(ctx.db.conn(), id, active)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    else {
        return Err(StatusCode::NOT_FOUND);
    };

    Ok(Json(MeasuringEquipmentDto {
        id: m.id,
        equipment_code: m.equipment_code,
        equipment_name: m.equipment_name,
        equipment_model: m.equipment_model,
        equipment_type: m.equipment_type,
        equipment_status: m.equipment_status,
        next_calibration_date: m.next_calibration_date,
        location: m.location,
    }))
}

async fn delete_measuring_equipment(
    State(ctx): State<ApiContext>,
    Path(id): Path<i64>,
) -> Result<StatusCode, StatusCode> {
    dao::measuring_equipment_dao::delete(ctx.db.conn(), id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(StatusCode::NO_CONTENT)
}

// ---- Supplier Quality Evaluations ----

async fn list_supplier_evaluations(
    State(ctx): State<ApiContext>,
    Query(q): Query<SupplierQualityEvaluationQuery>,
) -> Result<Json<quality::PageResult<SupplierQualityEvaluationDto>>, StatusCode> {
    let filter = dao::supplier_quality_evaluation_dao::SupplierQualityEvaluationFilter {
        supplier_id: q.supplier_id,
        evaluation_period: q.evaluation_period.clone(),
    };
    let (items, total) = dao::supplier_quality_evaluation_dao::list(ctx.db.conn(), filter, q.page, q.page_size)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mapped = items
        .into_iter()
        .map(|m| SupplierQualityEvaluationDto {
            id: m.id,
            evaluation_no: m.evaluation_no,
            supplier_id: m.supplier_id,
            evaluation_period: m.evaluation_period,
            evaluation_date: m.evaluation_date,
            batch_qualified_rate: m.batch_qualified_rate.to_f64().unwrap_or(0.0),
            quantity_qualified_rate: m.quantity_qualified_rate.to_f64().unwrap_or(0.0),
            total_score: m.total_score.to_f64().unwrap_or(0.0),
            evaluation_level: m.evaluation_level,
            is_approved: m.is_approved,
        })
        .collect();

    Ok(Json(quality::PageResult { items: mapped, total }))
}

async fn get_supplier_evaluation(
    State(ctx): State<ApiContext>,
    Path(id): Path<i64>,
) -> Result<Json<SupplierQualityEvaluationDto>, StatusCode> {
    let Some(m) = dao::supplier_quality_evaluation_dao::get_by_id(ctx.db.conn(), id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    else {
        return Err(StatusCode::NOT_FOUND);
    };
    Ok(Json(SupplierQualityEvaluationDto {
        id: m.id,
        evaluation_no: m.evaluation_no,
        supplier_id: m.supplier_id,
        evaluation_period: m.evaluation_period,
        evaluation_date: m.evaluation_date,
        batch_qualified_rate: m.batch_qualified_rate.to_f64().unwrap_or(0.0),
        quantity_qualified_rate: m.quantity_qualified_rate.to_f64().unwrap_or(0.0),
        total_score: m.total_score.to_f64().unwrap_or(0.0),
        evaluation_level: m.evaluation_level,
        is_approved: m.is_approved,
    }))
}

async fn create_supplier_evaluation(
    State(ctx): State<ApiContext>,
    Json(body): Json<SupplierQualityEvaluationPayload>,
) -> Result<Json<SupplierQualityEvaluationDto>, StatusCode> {
    use crate::db::entity::supplier_quality_evaluations;
    
    let active = supplier_quality_evaluations::ActiveModel {
        evaluation_no: Set(body.evaluation_no),
        supplier_id: Set(body.supplier_id),
        evaluation_period: Set(body.evaluation_period),
        evaluation_date: Set(body.evaluation_date),
        evaluator_id: Set(body.evaluator_id),
        total_receipts: Set(body.total_receipts.unwrap_or(0)),
        qualified_receipts: Set(body.qualified_receipts.unwrap_or(0)),
        unqualified_receipts: Set(body.unqualified_receipts.unwrap_or(0)),
        batch_qualified_rate: Set(Decimal::from_f64(body.batch_qualified_rate.unwrap_or(0.0)).unwrap_or_default()),
        quantity_qualified_rate: Set(Decimal::from_f64(body.quantity_qualified_rate.unwrap_or(0.0)).unwrap_or_default()),
        on_time_delivery_rate: Set(Decimal::from_f64(body.on_time_delivery_rate.unwrap_or(0.0)).unwrap_or_default()),
        quality_score: Set(Decimal::from_f64(body.quality_score.unwrap_or(0.0)).unwrap_or_default()),
        delivery_score: Set(Decimal::from_f64(body.delivery_score.unwrap_or(0.0)).unwrap_or_default()),
        service_score: Set(Decimal::from_f64(body.service_score.unwrap_or(0.0)).unwrap_or_default()),
        total_score: Set(Decimal::from_f64(body.total_score.unwrap_or(0.0)).unwrap_or_default()),
        evaluation_level: Set(body.evaluation_level.unwrap_or_else(|| "C".to_string())),
        evaluation_conclusion: Set(body.evaluation_conclusion),
        remark: Set(body.remark),
        created_time: Set(chrono::Utc::now().into()),
        updated_time: Set(chrono::Utc::now().into()),
        is_deleted: Set(0),
        ..Default::default()
    };

    let m = dao::supplier_quality_evaluation_dao::create(ctx.db.conn(), active)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(SupplierQualityEvaluationDto {
        id: m.id,
        evaluation_no: m.evaluation_no,
        supplier_id: m.supplier_id,
        evaluation_period: m.evaluation_period,
        evaluation_date: m.evaluation_date,
        batch_qualified_rate: m.batch_qualified_rate.to_f64().unwrap_or(0.0),
        quantity_qualified_rate: m.quantity_qualified_rate.to_f64().unwrap_or(0.0),
        total_score: m.total_score.to_f64().unwrap_or(0.0),
        evaluation_level: m.evaluation_level,
        is_approved: m.is_approved,
    }))
}

async fn update_supplier_evaluation(
    State(ctx): State<ApiContext>,
    Path(id): Path<i64>,
    Json(body): Json<SupplierQualityEvaluationPayload>,
) -> Result<Json<SupplierQualityEvaluationDto>, StatusCode> {
    use crate::db::entity::supplier_quality_evaluations;
    
    let active = supplier_quality_evaluations::ActiveModel {
        evaluation_no: Set(body.evaluation_no),
        supplier_id: Set(body.supplier_id),
        evaluation_period: Set(body.evaluation_period),
        evaluation_date: Set(body.evaluation_date),
        batch_qualified_rate: Set(Decimal::from_f64(body.batch_qualified_rate.unwrap_or(0.0)).unwrap_or_default()),
        quantity_qualified_rate: Set(Decimal::from_f64(body.quantity_qualified_rate.unwrap_or(0.0)).unwrap_or_default()),
        total_score: Set(Decimal::from_f64(body.total_score.unwrap_or(0.0)).unwrap_or_default()),
        evaluation_level: Set(body.evaluation_level.unwrap_or_else(|| "C".to_string())),
        evaluation_conclusion: Set(body.evaluation_conclusion),
        remark: Set(body.remark),
        updated_time: Set(chrono::Utc::now().into()),
        ..Default::default()
    };

    let Some(m) = dao::supplier_quality_evaluation_dao::update(ctx.db.conn(), id, active)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    else {
        return Err(StatusCode::NOT_FOUND);
    };

    Ok(Json(SupplierQualityEvaluationDto {
        id: m.id,
        evaluation_no: m.evaluation_no,
        supplier_id: m.supplier_id,
        evaluation_period: m.evaluation_period,
        evaluation_date: m.evaluation_date,
        batch_qualified_rate: m.batch_qualified_rate.to_f64().unwrap_or(0.0),
        quantity_qualified_rate: m.quantity_qualified_rate.to_f64().unwrap_or(0.0),
        total_score: m.total_score.to_f64().unwrap_or(0.0),
        evaluation_level: m.evaluation_level,
        is_approved: m.is_approved,
    }))
}

async fn delete_supplier_evaluation(
    State(ctx): State<ApiContext>,
    Path(id): Path<i64>,
) -> Result<StatusCode, StatusCode> {
    dao::supplier_quality_evaluation_dao::delete(ctx.db.conn(), id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(StatusCode::NO_CONTENT)
}

// ---- Quality Traceability Records ----

async fn list_traceability_records(
    State(ctx): State<ApiContext>,
    Query(q): Query<QualityTraceabilityRecordQuery>,
) -> Result<Json<quality::PageResult<QualityTraceabilityRecordDto>>, StatusCode> {
    let filter = dao::quality_traceability_record_dao::QualityTraceabilityRecordFilter {
        material_id: q.material_id,
        batch_no: q.batch_no.clone(),
        trace_type: q.trace_type,
    };
    let (items, total) = dao::quality_traceability_record_dao::list(ctx.db.conn(), filter, q.page, q.page_size)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mapped = items
        .into_iter()
        .map(|m| QualityTraceabilityRecordDto {
            id: m.id,
            trace_no: m.trace_no,
            trace_type: m.trace_type,
            material_id: m.material_id,
            batch_no: m.batch_no,
            serial_no: m.serial_no,
            production_order_no: m.production_order_no,
            trace_date: m.trace_date,
            trace_result: m.trace_result,
        })
        .collect();

    Ok(Json(quality::PageResult { items: mapped, total }))
}

async fn get_traceability_record(
    State(ctx): State<ApiContext>,
    Path(id): Path<i64>,
) -> Result<Json<QualityTraceabilityRecordDto>, StatusCode> {
    let Some(m) = dao::quality_traceability_record_dao::get_by_id(ctx.db.conn(), id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    else {
        return Err(StatusCode::NOT_FOUND);
    };
    Ok(Json(QualityTraceabilityRecordDto {
        id: m.id,
        trace_no: m.trace_no,
        trace_type: m.trace_type,
        material_id: m.material_id,
        batch_no: m.batch_no,
        serial_no: m.serial_no,
        production_order_no: m.production_order_no,
        trace_date: m.trace_date,
        trace_result: m.trace_result,
    }))
}

async fn create_traceability_record(
    State(ctx): State<ApiContext>,
    Json(body): Json<QualityTraceabilityRecordPayload>,
) -> Result<Json<QualityTraceabilityRecordDto>, StatusCode> {
    use crate::db::entity::quality_traceability_records;
    
    let active = quality_traceability_records::ActiveModel {
        trace_no: Set(body.trace_no),
        trace_type: Set(body.trace_type),
        material_id: Set(body.material_id),
        batch_no: Set(body.batch_no),
        serial_no: Set(body.serial_no),
        production_order_no: Set(body.production_order_no),
        trace_reason: Set(body.trace_reason),
        trace_date: Set(body.trace_date),
        tracer_id: Set(body.tracer_id),
        remark: Set(body.remark),
        created_time: Set(chrono::Utc::now().into()),
        updated_time: Set(chrono::Utc::now().into()),
        is_deleted: Set(0),
        ..Default::default()
    };

    let m = dao::quality_traceability_record_dao::create(ctx.db.conn(), active)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(QualityTraceabilityRecordDto {
        id: m.id,
        trace_no: m.trace_no,
        trace_type: m.trace_type,
        material_id: m.material_id,
        batch_no: m.batch_no,
        serial_no: m.serial_no,
        production_order_no: m.production_order_no,
        trace_date: m.trace_date,
        trace_result: m.trace_result,
    }))
}

async fn update_traceability_record(
    State(ctx): State<ApiContext>,
    Path(id): Path<i64>,
    Json(body): Json<QualityTraceabilityRecordPayload>,
) -> Result<Json<QualityTraceabilityRecordDto>, StatusCode> {
    use crate::db::entity::quality_traceability_records;
    
    let active = quality_traceability_records::ActiveModel {
        trace_no: Set(body.trace_no),
        trace_type: Set(body.trace_type),
        material_id: Set(body.material_id),
        batch_no: Set(body.batch_no),
        serial_no: Set(body.serial_no),
        production_order_no: Set(body.production_order_no),
        trace_reason: Set(body.trace_reason),
        trace_date: Set(body.trace_date),
        remark: Set(body.remark),
        updated_time: Set(chrono::Utc::now().into()),
        ..Default::default()
    };

    let Some(m) = dao::quality_traceability_record_dao::update(ctx.db.conn(), id, active)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    else {
        return Err(StatusCode::NOT_FOUND);
    };

    Ok(Json(QualityTraceabilityRecordDto {
        id: m.id,
        trace_no: m.trace_no,
        trace_type: m.trace_type,
        material_id: m.material_id,
        batch_no: m.batch_no,
        serial_no: m.serial_no,
        production_order_no: m.production_order_no,
        trace_date: m.trace_date,
        trace_result: m.trace_result,
    }))
}

async fn delete_traceability_record(
    State(ctx): State<ApiContext>,
    Path(id): Path<i64>,
) -> Result<StatusCode, StatusCode> {
    dao::quality_traceability_record_dao::delete(ctx.db.conn(), id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(StatusCode::NO_CONTENT)
}

// ---- Quality Costs ----

async fn list_quality_costs(
    State(ctx): State<ApiContext>,
    Query(q): Query<QualityCostQuery>,
) -> Result<Json<quality::PageResult<QualityCostDto>>, StatusCode> {
    let filter = dao::quality_cost_dao::QualityCostFilter {
        cost_period: q.cost_period.clone(),
        cost_category: q.cost_category,
    };
    let (items, total) = dao::quality_cost_dao::list(ctx.db.conn(), filter, q.page, q.page_size)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mapped = items
        .into_iter()
        .map(|m| QualityCostDto {
            id: m.id,
            cost_no: m.cost_no,
            cost_period: m.cost_period,
            cost_date: m.cost_date,
            cost_category: m.cost_category,
            cost_type: m.cost_type,
            cost_item: m.cost_item,
            cost_amount: m.cost_amount.to_f64().unwrap_or(0.0),
        })
        .collect();

    Ok(Json(quality::PageResult { items: mapped, total }))
}

async fn get_quality_cost(
    State(ctx): State<ApiContext>,
    Path(id): Path<i64>,
) -> Result<Json<QualityCostDto>, StatusCode> {
    let Some(m) = dao::quality_cost_dao::get_by_id(ctx.db.conn(), id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    else {
        return Err(StatusCode::NOT_FOUND);
    };
    Ok(Json(QualityCostDto {
        id: m.id,
        cost_no: m.cost_no,
        cost_period: m.cost_period,
        cost_date: m.cost_date,
        cost_category: m.cost_category,
        cost_type: m.cost_type,
        cost_item: m.cost_item,
        cost_amount: m.cost_amount.to_f64().unwrap_or(0.0),
    }))
}

async fn create_quality_cost(
    State(ctx): State<ApiContext>,
    Json(body): Json<QualityCostPayload>,
) -> Result<Json<QualityCostDto>, StatusCode> {
    use crate::db::entity::quality_costs;
    
    let active = quality_costs::ActiveModel {
        cost_no: Set(body.cost_no),
        cost_period: Set(body.cost_period),
        cost_date: Set(body.cost_date),
        cost_category: Set(body.cost_category),
        cost_type: Set(body.cost_type),
        cost_item: Set(body.cost_item),
        cost_amount: Set(Decimal::from_f64(body.cost_amount).unwrap_or_default()),
        cost_description: Set(body.cost_description),
        remark: Set(body.remark),
        created_time: Set(chrono::Utc::now().into()),
        updated_time: Set(chrono::Utc::now().into()),
        is_deleted: Set(0),
        ..Default::default()
    };

    let m = dao::quality_cost_dao::create(ctx.db.conn(), active)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(QualityCostDto {
        id: m.id,
        cost_no: m.cost_no,
        cost_period: m.cost_period,
        cost_date: m.cost_date,
        cost_category: m.cost_category,
        cost_type: m.cost_type,
        cost_item: m.cost_item,
        cost_amount: m.cost_amount.to_f64().unwrap_or(0.0),
    }))
}

async fn update_quality_cost(
    State(ctx): State<ApiContext>,
    Path(id): Path<i64>,
    Json(body): Json<QualityCostPayload>,
) -> Result<Json<QualityCostDto>, StatusCode> {
    use crate::db::entity::quality_costs;
    
    let active = quality_costs::ActiveModel {
        cost_no: Set(body.cost_no),
        cost_period: Set(body.cost_period),
        cost_date: Set(body.cost_date),
        cost_category: Set(body.cost_category),
        cost_type: Set(body.cost_type),
        cost_item: Set(body.cost_item),
        cost_amount: Set(Decimal::from_f64(body.cost_amount).unwrap_or_default()),
        cost_description: Set(body.cost_description),
        remark: Set(body.remark),
        updated_time: Set(chrono::Utc::now().into()),
        ..Default::default()
    };

    let Some(m) = dao::quality_cost_dao::update(ctx.db.conn(), id, active)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    else {
        return Err(StatusCode::NOT_FOUND);
    };

    Ok(Json(QualityCostDto {
        id: m.id,
        cost_no: m.cost_no,
        cost_period: m.cost_period,
        cost_date: m.cost_date,
        cost_category: m.cost_category,
        cost_type: m.cost_type,
        cost_item: m.cost_item,
        cost_amount: m.cost_amount.to_f64().unwrap_or(0.0),
    }))
}

async fn delete_quality_cost(
    State(ctx): State<ApiContext>,
    Path(id): Path<i64>,
) -> Result<StatusCode, StatusCode> {
    dao::quality_cost_dao::delete(ctx.db.conn(), id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(StatusCode::NO_CONTENT)
}

// ---- Quality KPI ----

async fn list_quality_kpi(
    State(ctx): State<ApiContext>,
    Query(q): Query<QualityKpiQuery>,
) -> Result<Json<quality::PageResult<QualityKpiDto>>, StatusCode> {
    let filter = dao::quality_kpi_dao::QualityKpiFilter {
        kpi_type: q.kpi_type,
        dept_id: q.dept_id,
        workshop_id: q.workshop_id,
        start_date: q.start_date,
        end_date: q.end_date,
    };
    let (items, total) = dao::quality_kpi_dao::list(ctx.db.conn(), filter, q.page, q.page_size)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mapped = items
        .into_iter()
        .map(|m| QualityKpiDto {
            id: m.id,
            kpi_date: m.kpi_date,
            kpi_type: m.kpi_type,
            dept_id: m.dept_id,
            workshop_id: m.workshop_id,
            batch_qualified_rate: m.batch_qualified_rate.to_f64().unwrap_or(0.0),
            quantity_qualified_rate: m.quantity_qualified_rate.to_f64().unwrap_or(0.0),
            first_pass_yield: m.first_pass_yield.to_f64().unwrap_or(0.0),
            iqc_qualified_rate: m.iqc_qualified_rate.to_f64().unwrap_or(0.0),
            ipqc_qualified_rate: m.ipqc_qualified_rate.to_f64().unwrap_or(0.0),
            fqc_qualified_rate: m.fqc_qualified_rate.to_f64().unwrap_or(0.0),
            oqc_qualified_rate: m.oqc_qualified_rate.to_f64().unwrap_or(0.0),
            rework_rate: m.rework_rate.to_f64().unwrap_or(0.0),
            scrap_rate: m.scrap_rate.to_f64().unwrap_or(0.0),
            complaint_rate: m.complaint_rate.to_f64().unwrap_or(0.0),
            ncr_count: m.ncr_count,
            total_quality_cost: m.total_quality_cost.to_f64().unwrap_or(0.0),
            quality_cost_rate: m.quality_cost_rate.to_f64().unwrap_or(0.0),
            dppm: m.dppm.to_f64().unwrap_or(0.0),
            cpk: m.cpk.map(|v| v.to_f64().unwrap_or(0.0)),
            sigma_level: m.sigma_level.map(|v| v.to_f64().unwrap_or(0.0)),
        })
        .collect();

    Ok(Json(quality::PageResult { items: mapped, total }))
}

async fn get_quality_kpi(
    State(ctx): State<ApiContext>,
    Path(id): Path<i64>,
) -> Result<Json<QualityKpiDto>, StatusCode> {
    let Some(m) = dao::quality_kpi_dao::get_by_id(ctx.db.conn(), id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    else {
        return Err(StatusCode::NOT_FOUND);
    };
    Ok(Json(QualityKpiDto {
        id: m.id,
        kpi_date: m.kpi_date,
        kpi_type: m.kpi_type,
        dept_id: m.dept_id,
        workshop_id: m.workshop_id,
        batch_qualified_rate: m.batch_qualified_rate.to_f64().unwrap_or(0.0),
        quantity_qualified_rate: m.quantity_qualified_rate.to_f64().unwrap_or(0.0),
        first_pass_yield: m.first_pass_yield.to_f64().unwrap_or(0.0),
        iqc_qualified_rate: m.iqc_qualified_rate.to_f64().unwrap_or(0.0),
        ipqc_qualified_rate: m.ipqc_qualified_rate.to_f64().unwrap_or(0.0),
        fqc_qualified_rate: m.fqc_qualified_rate.to_f64().unwrap_or(0.0),
        oqc_qualified_rate: m.oqc_qualified_rate.to_f64().unwrap_or(0.0),
        rework_rate: m.rework_rate.to_f64().unwrap_or(0.0),
        scrap_rate: m.scrap_rate.to_f64().unwrap_or(0.0),
        complaint_rate: m.complaint_rate.to_f64().unwrap_or(0.0),
        ncr_count: m.ncr_count,
        total_quality_cost: m.total_quality_cost.to_f64().unwrap_or(0.0),
        quality_cost_rate: m.quality_cost_rate.to_f64().unwrap_or(0.0),
        dppm: m.dppm.to_f64().unwrap_or(0.0),
        cpk: m.cpk.map(|v| v.to_f64().unwrap_or(0.0)),
        sigma_level: m.sigma_level.map(|v| v.to_f64().unwrap_or(0.0)),
    }))
}

async fn create_quality_kpi(
    State(ctx): State<ApiContext>,
    Json(body): Json<QualityKpiDto>,
) -> Result<Json<QualityKpiDto>, StatusCode> {
    use crate::db::entity::quality_kpi;
    
    let active = quality_kpi::ActiveModel {
        kpi_date: Set(body.kpi_date),
        kpi_type: Set(body.kpi_type),
        dept_id: Set(body.dept_id),
        workshop_id: Set(body.workshop_id),
        batch_qualified_rate: Set(Decimal::from_f64(body.batch_qualified_rate).unwrap_or_default()),
        quantity_qualified_rate: Set(Decimal::from_f64(body.quantity_qualified_rate).unwrap_or_default()),
        first_pass_yield: Set(Decimal::from_f64(body.first_pass_yield).unwrap_or_default()),
        iqc_qualified_rate: Set(Decimal::from_f64(body.iqc_qualified_rate).unwrap_or_default()),
        ipqc_qualified_rate: Set(Decimal::from_f64(body.ipqc_qualified_rate).unwrap_or_default()),
        fqc_qualified_rate: Set(Decimal::from_f64(body.fqc_qualified_rate).unwrap_or_default()),
        oqc_qualified_rate: Set(Decimal::from_f64(body.oqc_qualified_rate).unwrap_or_default()),
        rework_rate: Set(Decimal::from_f64(body.rework_rate).unwrap_or_default()),
        scrap_rate: Set(Decimal::from_f64(body.scrap_rate).unwrap_or_default()),
        complaint_rate: Set(Decimal::from_f64(body.complaint_rate).unwrap_or_default()),
        ncr_count: Set(body.ncr_count),
        total_quality_cost: Set(Decimal::from_f64(body.total_quality_cost).unwrap_or_default()),
        quality_cost_rate: Set(Decimal::from_f64(body.quality_cost_rate).unwrap_or_default()),
        dppm: Set(Decimal::from_f64(body.dppm).unwrap_or_default()),
        cpk: Set(body.cpk.map(|v| Decimal::from_f64(v).unwrap_or_default())),
        sigma_level: Set(body.sigma_level.map(|v| Decimal::from_f64(v).unwrap_or_default())),
        created_time: Set(chrono::Utc::now().into()),
        updated_time: Set(chrono::Utc::now().into()),
        ..Default::default()
    };

    let m = dao::quality_kpi_dao::create(ctx.db.conn(), active)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(QualityKpiDto {
        id: m.id,
        kpi_date: m.kpi_date,
        kpi_type: m.kpi_type,
        dept_id: m.dept_id,
        workshop_id: m.workshop_id,
        batch_qualified_rate: m.batch_qualified_rate.to_f64().unwrap_or(0.0),
        quantity_qualified_rate: m.quantity_qualified_rate.to_f64().unwrap_or(0.0),
        first_pass_yield: m.first_pass_yield.to_f64().unwrap_or(0.0),
        iqc_qualified_rate: m.iqc_qualified_rate.to_f64().unwrap_or(0.0),
        ipqc_qualified_rate: m.ipqc_qualified_rate.to_f64().unwrap_or(0.0),
        fqc_qualified_rate: m.fqc_qualified_rate.to_f64().unwrap_or(0.0),
        oqc_qualified_rate: m.oqc_qualified_rate.to_f64().unwrap_or(0.0),
        rework_rate: m.rework_rate.to_f64().unwrap_or(0.0),
        scrap_rate: m.scrap_rate.to_f64().unwrap_or(0.0),
        complaint_rate: m.complaint_rate.to_f64().unwrap_or(0.0),
        ncr_count: m.ncr_count,
        total_quality_cost: m.total_quality_cost.to_f64().unwrap_or(0.0),
        quality_cost_rate: m.quality_cost_rate.to_f64().unwrap_or(0.0),
        dppm: m.dppm.to_f64().unwrap_or(0.0),
        cpk: m.cpk.map(|v| v.to_f64().unwrap_or(0.0)),
        sigma_level: m.sigma_level.map(|v| v.to_f64().unwrap_or(0.0)),
    }))
}

async fn update_quality_kpi(
    State(ctx): State<ApiContext>,
    Path(id): Path<i64>,
    Json(body): Json<QualityKpiDto>,
) -> Result<Json<QualityKpiDto>, StatusCode> {
    use crate::db::entity::quality_kpi;
    
    let active = quality_kpi::ActiveModel {
        kpi_date: Set(body.kpi_date),
        kpi_type: Set(body.kpi_type),
        dept_id: Set(body.dept_id),
        workshop_id: Set(body.workshop_id),
        batch_qualified_rate: Set(Decimal::from_f64(body.batch_qualified_rate).unwrap_or_default()),
        quantity_qualified_rate: Set(Decimal::from_f64(body.quantity_qualified_rate).unwrap_or_default()),
        first_pass_yield: Set(Decimal::from_f64(body.first_pass_yield).unwrap_or_default()),
        iqc_qualified_rate: Set(Decimal::from_f64(body.iqc_qualified_rate).unwrap_or_default()),
        ipqc_qualified_rate: Set(Decimal::from_f64(body.ipqc_qualified_rate).unwrap_or_default()),
        fqc_qualified_rate: Set(Decimal::from_f64(body.fqc_qualified_rate).unwrap_or_default()),
        oqc_qualified_rate: Set(Decimal::from_f64(body.oqc_qualified_rate).unwrap_or_default()),
        rework_rate: Set(Decimal::from_f64(body.rework_rate).unwrap_or_default()),
        scrap_rate: Set(Decimal::from_f64(body.scrap_rate).unwrap_or_default()),
        complaint_rate: Set(Decimal::from_f64(body.complaint_rate).unwrap_or_default()),
        ncr_count: Set(body.ncr_count),
        total_quality_cost: Set(Decimal::from_f64(body.total_quality_cost).unwrap_or_default()),
        quality_cost_rate: Set(Decimal::from_f64(body.quality_cost_rate).unwrap_or_default()),
        dppm: Set(Decimal::from_f64(body.dppm).unwrap_or_default()),
        cpk: Set(body.cpk.map(|v| Decimal::from_f64(v).unwrap_or_default())),
        sigma_level: Set(body.sigma_level.map(|v| Decimal::from_f64(v).unwrap_or_default())),
        updated_time: Set(chrono::Utc::now().into()),
        ..Default::default()
    };

    let Some(m) = dao::quality_kpi_dao::update(ctx.db.conn(), id, active)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    else {
        return Err(StatusCode::NOT_FOUND);
    };

    Ok(Json(QualityKpiDto {
        id: m.id,
        kpi_date: m.kpi_date,
        kpi_type: m.kpi_type,
        dept_id: m.dept_id,
        workshop_id: m.workshop_id,
        batch_qualified_rate: m.batch_qualified_rate.to_f64().unwrap_or(0.0),
        quantity_qualified_rate: m.quantity_qualified_rate.to_f64().unwrap_or(0.0),
        first_pass_yield: m.first_pass_yield.to_f64().unwrap_or(0.0),
        iqc_qualified_rate: m.iqc_qualified_rate.to_f64().unwrap_or(0.0),
        ipqc_qualified_rate: m.ipqc_qualified_rate.to_f64().unwrap_or(0.0),
        fqc_qualified_rate: m.fqc_qualified_rate.to_f64().unwrap_or(0.0),
        oqc_qualified_rate: m.oqc_qualified_rate.to_f64().unwrap_or(0.0),
        rework_rate: m.rework_rate.to_f64().unwrap_or(0.0),
        scrap_rate: m.scrap_rate.to_f64().unwrap_or(0.0),
        complaint_rate: m.complaint_rate.to_f64().unwrap_or(0.0),
        ncr_count: m.ncr_count,
        total_quality_cost: m.total_quality_cost.to_f64().unwrap_or(0.0),
        quality_cost_rate: m.quality_cost_rate.to_f64().unwrap_or(0.0),
        dppm: m.dppm.to_f64().unwrap_or(0.0),
        cpk: m.cpk.map(|v| v.to_f64().unwrap_or(0.0)),
        sigma_level: m.sigma_level.map(|v| v.to_f64().unwrap_or(0.0)),
    }))
}

