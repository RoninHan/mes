#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Model {
    
    pub id: i64,
    pub role_id: i64,
    pub permission_id: i64,
    pub created_by: Option<i64>,
    pub created_time: chrono::DateTime<chrono::Utc>,
}

