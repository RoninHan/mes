use crate::mqtt::manager::MqttClientManager;
use anyhow::Result;

pub struct MqttService {
    pub manager: MqttClientManager,
}

impl MqttService {
    pub fn new(manager: MqttClientManager) -> Self {
        Self { manager }
    }

    pub async fn publish_control_command(
        &self,
        equipment_id: i64,
        topic: String,
        payload: serde_json::Value,
    ) -> Result<()> {
        if let Some(client) = self.manager.get_client(equipment_id) {
            let bytes = serde_json::to_vec(&payload)?;
            client
                .publish(&topic, bytes, rumqttc::QoS::AtLeastOnce)
                .await?;
        }
        Ok(())
    }
}


