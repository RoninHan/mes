#[derive(Clone, Debug, PartialEq)]
pub struct Model {
    
    pub id: i64,
    pub username: String,
    pub password: String,
    pub real_name: String,
    pub employee_no: Option<String>,
    pub dept_id: Option<i64>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub gender: Option<i8>,
    pub avatar: Option<String>,
    pub job_title: Option<String>,
    pub status: i8,
    pub is_locked: i8,
    pub lock_reason: Option<String>,
    pub pwd_update_time: Option<chrono::DateTime<chrono::Utc>>,
    pub last_login_time: Option<chrono::DateTime<chrono::Utc>>,
    pub last_login_ip: Option<String>,
    pub login_fail_count: i32,
    pub remark: Option<String>,
    pub created_by: Option<i64>,
    pub created_time: chrono::DateTime<chrono::Utc>,
    pub updated_by: Option<i64>,
    pub updated_time: chrono::DateTime<chrono::Utc>,
    pub is_deleted: i8,
}

