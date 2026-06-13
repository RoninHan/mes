#[derive(Clone, Debug, PartialEq)]
pub struct Model {
    
    pub id: i32,
    pub equipment_code: String,
    pub equipment_name: String,
    pub equipment_type: String,
    pub model: Option<String>,
    pub factory: Option<String>,
    pub production_date: Option<chrono::NaiveDate>,
    pub install_date: Option<chrono::NaiveDate>,
    pub status: i16,
    pub ip_address: Option<String>,
    pub mqtt_topic: String,
    pub location: Option<String>,
    pub responsible_person: Option<String>,
    pub remark: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}




