#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Model {
    
    pub id: i64,
    pub dept_code: String,
    pub dept_name: String,
    pub parent_id: i64,
    pub dept_level: i32,
    pub dept_path: Option<String>,
    pub manager_id: Option<i64>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub sort_order: i32,
    pub status: i8,
    pub remark: Option<String>,
    pub created_by: Option<i64>,
    pub created_time: chrono::DateTime<chrono::Utc>,
    pub updated_by: Option<i64>,
    pub updated_time: chrono::DateTime<chrono::Utc>,
    pub is_deleted: i8,
}

