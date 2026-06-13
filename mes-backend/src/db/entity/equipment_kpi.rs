#[derive(Clone, Debug, PartialEq)]
pub struct Model {
    
    pub id: i64,
    pub equipment_id: i64,
    pub stat_date: Date,
    pub runtime_minutes: Option<i32>,
    pub downtime_minutes: Option<i32>,
    pub fault_count: Option<i32>,
    pub mtbf_minutes: Option<i32>,
    pub mttr_minutes: Option<i32>,
    pub oee: Option<Decimal>,
    pub remark: Option<String>,
    pub created_time: chrono::DateTime<chrono::Utc>,
    pub updated_time: chrono::DateTime<chrono::Utc>,
    pub is_deleted: i16,
}

