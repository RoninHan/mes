#[derive(Clone, Debug, PartialEq)]
pub struct Model {
    
    pub id: i64,
    pub inspection_no: String,
    pub equipment_id: i64,
    pub inspection_type: i16,
    pub inspection_time: chrono::DateTime<chrono::Utc>,
    pub inspector_id: Option<i64>,
    pub result: i16,
    pub items: Option<Json>,
    pub remark: Option<String>,
    pub created_time: chrono::DateTime<chrono::Utc>,
    pub updated_time: chrono::DateTime<chrono::Utc>,
    pub is_deleted: i16,
}

