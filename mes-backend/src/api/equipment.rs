use crate::api::ApiContext;
use crate::db::dao;
use crate::model::equipment::*;
use crate::service::mqtt_service::MqttService;
use axum::{
    extract::{Path, Query, State},
    http::{HeaderMap, StatusCode},
    Json,
};
use num_traits::cast::ToPrimitive;
use sea_orm::prelude::Decimal;
use sea_orm::ActiveValue::Set;
use chrono::Utc;
use rust_decimal::prelude::FromPrimitive;

pub fn router() -> axum::Router<ApiContext> {
    use axum::routing::{delete, get, post, put};

    axum::Router::new()
        .route("/equipment", get(list_equipment).post(create_equipment))
        .route(
            "/equipment/:id",
            get(get_equipment).put(update_equipment).delete(delete_equipment),
        )
        .route(
            "/equipment/:id/mqtt-config",
            get(get_mqtt_config).post(save_mqtt_config),
        )
        .route("/equipment/:id/status", get(get_realtime_status))
        .route("/equipment/status-log", get(list_status_log))
        .route("/equipment/:id/control", post(control_equipment))
        .route(
            "/equipment/maintenance-plans",
            get(list_maintenance_plans).post(create_maintenance_plan),
        )
        .route(
            "/equipment/maintenance-plans/:id",
            put(update_maintenance_plan).delete(delete_maintenance_plan),
        )
        .route(
            "/equipment/maintenance-tasks",
            get(list_maintenance_tasks).post(create_maintenance_task),
        )
        .route(
            "/equipment/maintenance-tasks/:id",
            put(update_maintenance_task).delete(delete_maintenance_task),
        )
        .route(
            "/equipment/fault-reports",
            get(list_fault_reports).post(create_fault_report),
        )
        .route(
            "/equipment/fault-reports/:id",
            put(update_fault_report).delete(delete_fault_report),
        )
        .route(
            "/equipment/repair-orders",
            get(list_repair_orders).post(create_repair_order),
        )
        .route(
            "/equipment/repair-orders/:id",
            put(update_repair_order).delete(delete_repair_order),
        )
        .route(
            "/equipment/inspections",
            get(list_inspections).post(create_inspection),
        )
        .route(
            "/equipment/inspections/:id",
            put(update_inspection).delete(delete_inspection),
        )
        .route("/equipment/kpi", get(list_equipment_kpi))
}

async fn list_equipment(
    State(ctx): State<ApiContext>,
    Query(q): Query<EquipmentListQuery>,
) -> Result<Json<PageResult<EquipmentDetail>>, StatusCode> {
    let filter = dao::equipment_dao::EquipmentFilter { status: q.status };
    let (items, total) = dao::equipment_dao::list(ctx.db.conn(), filter, q.page, q.page_size)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mapped = items
        .into_iter()
        .map(|m| EquipmentDetail {
            id: m.id,
            equipment_code: m.equipment_code,
            equipment_name: m.equipment_name,
            equipment_type: m.equipment_type,
            model: m.model,
            factory: m.factory,
            production_date: m.production_date.map(|d| d),
            install_date: m.install_date.map(|d| d),
            status: m.status,
            ip_address: m.ip_address,
            mqtt_topic: m.mqtt_topic,
            location: m.location,
            responsible_person: m.responsible_person,
            remark: m.remark,
        })
        .collect();

    Ok(Json(PageResult { items: mapped, total }))
}

async fn get_equipment(
    State(ctx): State<ApiContext>,
    Path(id): Path<i32>,
) -> Result<Json<EquipmentDetail>, StatusCode> {
    let Some(m) = dao::equipment_dao::get_by_id(ctx.db.conn(), id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    else {
        return Err(StatusCode::NOT_FOUND);
    };

    Ok(Json(EquipmentDetail {
        id: m.id,
        equipment_code: m.equipment_code,
        equipment_name: m.equipment_name,
        equipment_type: m.equipment_type,
        model: m.model,
        factory: m.factory,
        production_date: m.production_date.map(|d| d),
        install_date: m.install_date.map(|d| d),
        status: m.status,
        ip_address: m.ip_address,
        mqtt_topic: m.mqtt_topic,
        location: m.location,
        responsible_person: m.responsible_person,
        remark: m.remark,
    }))
}

async fn create_equipment(
    State(ctx): State<ApiContext>,
    Json(body): Json<EquipmentCreateOrUpdate>,
) -> Result<Json<EquipmentDetail>, StatusCode> {
    let active = crate::db::entity::equipment::ActiveModel {
        equipment_code: Set(body.equipment_code),
        equipment_name: Set(body.equipment_name),
        equipment_type: Set(body.equipment_type),
        model: Set(body.model),
        factory: Set(body.factory),
        production_date: Set(body.production_date.map(|d| d.and_hms_opt(0, 0, 0).unwrap().into())),
        install_date: Set(body.install_date.map(|d| d.and_hms_opt(0, 0, 0).unwrap().into())),
        status: Set(body.status.unwrap_or(0)),
        ip_address: Set(body.ip_address),
        mqtt_topic: Set(body.mqtt_topic),
        location: Set(body.location),
        responsible_person: Set(body.responsible_person),
        remark: Set(body.remark),
        ..Default::default()
    };

    let m = dao::equipment_dao::create(ctx.db.conn(), active)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(EquipmentDetail {
        id: m.id,
        equipment_code: m.equipment_code,
        equipment_name: m.equipment_name,
        equipment_type: m.equipment_type,
        model: m.model,
        factory: m.factory,
        production_date: m.production_date.map(|d| d),
        install_date: m.install_date.map(|d| d),
        status: m.status,
        ip_address: m.ip_address,
        mqtt_topic: m.mqtt_topic,
        location: m.location,
        responsible_person: m.responsible_person,
        remark: m.remark,
    }))
}

