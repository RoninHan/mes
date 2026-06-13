#[derive(Clone, Debug, PartialEq)]
pub struct Model {
    
    pub id: i64,
    pub fault_no: String,
    pub equipment_id: i64,
    pub fault_level: i16,
    pub occur_time: chrono::DateTime<chrono::Utc>,
    pub report_time: chrono::DateTime<chrono::Utc>,
    pub reporter_id: Option<i64>,
    pub description: Option<String>,
    pub status: i16,
    pub root_cause: Option<String>,
    pub solution: Option<String>,
    pub remark: Option<String>,
    pub created_time: chrono::DateTime<chrono::Utc>,
    pub updated_time: chrono::DateTime<chrono::Utc>,
    pub is_deleted: i16,
}

