#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Model {
    
    pub id: i64,
    pub role_code: String,
    pub role_name: String,
    pub role_type: i8,
    pub data_scope: i8,
    pub sort_order: i32,
    pub status: i8,
    pub remark: Option<String>,
    pub created_by: Option<i64>,
    pub created_time: chrono::DateTime<chrono::Utc>,
    pub updated_by: Option<i64>,
    pub updated_time: chrono::DateTime<chrono::Utc>,
    pub is_deleted: i8,
}

