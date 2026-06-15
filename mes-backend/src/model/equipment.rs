use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct EquipmentListQuery {
    pub status: Option<i32>,
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
    pub status: Option<i32>,
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
    pub status: i32,
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
    pub qos: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct StatusLogQuery {
    pub equipment_id: Option<i32>,
    pub status: Option<i32>,
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
    pub status: i32,
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

// ---------- Maintenance Plans ----------

#[derive(Debug, Deserialize)]
pub struct MaintenancePlanQuery {
    pub equipment_id: Option<i64>,
    pub status: Option<i32>,
    #[serde(default = "default_page")]
    pub page: u64,
    #[serde(default = "default_page_size")]
    pub page_size: u64,
}

#[derive(Debug, Deserialize)]
pub struct MaintenancePlanPayload {
    pub plan_no: String,
    pub equipment_id: i64,
    pub plan_type: i32,
    pub cycle_type: i32,
    pub cycle_value: i32,
    pub next_due_time: Option<chrono::DateTime<chrono::Utc>>,
    pub status: Option<i32>,
    pub remark: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct MaintenancePlanDto {
    pub id: i64,
    pub plan_no: String,
    pub equipment_id: i64,
    pub plan_type: i32,
    pub cycle_type: i32,
    pub cycle_value: i32,
    pub next_due_time: Option<chrono::DateTime<chrono::Utc>>,
    pub status: i32,
    pub remark: Option<String>,
}

// ---------- Maintenance Tasks ----------

#[derive(Debug, Deserialize)]
pub struct MaintenanceTaskQuery {
    pub equipment_id: Option<i64>,
    pub status: Option<i32>,
    #[serde(default = "default_page")]
    pub page: u64,
    #[serde(default = "default_page_size")]
    pub page_size: u64,
}

#[derive(Debug, Deserialize)]
pub struct MaintenanceTaskPayload {
    pub task_no: String,
    pub plan_id: Option<i64>,
    pub equipment_id: i64,
    pub task_type: i32,
    pub scheduled_time: Option<chrono::DateTime<chrono::Utc>>,
    pub start_time: Option<chrono::DateTime<chrono::Utc>>,
    pub end_time: Option<chrono::DateTime<chrono::Utc>>,
    pub result: Option<i32>,
    pub status: Option<i32>,
    pub remark: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct MaintenanceTaskDto {
    pub id: i64,
    pub task_no: String,
    pub plan_id: Option<i64>,
    pub equipment_id: i64,
    pub task_type: i32,
    pub scheduled_time: Option<chrono::DateTime<chrono::Utc>>,
    pub start_time: Option<chrono::DateTime<chrono::Utc>>,
    pub end_time: Option<chrono::DateTime<chrono::Utc>>,
    pub result: Option<i32>,
    pub status: i32,
    pub remark: Option<String>,
}

// ---------- Fault Reports ----------

#[derive(Debug, Deserialize)]
pub struct FaultReportQuery {
    pub equipment_id: Option<i64>,
    pub status: Option<i32>,
    pub fault_level: Option<i32>,
    #[serde(default = "default_page")]
    pub page: u64,
    #[serde(default = "default_page_size")]
    pub page_size: u64,
}

#[derive(Debug, Deserialize)]
pub struct FaultReportPayload {
    pub fault_no: String,
    pub equipment_id: i64,
    pub fault_level: i32,
    pub occur_time: chrono::DateTime<chrono::Utc>,
    pub report_time: Option<chrono::DateTime<chrono::Utc>>,
    pub reporter_id: Option<i64>,
    pub description: Option<String>,
    pub status: Option<i32>,
    pub root_cause: Option<String>,
    pub solution: Option<String>,
    pub remark: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct FaultReportDto {
    pub id: i64,
    pub fault_no: String,
    pub equipment_id: i64,
    pub fault_level: i32,
    pub occur_time: chrono::DateTime<chrono::Utc>,
    pub report_time: chrono::DateTime<chrono::Utc>,
    pub reporter_id: Option<i64>,
    pub description: Option<String>,
    pub status: i32,
    pub root_cause: Option<String>,
    pub solution: Option<String>,
    pub remark: Option<String>,
}

// ---------- Repair Orders ----------

#[derive(Debug, Deserialize)]
pub struct RepairOrderQuery {
    pub equipment_id: Option<i64>,
    pub status: Option<i32>,
    #[serde(default = "default_page")]
    pub page: u64,
    #[serde(default = "default_page_size")]
    pub page_size: u64,
}

#[derive(Debug, Deserialize)]
pub struct RepairOrderPayload {
    pub repair_no: String,
    pub fault_id: Option<i64>,
    pub equipment_id: i64,
    pub repair_type: i32,
    pub start_time: Option<chrono::DateTime<chrono::Utc>>,
    pub end_time: Option<chrono::DateTime<chrono::Utc>>,
    pub downtime_minutes: Option<i32>,
    pub repair_person_id: Option<i64>,
    pub cost_labor: f64,
    pub cost_spare_parts: f64,
    pub status: Option<i32>,
    pub remark: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct RepairOrderDto {
    pub id: i64,
    pub repair_no: String,
    pub fault_id: Option<i64>,
    pub equipment_id: i64,
    pub repair_type: i32,
    pub start_time: Option<chrono::DateTime<chrono::Utc>>,
    pub end_time: Option<chrono::DateTime<chrono::Utc>>,
    pub downtime_minutes: Option<i32>,
    pub repair_person_id: Option<i64>,
    pub cost_labor: f64,
    pub cost_spare_parts: f64,
    pub status: i32,
    pub remark: Option<String>,
}

// ---------- Equipment Inspections ----------

#[derive(Debug, Deserialize)]
pub struct EquipmentInspectionQuery {
    pub equipment_id: Option<i64>,
    pub inspection_type: Option<i32>,
    pub result: Option<i32>,
    #[serde(default = "default_page")]
    pub page: u64,
    #[serde(default = "default_page_size")]
    pub page_size: u64,
}

#[derive(Debug, Deserialize)]
pub struct EquipmentInspectionPayload {
    pub inspection_no: String,
    pub equipment_id: i64,
    pub inspection_type: i32,
    pub inspection_time: Option<chrono::DateTime<chrono::Utc>>,
    pub inspector_id: Option<i64>,
    pub result: i32,
    pub items: Option<serde_json::Value>,
    pub remark: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct EquipmentInspectionDto {
    pub id: i64,
    pub inspection_no: String,
    pub equipment_id: i64,
    pub inspection_type: i32,
    pub inspection_time: chrono::DateTime<chrono::Utc>,
    pub inspector_id: Option<i64>,
    pub result: i32,
    pub items: Option<serde_json::Value>,
    pub remark: Option<String>,
}

// ---------- Equipment KPI ----------

#[derive(Debug, Deserialize)]
pub struct EquipmentKpiQuery {
    pub equipment_id: Option<i64>,
    #[serde(default = "default_page")]
    pub page: u64,
    #[serde(default = "default_page_size")]
    pub page_size: u64,
}

#[derive(Debug, Serialize)]
pub struct EquipmentKpiDto {
    pub id: i64,
    pub equipment_id: i64,
    pub stat_date: chrono::NaiveDate,
    pub runtime_minutes: Option<i32>,
    pub downtime_minutes: Option<i32>,
    pub fault_count: Option<i32>,
    pub mtbf_minutes: Option<i32>,
    pub mttr_minutes: Option<i32>,
    pub oee: Option<f64>,
    pub remark: Option<String>,
}


