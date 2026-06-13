#[derive(Clone, Debug, PartialEq)]
pub struct Model {
    
    pub id: i32,
    pub equipment_id: i32,
    pub broker_address: String,
    pub username: Option<String>,
    pub password: Option<String>,
    pub client_id: String,
    pub keep_alive: Option<i32>,
    pub qos: Option<i16>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}



impl Related<super::equipment::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Equipment.def()
    }
}

