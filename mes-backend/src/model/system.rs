use serde::{Deserialize, Serialize};

fn default_page() -> u64 {
    0
}
fn default_page_size() -> u64 {
    20
}

#[derive(Debug, Serialize)]
pub struct PageResult<T> {
    pub items: Vec<T>,
    pub total: u64,
}

// Login logs

#[derive(Debug, Deserialize)]
pub struct LoginLogQuery {
    pub user_id: Option<i64>,
    pub username: Option<String>,
    pub result: Option<i16>,
    #[serde(default = "default_page")]
    pub page: u64,
    #[serde(default = "default_page_size")]
    pub page_size: u64,
}

#[derive(Debug, Serialize)]
pub struct LoginLogDto {
    pub id: i64,
    pub user_id: Option<i64>,
    pub username: Option<String>,
    pub login_time: chrono::DateTime<chrono::Utc>,
    pub login_ip: Option<String>,
    pub user_agent: Option<String>,
    pub result: i16,
    pub fail_reason: Option<String>,
}

// Operation logs

#[derive(Debug, Deserialize)]
pub struct OperationLogQuery {
    pub user_id: Option<i64>,
    pub module: Option<String>,
    pub action: Option<String>,
    pub success: Option<i16>,
    #[serde(default = "default_page")]
    pub page: u64,
    #[serde(default = "default_page_size")]
    pub page_size: u64,
}

#[derive(Debug, Serialize)]
pub struct OperationLogDto {
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
    pub payload: Option<serde_json::Value>,
    pub error_message: Option<String>,
}



