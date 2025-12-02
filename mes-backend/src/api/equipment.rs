use crate::api::ApiContext;
use crate::db::dao;
use crate::model::equipment::*;
use crate::service::mqtt_service::MqttService;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use sea_orm::ActiveValue::Set;

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
            production_date: m.production_date.map(|d| d.naive_utc().date()),
            install_date: m.install_date.map(|d| d.naive_utc().date()),
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
        production_date: m.production_date.map(|d| d.naive_utc().date()),
        install_date: m.install_date.map(|d| d.naive_utc().date()),
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
        production_date: m.production_date.map(|d| d.naive_utc().date()),
        install_date: m.install_date.map(|d| d.naive_utc().date()),
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
        production_date: m.production_date.map(|d| d.naive_utc().date()),
        install_date: m.install_date.map(|d| d.naive_utc().date()),
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

    // TODO: 可回退到数据库最新一条状态
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
    Path(id): Path<i32>,
    Json(body): Json<ControlCommandRequest>,
) -> Result<StatusCode, StatusCode> {
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

    let svc = MqttService::new(ctx.mqtt.clone());
    svc.publish_control_command(eq.id as i64, topic, payload)
        .await
        .map_err(|_| StatusCode::BAD_GATEWAY)?;

    Ok(StatusCode::NO_CONTENT)
}


