use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct EquipmentListQuery {
    pub status: Option<i16>,
    #[serde(default = "default_page")]
    pub page: u64,
    #[serde(default = "default_page_size")]
    pub page_size: u64,
}

fn default_page() -> u64 {
    0
}
fn default_page_size() -> u64 {
    20
}

#[derive(Debug, Serialize)]
pub struct PageResult<T> {
    pub items: Vec<T>,
    pub total: u64,
}

#[derive(Debug, Deserialize)]
pub struct EquipmentCreateOrUpdate {
    pub equipment_code: String,
    pub equipment_name: String,
    pub equipment_type: String,
    pub model: Option<String>,
    pub factory: Option<String>,
    pub production_date: Option<chrono::NaiveDate>,
    pub install_date: Option<chrono::NaiveDate>,
    pub status: Option<i16>,
    pub ip_address: Option<String>,
    pub mqtt_topic: String,
    pub location: Option<String>,
    pub responsible_person: Option<String>,
    pub remark: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct EquipmentDetail {
    pub id: i32,
    pub equipment_code: String,
    pub equipment_name: String,
    pub equipment_type: String,
    pub model: Option<String>,
    pub factory: Option<String>,
    pub production_date: Option<chrono::NaiveDate>,
    pub install_date: Option<chrono::NaiveDate>,
    pub status: i16,
    pub ip_address: Option<String>,
    pub mqtt_topic: String,
    pub location: Option<String>,
    pub responsible_person: Option<String>,
    pub remark: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EquipmentMqttConfigDto {
    pub broker_address: String,
    pub username: Option<String>,
    pub password: Option<String>,
    pub client_id: String,
    pub keep_alive: Option<i32>,
    pub qos: Option<i16>,
}

#[derive(Debug, Deserialize)]
pub struct StatusLogQuery {
    pub equipment_id: Option<i32>,
    pub status: Option<i16>,
    pub start_time: Option<chrono::DateTime<chrono::Utc>>,
    pub end_time: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(default = "default_page")]
    pub page: u64,
    #[serde(default = "default_page_size")]
    pub page_size: u64,
}

#[derive(Debug, Serialize)]
pub struct EquipmentStatusLogDto {
    pub id: i32,
    pub equipment_id: i32,
    pub status: i16,
    pub running_param: Option<serde_json::Value>,
    pub error_code: Option<String>,
    pub error_desc: Option<String>,
    pub log_time: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Deserialize)]
pub struct ControlCommandRequest {
    pub command: String,
    pub param: Option<serde_json::Value>,
}


