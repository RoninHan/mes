#[derive(Clone, Debug, PartialEq)]
pub struct Model {
    
    pub id: i64,
    pub user_id: Option<i64>,
    pub username: Option<String>,
    pub login_time: chrono::DateTime<chrono::Utc>,
    pub login_ip: Option<String>,
    pub user_agent: Option<String>,
    pub result: i16,
    pub fail_reason: Option<String>,
    pub created_time: chrono::DateTime<chrono::Utc>,
}

