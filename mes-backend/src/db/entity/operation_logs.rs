#[derive(Clone, Debug, PartialEq)]
pub struct Model {
    
    pub id: i64,
    pub user_id: Option<i64>,
    pub username: Option<String>,
    pub module: Option<String>,
    pub action: Option<String>,
    pub request_path: Option<String>,
    pub method: Option<String>,
    pub request_time: chrono::DateTime<chrono::Utc>,
    pub success: i16,
    pub client_ip: Option<String>,
    pub payload: Option<Json>,
    pub error_message: Option<String>,
    pub created_time: chrono::DateTime<chrono::Utc>,
}

