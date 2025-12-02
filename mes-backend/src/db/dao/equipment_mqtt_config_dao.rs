use crate::db::entity::{self, equipment_mqtt_config, ConnRef};
use anyhow::Result;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, Set};

pub async fn get_by_equipment_id(
    conn: ConnRef<'_>,
    equipment_id: i32,
) -> Result<Option<equipment_mqtt_config::Model>> {
    Ok(entity::EquipmentMqttConfig::find()
        .filter(equipment_mqtt_config::Column::EquipmentId.eq(equipment_id))
        .one(conn)
        .await?)
}

pub async fn upsert(
    conn: ConnRef<'_>,
    equipment_id: i32,
    mut data: equipment_mqtt_config::ActiveModel,
) -> Result<equipment_mqtt_config::Model> {
    data.equipment_id = Set(equipment_id);

    if let Some(existing) = get_by_equipment_id(conn, equipment_id).await? {
        data.id = Set(existing.id);
        Ok(entity::EquipmentMqttConfig::update(data)
            .exec_with_returning(conn)
            .await?)
    } else {
        Ok(entity::EquipmentMqttConfig::insert(data)
            .exec_with_returning(conn)
            .await?)
    }
}


