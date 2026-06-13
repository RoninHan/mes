#[derive(Clone, Debug, PartialEq)]
pub struct Model {
    
    pub id: i64,
    pub task_no: String,
    pub plan_id: Option<i64>,
    pub equipment_id: i64,
    pub task_type: i16,
    pub scheduled_time: Option<chrono::DateTime<chrono::Utc>>,
    pub start_time: Option<chrono::DateTime<chrono::Utc>>,
    pub end_time: Option<chrono::DateTime<chrono::Utc>>,
    pub result: Option<i16>,
    pub status: i16,
    pub remark: Option<String>,
    pub created_time: chrono::DateTime<chrono::Utc>,
    pub updated_time: chrono::DateTime<chrono::Utc>,
    pub is_deleted: i16,
}

