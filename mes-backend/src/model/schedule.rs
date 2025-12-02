use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

fn default_page() -> u64 {
    0
}
fn default_page_size() -> u64 {
    100
}

#[derive(Debug, Deserialize)]
pub struct TimelineQuery {
    pub workshop_id: Option<i64>,
    pub equipment_id: Option<i64>,
    pub from: DateTime<Utc>,
    pub to: DateTime<Utc>,
    #[serde(default = "default_page")]
    pub page: u64,
    #[serde(default = "default_page_size")]
    pub page_size: u64,
}

#[derive(Debug, Serialize)]
pub struct TimelineItemDto {
    pub id: i32,
    pub work_order_id: i64,
    pub work_order_no: String,
    pub material_id: i64,
    pub equipment_id: Option<i64>,
    pub workshop_id: Option<i64>,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub status: i16,
    pub priority: i16,
}

#[derive(Debug, Serialize)]
pub struct PageResult<T> {
    pub items: Vec<T>,
    pub total: u64,
}

#[derive(Debug, Deserialize)]
pub struct ScheduleCreateRequest {
    pub work_order_id: i64,
    pub equipment_id: Option<i64>,
    pub workshop_id: Option<i64>,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub status: Option<i16>,
    pub priority: Option<i16>,
    pub remark: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ScheduleUpdateRequest {
    pub equipment_id: Option<i64>,
    pub workshop_id: Option<i64>,
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
    pub status: Option<i16>,
    pub priority: Option<i16>,
    pub remark: Option<String>,
}


