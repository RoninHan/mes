#[derive(Clone, Debug, PartialEq)]
pub struct Model {
    
    pub id: i64,
    pub warehouse_code: String,
    pub warehouse_name: String,
    pub warehouse_type: i16,
    pub location: Option<String>,
    pub status: i16,
    pub remark: Option<String>,
    pub created_time: chrono::DateTime<chrono::Utc>,
    pub updated_time: chrono::DateTime<chrono::Utc>,
    pub is_deleted: i16,
}