async fn update_equipment(
    State(ctx): State<ApiContext>,
    Path(id): Path<i32>,
    Json(body): Json<EquipmentCreateOrUpdate>,
) -> Result<Json<EquipmentDetail>, StatusCode> {
    let active = crate::db::entity::equipment::ActiveModel {
        equipment_code: Set(body.equipment_code),
        equipment_name: Set(body.equipment_name),
        equipment_type: Set(body.equipment_type),
        model: Set(body.model),
        factory: Set(body.factory),
        production_date: Set(body.production_date.map(|d| d.and_hms_opt(0, 0, 0).unwrap().into())),
        install_date: Set(body.install_date.map(|d| d.and_hms_opt(0, 0, 0).unwrap().into())),
        status: Set(body.status.unwrap_or(0)),
        ip_address: Set(body.ip_address),
        mqtt_topic: Set(body.mqtt_topic),
        location: Set(body.location),
        responsible_person: Set(body.responsible_person),
        remark: Set(body.remark),
        ..Default::default()
    };

    let Some(m) = dao::equipment_dao::update(ctx.db.conn(), id, active)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    else {
        return Err(StatusCode::NOT_FOUND);
    };

    Ok(Json(EquipmentDetail {
        id: m.id,
        equipment_code: m.equipment_code,
        equipment_name: m.equipment_name,
        equipment_type: m.equipment_type,
        model: m.model,
        factory: m.factory,
        production_date: m.production_date.map(|d| d),
        install_date: m.install_date.map(|d| d),
        status: m.status,
        ip_address: m.ip_address,
        mqtt_topic: m.mqtt_topic,
        location: m.location,
        responsible_person: m.responsible_person,
        remark: m.remark,
    }))
}

