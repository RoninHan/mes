use anyhow::Result;
use rumqttc::{AsyncClient, EventLoop, MqttOptions, QoS};
use std::time::Duration;
use tokio::task::JoinHandle;

#[derive(Clone)]
pub struct MqttClient {
    pub client: AsyncClient,
}

pub struct MqttClientHandle {
    pub client: MqttClient,
    pub event_loop_task: JoinHandle<()>,
}

impl MqttClient {
    pub fn new(broker: &str, client_id: &str) -> (Self, EventLoop) {
        let mut opts = MqttOptions::new(client_id, broker, 1883);
        opts.set_keep_alive(Duration::from_secs(60));

        let (client, event_loop) = AsyncClient::new(opts, 10);
        (Self { client }, event_loop)
    }

    pub async fn subscribe(&self, topic: &str, qos: QoS) -> Result<()> {
        self.client.subscribe(topic, qos).await?;
        Ok(())
    }

    pub async fn publish(&self, topic: &str, payload: Vec<u8>, qos: QoS) -> Result<()> {
        self.client.publish(topic, qos, false, payload).await?;
        Ok(())
    }
}


