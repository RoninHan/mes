#[derive(Clone, Debug, PartialEq)]
pub struct Model {
    
    pub id: i64,
    pub material_id: i64,
    pub bom_code: String,
    pub version: String,
    pub bom_type: i16,
    pub is_default: i16,
    pub status: i16,
    pub items: Json,
    pub remark: Option<String>,
    pub created_time: chrono::DateTime<chrono::Utc>,
    pub updated_time: chrono::DateTime<chrono::Utc>,
    pub is_deleted: i16,
}

