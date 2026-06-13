#[derive(Clone, Debug, PartialEq)]
pub struct Model {
    
    pub id: i64,
    pub plan_no: String,
    pub equipment_id: i64,
    pub plan_type: i16,
    pub cycle_type: i16,
    pub cycle_value: i32,
    pub next_due_time: Option<chrono::DateTime<chrono::Utc>>,
    pub status: i16,
    pub remark: Option<String>,
    pub created_time: chrono::DateTime<chrono::Utc>,
    pub updated_time: chrono::DateTime<chrono::Utc>,
    pub is_deleted: i16,
}

