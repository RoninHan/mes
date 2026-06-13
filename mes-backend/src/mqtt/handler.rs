use crate::cache::{RedisCache, EQUIPMENT_STATUS_KEY_PREFIX};
use anyhow::Result;
use redis::AsyncCommands;
use serde::{Deserialize, Serialize};
use tracing::error;

#[derive(Debug, Deserialize, Serialize)]
pub struct EquipmentStatusPayload {
    pub equipment_id: i64,
    pub status: i16,
    pub running_param: serde_json::Value,
    pub error_code: Option<String>,
    pub error_desc: Option<String>,
}

pub async fn handle_status_message(cache: &RedisCache, payload: &str) -> Result<()> {
    let status: EquipmentStatusPayload = match serde_json::from_str(payload) {
        Ok(s) => s,
        Err(e) => {
            error!("Failed to parse equipment status payload: {e}");
            return Ok(()); // swallow to keep MQTT loop alive
        }
    };

    let key = format!("{}{}", EQUIPMENT_STATUS_KEY_PREFIX, status.equipment_id);
    let mut conn = cache.manager().as_ref().clone();
    let value = serde_json::to_string(&status)?;
    let _: () = conn.set(key, value).await?;

    Ok(())
}


