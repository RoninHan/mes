#[derive(Clone, Debug, PartialEq)]
pub struct Model {
    
    pub id: i32,
    pub work_order_id: i64,
    pub equipment_id: Option<i64>,
    pub workshop_id: Option<i64>,
    pub start_time: chrono::DateTime<chrono::Utc>,
    pub end_time: chrono::DateTime<chrono::Utc>,
    pub status: i16,
    pub priority: i16,
    pub remark: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

