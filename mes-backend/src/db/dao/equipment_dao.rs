use crate::db::entity::{self, equipment, equipment_mqtt_config, equipment_status_log, ConnRef};
use anyhow::Result;
use chrono::{DateTime, Utc};
use sea_orm::{
    ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, Set,
};

#[derive(Debug)]
pub struct EquipmentFilter {
    pub status: Option<i16>,
}

pub async fn list(
    conn: ConnRef<'_>,
    filter: EquipmentFilter,
    page: u64,
    page_size: u64,
) -> Result<(Vec<equipment::Model>, u64)> {
    let mut query = equipment::Entity::find();

    if let Some(s) = filter.status {
        query = query.filter(equipment::Column::Status.eq(s));
    }

    query = query.order_by_desc(equipment::Column::Id);

    let paginator = query.paginate(conn, page_size);
    let total = paginator.num_items().await?;
    let models = paginator.fetch_page(page).await?;
    Ok((models, total))
}

pub async fn get_by_id(conn: ConnRef<'_>, id: i32) -> Result<Option<equipment::Model>> {
    Ok(equipment::Entity::find_by_id(id).one(conn).await?)
}

pub async fn create(
    conn: ConnRef<'_>,
    model: equipment::ActiveModel,
) -> Result<equipment::Model> {
    let result = equipment::Entity::insert(model).exec(conn).await?;
    let model = equipment::Entity::find_by_id(result.last_insert_id)
        .one(conn)
        .await?
        .unwrap();
    Ok(model)
}

pub async fn update(
    conn: ConnRef<'_>,
    id: i32,
    mut data: equipment::ActiveModel,
) -> Result<Option<equipment::Model>> {
    data.id = Set(id);
    let updated = equipment::Entity::update(data).exec(conn).await?;
    Ok(Some(updated))
}

pub async fn delete(conn: ConnRef<'_>, id: i32) -> Result<u64> {
    let res = equipment::Entity::delete_by_id(id).exec(conn).await?;
    Ok(res.rows_affected)
}

pub async fn get_mqtt_config_by_equipment_id(
    conn: ConnRef<'_>,
    equipment_id: i32,
) -> Result<Option<equipment_mqtt_config::Model>> {
    Ok(equipment_mqtt_config::Entity::find()
        .filter(equipment_mqtt_config::Column::EquipmentId.eq(equipment_id))
        .one(conn)
        .await?)
}

pub async fn upsert_mqtt_config(
    conn: ConnRef<'_>,
    equipment_id: i32,
    mut data: equipment_mqtt_config::ActiveModel,
) -> Result<equipment_mqtt_config::Model> {
    data.equipment_id = Set(equipment_id);

    if let Some(existing) = get_mqtt_config_by_equipment_id(conn, equipment_id).await? {
        data.id = Set(existing.id);
        Ok(equipment_mqtt_config::Entity::update(data)
            .exec(conn)
            .await?)
    } else {
        let result = equipment_mqtt_config::Entity::insert(data).exec(conn).await?;
        let model = equipment_mqtt_config::Entity::find_by_id(result.last_insert_id)
            .one(conn)
            .await?
            .unwrap();
        Ok(model)
    }
}

pub async fn append_status_log(
    conn: ConnRef<'_>,
    equipment_id: i32,
    status: i16,
    running_param: Option<serde_json::Value>,
    error_code: Option<String>,
    error_desc: Option<String>,
) -> Result<equipment_status_log::Model> {
    let now: DateTime<Utc> = Utc::now();
    let mut active = equipment_status_log::ActiveModel {
        equipment_id: Set(equipment_id),
        status: Set(status),
        error_code: Set(error_code),
        error_desc: Set(error_desc),
        ..Default::default()
    };
    if let Some(json) = running_param {
        active.running_param = Set(Some(json.into()));
    }
    active.log_time = Set(now.into());

    let result = equipment_status_log::Entity::insert(active).exec(conn).await?;
    let model = equipment_status_log::Entity::find_by_id(result.last_insert_id)
        .one(conn)
        .await?
        .unwrap();
    Ok(model)
}