async fn delete_equipment(
    State(ctx): State<ApiContext>,
    Path(id): Path<i32>,
) -> Result<StatusCode, StatusCode> {
    let rows = dao::equipment_dao::delete(ctx.db.conn(), id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    if rows == 0 {
        return Err(StatusCode::NOT_FOUND);
    }
    Ok(StatusCode::NO_CONTENT)
}

async fn get_mqtt_config(
    State(ctx): State<ApiContext>,
    Path(id): Path<i32>,
) -> Result<Json<EquipmentMqttConfigDto>, StatusCode> {
    let Some(m) = dao::equipment_mqtt_config_dao::get_by_equipment_id(ctx.db.conn(), id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    else {
        return Err(StatusCode::NOT_FOUND);
    };

    Ok(Json(EquipmentMqttConfigDto {
        broker_address: m.broker_address,
        username: m.username,
        password: m.password,
        client_id: m.client_id,
        keep_alive: m.keep_alive,
        qos: m.qos,
    }))
}

async fn save_mqtt_config(
    State(ctx): State<ApiContext>,
    Path(id): Path<i32>,
    Json(body): Json<EquipmentMqttConfigDto>,
) -> Result<Json<EquipmentMqttConfigDto>, StatusCode> {
    let active = crate::db::entity::equipment_mqtt_config::ActiveModel {
        broker_address: Set(body.broker_address.clone()),
        username: Set(body.username.clone()),
        password: Set(body.password.clone()),
        client_id: Set(body.client_id.clone()),
        keep_alive: Set(body.keep_alive),
        qos: Set(body.qos),
        ..Default::default()
    };

    let m = dao::equipment_mqtt_config_dao::upsert(ctx.db.conn(), id, active)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(EquipmentMqttConfigDto {
        broker_address: m.broker_address,
        username: m.username,
        password: m.password,
        client_id: m.client_id,
        keep_alive: m.keep_alive,
        qos: m.qos,
    }))
}

async fn get_realtime_status(
    State(ctx): State<ApiContext>,
    Path(id): Path<i32>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    // 优先从 Redis 取
    let key = format!("{}{}", crate::cache::EQUIPMENT_STATUS_KEY_PREFIX, id);
    let mut conn = ctx.cache.manager().as_ref().clone();
    let val: Option<String> = redis::AsyncCommands::get(&mut conn, key)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if let Some(json) = val {
        let v: serde_json::Value =
            serde_json::from_str(&json).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        return Ok(Json(v));
    }

    // Redis 没有命中时，回退到数据库最近一条状态日志
    let (logs, _) = dao::equipment_status_log_dao::list(
        ctx.db.conn(),
        dao::equipment_status_log_dao::StatusLogFilter {
            equipment_id: Some(id),
            status: None,
            start_time: None,
            end_time: None,
        },
        0,
        1,
    )
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if let Some(last) = logs.into_iter().next() {
        return Ok(Json(serde_json::json!({
            "equipment_id": last.equipment_id,
            "status": last.status,
            "running_param": last.running_param,
            "error_code": last.error_code,
            "error_desc": last.error_desc,
            "log_time": last.log_time,
            "source": "db_fallback"
        })));
    }

    Err(StatusCode::NOT_FOUND)
}

async fn list_status_log(
    State(ctx): State<ApiContext>,
    Query(q): Query<StatusLogQuery>,
) -> Result<Json<PageResult<EquipmentStatusLogDto>>, StatusCode> {
    let filter = dao::equipment_status_log_dao::StatusLogFilter {
        equipment_id: q.equipment_id,
        status: q.status,
        start_time: q.start_time,
        end_time: q.end_time,
    };

    let (items, total) = dao::equipment_status_log_dao::list(
        ctx.db.conn(),
        filter,
        q.page,
        q.page_size,
    )
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mapped = items
        .into_iter()
        .map(|m| EquipmentStatusLogDto {
            id: m.id,
            equipment_id: m.equipment_id,
            status: m.status,
            running_param: m
                .running_param
                .map(|j| serde_json::from_value(j).unwrap_or(serde_json::Value::Null)),
            error_code: m.error_code,
            error_desc: m.error_desc,
            log_time: m.log_time.into(),
        })
        .collect();

    Ok(Json(PageResult { items: mapped, total }))
}

async fn control_equipment(
    State(ctx): State<ApiContext>,
    headers: HeaderMap,
    Path(id): Path<i32>,
    Json(body): Json<ControlCommandRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let Some(eq) = dao::equipment_dao::get_by_id(ctx.db.conn(), id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    else {
        return Err(StatusCode::NOT_FOUND);
    };

    let topic = format!("equipment/{}/control", eq.id);
    let payload = serde_json::json!({
        "equipment_id": eq.id,
        "command": body.command,
        "param": body.param
    });
    let Some(manager) = ctx.mqtt.clone() else {
        let resp = serde_json::json!({
            "equipment_id": eq.id,
            "command": body.command,
            "topic": topic,
            "delivered": false,
            "mode": "disabled",
            "message": "MQTT 管理器未启用，命令未下发"
        });
        write_control_operation_log(
            &ctx,
            &headers,
            id,
            &body.command,
            body.param.clone(),
            false,
            Some("MQTT 管理器未启用，命令未下发".to_string()),
            Some(topic.clone()),
        )
        .await;
        return Ok(Json(resp));
    };

    // 首次控制时自动建立该设备的 MQTT 客户端（用于状态订阅）
    if manager.get_client(eq.id as i64).is_none() {
        let status_topic = if eq.mqtt_topic.is_empty() {
            format!("equipment/{}/status", eq.id)
        } else {
            eq.mqtt_topic.clone()
        };
        manager
            .add_equipment_client(eq.id as i64, status_topic, ctx.cache.clone())
            .await
            .map_err(|_| StatusCode::BAD_GATEWAY)?;
    }

    let svc = MqttService::new(manager);
    if let Err(_) = svc.publish_control_command(eq.id as i64, topic.clone(), payload).await {
        write_control_operation_log(
            &ctx,
            &headers,
            id,
            &body.command,
            body.param.clone(),
            false,
            Some("MQTT 发布失败".to_string()),
            Some(topic.clone()),
        )
        .await;
        return Err(StatusCode::BAD_GATEWAY);
    }
    write_control_operation_log(
        &ctx,
        &headers,
        id,
        &body.command,
        body.param.clone(),
        true,
        None,
        Some(topic.clone()),
    )
    .await;
    Ok(Json(serde_json::json!({
        "equipment_id": eq.id,
        "command": body.command,
        "topic": topic,
        "delivered": true,
        "mode": "mqtt",
        "message": "控制命令已通过 MQTT 下发"
    })))
}

async fn write_control_operation_log(
    ctx: &ApiContext,
    headers: &HeaderMap,
    equipment_id: i32,
    command: &str,
    param: Option<serde_json::Value>,
    success: bool,
    error_message: Option<String>,
    topic: Option<String>,
) {
    let claims = headers
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .and_then(|token| crate::utils::jwt::decode_token(token).ok());

    let payload = serde_json::json!({
        "equipment_id": equipment_id,
        "command": command,
        "param": param,
        "topic": topic
    });

    let active = crate::db::entity::operation_logs::ActiveModel {
        user_id: Set(claims.as_ref().map(|c| c.sub)),
        username: Set(claims.as_ref().map(|c| c.username.clone())),
        module: Set(Some("equipment".to_string())),
        action: Set(Some("control_command".to_string())),
        request_path: Set(Some(format!("/api/equipment/{}/control", equipment_id))),
        method: Set(Some("POST".to_string())),
        request_time: Set(Utc::now()),
        success: Set(if success { 1 } else { 0 }),
        client_ip: Set(
            headers
                .get("x-forwarded-for")
                .or_else(|| headers.get("x-real-ip"))
                .and_then(|v| v.to_str().ok())
                .map(|s| s.to_string()),
        ),
        payload: Set(Some(payload)),
        error_message: Set(error_message),
        ..Default::default()
    };

    let _ = dao::operation_log_dao::create(ctx.db.conn(), active).await;
}

async fn list_maintenance_plans(
    State(ctx): State<ApiContext>,
    Query(q): Query<MaintenancePlanQuery>,
) -> Result<Json<PageResult<MaintenancePlanDto>>, StatusCode> {
    let filter = dao::equipment_maintenance_plan_dao::MaintenancePlanFilter {
        equipment_id: q.equipment_id,
        status: q.status,
    };
    let (items, total) = dao::equipment_maintenance_plan_dao::list(
        ctx.db.conn(),
        filter,
        q.page,
        q.page_size,
    )
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let mapped = items
        .into_iter()
        .map(|m| MaintenancePlanDto {
            id: m.id,
            plan_no: m.plan_no,
            equipment_id: m.equipment_id,
            plan_type: m.plan_type,
            cycle_type: m.cycle_type,
            cycle_value: m.cycle_value,
            next_due_time: m.next_due_time,
            status: m.status,
            remark: m.remark,
        })
        .collect();
    Ok(Json(PageResult { items: mapped, total }))
}

async fn create_maintenance_plan(
    State(ctx): State<ApiContext>,
    Json(body): Json<MaintenancePlanPayload>,
) -> Result<Json<MaintenancePlanDto>, StatusCode> {
    let active = crate::db::entity::equipment_maintenance_plans::ActiveModel {
        plan_no: Set(body.plan_no),
        equipment_id: Set(body.equipment_id),
        plan_type: Set(body.plan_type),
        cycle_type: Set(body.cycle_type),
        cycle_value: Set(body.cycle_value),
        next_due_time: Set(body.next_due_time),
        status: Set(body.status.unwrap_or(1)),
        remark: Set(body.remark),
        ..Default::default()
    };
    let m = dao::equipment_maintenance_plan_dao::create(ctx.db.conn(), active)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(MaintenancePlanDto {
        id: m.id,
        plan_no: m.plan_no,
        equipment_id: m.equipment_id,
        plan_type: m.plan_type,
        cycle_type: m.cycle_type,
        cycle_value: m.cycle_value,
        next_due_time: m.next_due_time,
        status: m.status,
        remark: m.remark,
    }))
}

async fn update_maintenance_plan(
    State(ctx): State<ApiContext>,
    Path(id): Path<i64>,
    Json(body): Json<MaintenancePlanPayload>,
) -> Result<Json<MaintenancePlanDto>, StatusCode> {
    let active = crate::db::entity::equipment_maintenance_plans::ActiveModel {
        plan_no: Set(body.plan_no),
        equipment_id: Set(body.equipment_id),
        plan_type: Set(body.plan_type),
        cycle_type: Set(body.cycle_type),
        cycle_value: Set(body.cycle_value),
        next_due_time: Set(body.next_due_time),
        status: Set(body.status.unwrap_or(1)),
        remark: Set(body.remark),
        ..Default::default()
    };
    let Some(m) = dao::equipment_maintenance_plan_dao::update(ctx.db.conn(), id, active)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    else {
        return Err(StatusCode::NOT_FOUND);
    };
    Ok(Json(MaintenancePlanDto {
        id: m.id,
        plan_no: m.plan_no,
        equipment_id: m.equipment_id,
        plan_type: m.plan_type,
        cycle_type: m.cycle_type,
        cycle_value: m.cycle_value,
        next_due_time: m.next_due_time,
        status: m.status,
        remark: m.remark,
    }))
}

async fn delete_maintenance_plan(
    State(ctx): State<ApiContext>,
    Path(id): Path<i64>,
) -> Result<StatusCode, StatusCode> {
    let rows = dao::equipment_maintenance_plan_dao::delete(ctx.db.conn(), id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    if rows == 0 {
        return Err(StatusCode::NOT_FOUND);
    }
    Ok(StatusCode::NO_CONTENT)
}

async fn list_maintenance_tasks(
    State(ctx): State<ApiContext>,
    Query(q): Query<MaintenanceTaskQuery>,
) -> Result<Json<PageResult<MaintenanceTaskDto>>, StatusCode> {
    let filter = dao::equipment_maintenance_task_dao::MaintenanceTaskFilter {
        equipment_id: q.equipment_id,
        status: q.status,
    };
    let (items, total) = dao::equipment_maintenance_task_dao::list(
        ctx.db.conn(),
        filter,
        q.page,
        q.page_size,
    )
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let mapped = items
        .into_iter()
        .map(|m| MaintenanceTaskDto {
            id: m.id,
            task_no: m.task_no,
            plan_id: m.plan_id,
            equipment_id: m.equipment_id,
            task_type: m.task_type,
            scheduled_time: m.scheduled_time,
            start_time: m.start_time,
            end_time: m.end_time,
            result: m.result,
            status: m.status,
            remark: m.remark,
        })
        .collect();
    Ok(Json(PageResult { items: mapped, total }))
}

async fn create_maintenance_task(
    State(ctx): State<ApiContext>,
    Json(body): Json<MaintenanceTaskPayload>,
) -> Result<Json<MaintenanceTaskDto>, StatusCode> {
    let active = crate::db::entity::equipment_maintenance_tasks::ActiveModel {
        task_no: Set(body.task_no),
        plan_id: Set(body.plan_id),
        equipment_id: Set(body.equipment_id),
        task_type: Set(body.task_type),
        scheduled_time: Set(body.scheduled_time),
        start_time: Set(body.start_time),
        end_time: Set(body.end_time),
        result: Set(body.result),
        status: Set(body.status.unwrap_or(1)),
        remark: Set(body.remark),
        ..Default::default()
    };
    let m = dao::equipment_maintenance_task_dao::create(ctx.db.conn(), active)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(MaintenanceTaskDto {
        id: m.id,
        task_no: m.task_no,
        plan_id: m.plan_id,
        equipment_id: m.equipment_id,
        task_type: m.task_type,
        scheduled_time: m.scheduled_time,
        start_time: m.start_time,
        end_time: m.end_time,
        result: m.result,
        status: m.status,
        remark: m.remark,
    }))
}

async fn update_maintenance_task(
    State(ctx): State<ApiContext>,
    Path(id): Path<i64>,
    Json(body): Json<MaintenanceTaskPayload>,
) -> Result<Json<MaintenanceTaskDto>, StatusCode> {
    let active = crate::db::entity::equipment_maintenance_tasks::ActiveModel {
        task_no: Set(body.task_no),
        plan_id: Set(body.plan_id),
        equipment_id: Set(body.equipment_id),
        task_type: Set(body.task_type),
        scheduled_time: Set(body.scheduled_time),
        start_time: Set(body.start_time),
        end_time: Set(body.end_time),
        result: Set(body.result),
        status: Set(body.status.unwrap_or(1)),
        remark: Set(body.remark),
        ..Default::default()
    };
    let Some(m) = dao::equipment_maintenance_task_dao::update(ctx.db.conn(), id, active)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    else {
        return Err(StatusCode::NOT_FOUND);
    };
    Ok(Json(MaintenanceTaskDto {
        id: m.id,
        task_no: m.task_no,
        plan_id: m.plan_id,
        equipment_id: m.equipment_id,
        task_type: m.task_type,
        scheduled_time: m.scheduled_time,
        start_time: m.start_time,
        end_time: m.end_time,
        result: m.result,
        status: m.status,
        remark: m.remark,
    }))
}

async fn delete_maintenance_task(
    State(ctx): State<ApiContext>,
    Path(id): Path<i64>,
) -> Result<StatusCode, StatusCode> {
    let rows = dao::equipment_maintenance_task_dao::delete(ctx.db.conn(), id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    if rows == 0 {
        return Err(StatusCode::NOT_FOUND);
    }
    Ok(StatusCode::NO_CONTENT)
}

async fn list_fault_reports(
    State(ctx): State<ApiContext>,
    Query(q): Query<FaultReportQuery>,
) -> Result<Json<PageResult<FaultReportDto>>, StatusCode> {
    let filter = dao::equipment_fault_report_dao::FaultReportFilter {
        equipment_id: q.equipment_id,
        status: q.status,
        fault_level: q.fault_level,
    };
    let (items, total) =
        dao::equipment_fault_report_dao::list(ctx.db.conn(), filter, q.page, q.page_size)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let mapped = items
        .into_iter()
        .map(|m| FaultReportDto {
            id: m.id,
            fault_no: m.fault_no,
            equipment_id: m.equipment_id,
            fault_level: m.fault_level,
            occur_time: m.occur_time,
            report_time: m.report_time,
            reporter_id: m.reporter_id,
            description: m.description,
            status: m.status,
            root_cause: m.root_cause,
            solution: m.solution,
            remark: m.remark,
        })
        .collect();
    Ok(Json(PageResult { items: mapped, total }))
}

async fn create_fault_report(
    State(ctx): State<ApiContext>,
    Json(body): Json<FaultReportPayload>,
) -> Result<Json<FaultReportDto>, StatusCode> {
    let active = crate::db::entity::equipment_fault_reports::ActiveModel {
        fault_no: Set(body.fault_no),
        equipment_id: Set(body.equipment_id),
        fault_level: Set(body.fault_level),
        occur_time: Set(body.occur_time),
        report_time: Set(body.report_time.unwrap_or(body.occur_time)),
        reporter_id: Set(body.reporter_id),
        description: Set(body.description),
        status: Set(body.status.unwrap_or(1)),
        root_cause: Set(body.root_cause),
        solution: Set(body.solution),
        remark: Set(body.remark),
        ..Default::default()
    };
    let m = dao::equipment_fault_report_dao::create(ctx.db.conn(), active)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(FaultReportDto {
        id: m.id,
        fault_no: m.fault_no,
        equipment_id: m.equipment_id,
        fault_level: m.fault_level,
        occur_time: m.occur_time,
        report_time: m.report_time,
        reporter_id: m.reporter_id,
        description: m.description,
        status: m.status,
        root_cause: m.root_cause,
        solution: m.solution,
        remark: m.remark,
    }))
}

async fn update_fault_report(
    State(ctx): State<ApiContext>,
    Path(id): Path<i64>,
    Json(body): Json<FaultReportPayload>,
) -> Result<Json<FaultReportDto>, StatusCode> {
    let active = crate::db::entity::equipment_fault_reports::ActiveModel {
        fault_no: Set(body.fault_no),
        equipment_id: Set(body.equipment_id),
        fault_level: Set(body.fault_level),
        occur_time: Set(body.occur_time),
        report_time: Set(body.report_time.unwrap_or(body.occur_time)),
        reporter_id: Set(body.reporter_id),
        description: Set(body.description),
        status: Set(body.status.unwrap_or(1)),
        root_cause: Set(body.root_cause),
        solution: Set(body.solution),
        remark: Set(body.remark),
        ..Default::default()
    };
    let Some(m) = dao::equipment_fault_report_dao::update(ctx.db.conn(), id, active)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    else {
        return Err(StatusCode::NOT_FOUND);
    };
    Ok(Json(FaultReportDto {
        id: m.id,
        fault_no: m.fault_no,
        equipment_id: m.equipment_id,
        fault_level: m.fault_level,
        occur_time: m.occur_time,
        report_time: m.report_time,
        reporter_id: m.reporter_id,
        description: m.description,
        status: m.status,
        root_cause: m.root_cause,
        solution: m.solution,
        remark: m.remark,
    }))
}

async fn delete_fault_report(
    State(ctx): State<ApiContext>,
    Path(id): Path<i64>,
) -> Result<StatusCode, StatusCode> {
    let rows = dao::equipment_fault_report_dao::delete(ctx.db.conn(), id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    if rows == 0 {
        return Err(StatusCode::NOT_FOUND);
    }
    Ok(StatusCode::NO_CONTENT)
}

async fn list_repair_orders(
    State(ctx): State<ApiContext>,
    Query(q): Query<RepairOrderQuery>,
) -> Result<Json<PageResult<RepairOrderDto>>, StatusCode> {
    let filter = dao::equipment_repair_order_dao::RepairOrderFilter {
        equipment_id: q.equipment_id,
        status: q.status,
    };
    let (items, total) =
        dao::equipment_repair_order_dao::list(ctx.db.conn(), filter, q.page, q.page_size)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let mapped = items
        .into_iter()
        .map(|m| RepairOrderDto {
            id: m.id,
            repair_no: m.repair_no,
            fault_id: m.fault_id,
            equipment_id: m.equipment_id,
            repair_type: m.repair_type,
            start_time: m.start_time,
            end_time: m.end_time,
            downtime_minutes: m.downtime_minutes,
            repair_person_id: m.repair_person_id,
            cost_labor: m.cost_labor.to_f64().unwrap_or(0.0),
            cost_spare_parts: m.cost_spare_parts.to_f64().unwrap_or(0.0),
            status: m.status,
            remark: m.remark,
        })
        .collect();
    Ok(Json(PageResult { items: mapped, total }))
}

async fn create_repair_order(
    State(ctx): State<ApiContext>,
    Json(body): Json<RepairOrderPayload>,
) -> Result<Json<RepairOrderDto>, StatusCode> {
    let active = crate::db::entity::equipment_repair_orders::ActiveModel {
        repair_no: Set(body.repair_no),
        fault_id: Set(body.fault_id),
        equipment_id: Set(body.equipment_id),
        repair_type: Set(body.repair_type),
        start_time: Set(body.start_time),
        end_time: Set(body.end_time),
        downtime_minutes: Set(body.downtime_minutes),
        repair_person_id: Set(body.repair_person_id),
        cost_labor: Set(Decimal::from_f64(body.cost_labor).unwrap_or_default()),
        cost_spare_parts: Set(Decimal::from_f64(body.cost_spare_parts).unwrap_or_default()),
        status: Set(body.status.unwrap_or(1)),
        remark: Set(body.remark),
        ..Default::default()
    };
    let m = dao::equipment_repair_order_dao::create(ctx.db.conn(), active)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(RepairOrderDto {
        id: m.id,
        repair_no: m.repair_no,
        fault_id: m.fault_id,
        equipment_id: m.equipment_id,
        repair_type: m.repair_type,
        start_time: m.start_time,
        end_time: m.end_time,
        downtime_minutes: m.downtime_minutes,
        repair_person_id: m.repair_person_id,
        cost_labor: m.cost_labor.to_f64().unwrap_or(0.0),
        cost_spare_parts: m.cost_spare_parts.to_f64().unwrap_or(0.0),
        status: m.status,
        remark: m.remark,
    }))
}

async fn update_repair_order(
    State(ctx): State<ApiContext>,
    Path(id): Path<i64>,
    Json(body): Json<RepairOrderPayload>,
) -> Result<Json<RepairOrderDto>, StatusCode> {
    let active = crate::db::entity::equipment_repair_orders::ActiveModel {
        repair_no: Set(body.repair_no),
        fault_id: Set(body.fault_id),
        equipment_id: Set(body.equipment_id),
        repair_type: Set(body.repair_type),
        start_time: Set(body.start_time),
        end_time: Set(body.end_time),
        downtime_minutes: Set(body.downtime_minutes),
        repair_person_id: Set(body.repair_person_id),
        cost_labor: Set(Decimal::from_f64(body.cost_labor).unwrap_or_default()),
        cost_spare_parts: Set(Decimal::from_f64(body.cost_spare_parts).unwrap_or_default()),
        status: Set(body.status.unwrap_or(1)),
        remark: Set(body.remark),
        ..Default::default()
    };
    let Some(m) = dao::equipment_repair_order_dao::update(ctx.db.conn(), id, active)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    else {
        return Err(StatusCode::NOT_FOUND);
    };
    Ok(Json(RepairOrderDto {
        id: m.id,
        repair_no: m.repair_no,
        fault_id: m.fault_id,
        equipment_id: m.equipment_id,
        repair_type: m.repair_type,
        start_time: m.start_time,
        end_time: m.end_time,
        downtime_minutes: m.downtime_minutes,
        repair_person_id: m.repair_person_id,
        cost_labor: m.cost_labor.to_f64().unwrap_or(0.0),
        cost_spare_parts: m.cost_spare_parts.to_f64().unwrap_or(0.0),
        status: m.status,
        remark: m.remark,
    }))
}

async fn delete_repair_order(
    State(ctx): State<ApiContext>,
    Path(id): Path<i64>,
) -> Result<StatusCode, StatusCode> {
    let rows = dao::equipment_repair_order_dao::delete(ctx.db.conn(), id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    if rows == 0 {
        return Err(StatusCode::NOT_FOUND);
    }
    Ok(StatusCode::NO_CONTENT)
}

async fn list_inspections(
    State(ctx): State<ApiContext>,
    Query(q): Query<EquipmentInspectionQuery>,
) -> Result<Json<PageResult<EquipmentInspectionDto>>, StatusCode> {
    let filter = dao::equipment_inspection_dao::EquipmentInspectionFilter {
        equipment_id: q.equipment_id,
        inspection_type: q.inspection_type,
        result: q.result,
    };
    let (items, total) =
        dao::equipment_inspection_dao::list(ctx.db.conn(), filter, q.page, q.page_size)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let mapped = items
        .into_iter()
        .map(|m| EquipmentInspectionDto {
            id: m.id,
            inspection_no: m.inspection_no,
            equipment_id: m.equipment_id,
            inspection_type: m.inspection_type,
            inspection_time: m.inspection_time,
            inspector_id: m.inspector_id,
            result: m.result,
            items: m.items,
            remark: m.remark,
        })
        .collect();
    Ok(Json(PageResult { items: mapped, total }))
}

async fn create_inspection(
    State(ctx): State<ApiContext>,
    Json(body): Json<EquipmentInspectionPayload>,
) -> Result<Json<EquipmentInspectionDto>, StatusCode> {
    let active = crate::db::entity::equipment_inspections::ActiveModel {
        inspection_no: Set(body.inspection_no),
        equipment_id: Set(body.equipment_id),
        inspection_type: Set(body.inspection_type),
        inspection_time: Set(body.inspection_time.unwrap_or_else(chrono::Utc::now)),
        inspector_id: Set(body.inspector_id),
        result: Set(body.result),
        items: Set(body.items),
        remark: Set(body.remark),
        ..Default::default()
    };
    let m = dao::equipment_inspection_dao::create(ctx.db.conn(), active)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(EquipmentInspectionDto {
        id: m.id,
        inspection_no: m.inspection_no,
        equipment_id: m.equipment_id,
        inspection_type: m.inspection_type,
        inspection_time: m.inspection_time,
        inspector_id: m.inspector_id,
        result: m.result,
        items: m.items,
        remark: m.remark,
    }))
}

async fn update_inspection(
    State(ctx): State<ApiContext>,
    Path(id): Path<i64>,
    Json(body): Json<EquipmentInspectionPayload>,
) -> Result<Json<EquipmentInspectionDto>, StatusCode> {
    let active = crate::db::entity::equipment_inspections::ActiveModel {
        inspection_no: Set(body.inspection_no),
        equipment_id: Set(body.equipment_id),
        inspection_type: Set(body.inspection_type),
        inspection_time: Set(body.inspection_time.unwrap_or_else(chrono::Utc::now)),
        inspector_id: Set(body.inspector_id),
        result: Set(body.result),
        items: Set(body.items),
        remark: Set(body.remark),
        ..Default::default()
    };
    let Some(m) = dao::equipment_inspection_dao::update(ctx.db.conn(), id, active)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    else {
        return Err(StatusCode::NOT_FOUND);
    };
    Ok(Json(EquipmentInspectionDto {
        id: m.id,
        inspection_no: m.inspection_no,
        equipment_id: m.equipment_id,
        inspection_type: m.inspection_type,
        inspection_time: m.inspection_time,
        inspector_id: m.inspector_id,
        result: m.result,
        items: m.items,
        remark: m.remark,
    }))
}

async fn delete_inspection(
    State(ctx): State<ApiContext>,
    Path(id): Path<i64>,
) -> Result<StatusCode, StatusCode> {
    let rows = dao::equipment_inspection_dao::delete(ctx.db.conn(), id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    if rows == 0 {
        return Err(StatusCode::NOT_FOUND);
    }
    Ok(StatusCode::NO_CONTENT)
}

async fn list_equipment_kpi(
    State(ctx): State<ApiContext>,
    Query(q): Query<EquipmentKpiQuery>,
) -> Result<Json<PageResult<EquipmentKpiDto>>, StatusCode> {
    let filter = dao::equipment_kpi_dao::EquipmentKpiFilter {
        equipment_id: q.equipment_id,
    };
    let (items, total) =
        dao::equipment_kpi_dao::list(ctx.db.conn(), filter, q.page, q.page_size)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let mapped = items
        .into_iter()
        .map(|m| EquipmentKpiDto {
            id: m.id,
            equipment_id: m.equipment_id,
            stat_date: m.stat_date,
            runtime_minutes: m.runtime_minutes,
            downtime_minutes: m.downtime_minutes,
            fault_count: m.fault_count,
            mtbf_minutes: m.mtbf_minutes,
            mttr_minutes: m.mttr_minutes,
            oee: m.oee.and_then(|v| v.to_f64()),
            remark: m.remark,
        })
        .collect();
    Ok(Json(PageResult { items: mapped, total }))
}


