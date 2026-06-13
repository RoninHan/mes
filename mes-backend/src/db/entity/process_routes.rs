#[derive(Clone, Debug, PartialEq)]
pub struct Model {
    
    pub id: i64,
    pub material_id: i64,
    pub route_code: String,
    pub route_name: String,
    pub version: String,
    pub is_default: i16,
    pub status: i16,
    pub operations: Json,
    pub remark: Option<String>,
    pub created_time: chrono::DateTime<chrono::Utc>,
    pub updated_time: chrono::DateTime<chrono::Utc>,
    pub is_deleted: i16,
}

