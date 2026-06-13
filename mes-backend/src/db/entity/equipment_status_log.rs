#[derive(Clone, Debug, PartialEq, )]
pub struct Model {
    
    pub id: i32,
    pub equipment_id: i32,
    pub status: i16,
    pub running_param: Option<Json>,
    pub error_code: Option<String>,
    pub error_desc: Option<String>,
    pub log_time: chrono::DateTime<chrono::Utc>,
}



impl Related<super::equipment::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Equipment.def()
    }
}

