#[derive(Clone, Debug, PartialEq)]
pub struct Model {
    
    pub id: i64,
    pub repair_no: String,
    pub fault_id: Option<i64>,
    pub equipment_id: i64,
    pub repair_type: i16,
    pub start_time: Option<chrono::DateTime<chrono::Utc>>,
    pub end_time: Option<chrono::DateTime<chrono::Utc>>,
    pub downtime_minutes: Option<i32>,
    pub repair_person_id: Option<i64>,
    pub cost_labor: Decimal,
    pub cost_spare_parts: Decimal,
    pub status: i16,
    pub remark: Option<String>,
    pub created_time: chrono::DateTime<chrono::Utc>,
    pub updated_time: chrono::DateTime<chrono::Utc>,
    pub is_deleted: i16,
}

