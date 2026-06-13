#[derive(Clone, Debug, PartialEq)]
pub struct Model {
    
    pub id: i64,
    pub material_id: i64,
    pub warehouse_id: i64,
    pub min_quantity: Option<Decimal>,
    pub max_quantity: Option<Decimal>,
    pub enabled: i16,
    pub remark: Option<String>,
    pub created_time: chrono::DateTime<chrono::Utc>,
    pub updated_time: chrono::DateTime<chrono::Utc>,
    pub is_deleted: i16,
}

