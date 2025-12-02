use super::client::{MqttClient, MqttClientHandle};
use crate::cache::RedisCache;
use crate::mqtt::handler::handle_status_message;
use anyhow::Result;
use rumqttc::Event;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::task::JoinHandle;
use tracing::{error, info};

#[derive(Clone)]
pub struct MqttClientManager {
    inner: Arc<Mutex<HashMap<i64, MqttClientHandle>>>,
    default_broker: String,
}

impl MqttClientManager {
    pub fn new(default_broker: String) -> Self {
        Self {
            inner: Arc::new(Mutex::new(HashMap::new())),
            default_broker,
        }
    }

    pub async fn add_equipment_client(
        &self,
        equipment_id: i64,
        topic: String,
        cache: RedisCache,
    ) -> Result<()> {
        let client_id = format!("mes-equipment-{}", equipment_id);
        let (client, mut event_loop) = MqttClient::new(&self.default_broker, &client_id);
        client.subscribe(&topic, rumqttc::QoS::AtLeastOnce).await?;

        let handle_client = client.clone();
        let task: JoinHandle<()> = tokio::spawn(async move {
            loop {
                match event_loop.poll().await {
                    Ok(Event::Incoming(rumqttc::Packet::Publish(p))) => {
                        if let Ok(payload) = String::from_utf8(p.payload.to_vec()) {
                            if let Err(e) = handle_status_message(&cache, &payload).await {
                                error!("MQTT status handler error: {e}");
                            }
                        }
                    }
                    Ok(_) => {}
                    Err(e) => {
                        error!("MQTT event loop error: {e}");
                        break;
                    }
                }
            }
        });

        let mut map = self.inner.lock().unwrap();
        map.insert(
            equipment_id,
            MqttClientHandle {
                client: handle_client,
                event_loop_task: task,
            },
        );

        info!("MQTT client created for equipment {}", equipment_id);
        Ok(())
    }

    pub fn get_client(&self, equipment_id: i64) -> Option<MqttClient> {
        self.inner
            .lock()
            .unwrap()
            .get(&equipment_id)
            .map(|h| h.client.clone())
    }
}


