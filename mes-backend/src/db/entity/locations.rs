#[derive(Clone, Debug, PartialEq)]
pub struct Model {
    
    pub id: i64,
    pub warehouse_id: i64,
    pub location_code: String,
    pub location_name: String,
    pub location_type: i16,
    pub status: i16,
    pub remark: Option<String>,
    pub created_time: chrono::DateTime<chrono::Utc>,
    pub updated_time: chrono::DateTime<chrono::Utc>,
    pub is_deleted: i16,
}



impl Related<super::warehouses::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Warehouses.def()
    }
}

