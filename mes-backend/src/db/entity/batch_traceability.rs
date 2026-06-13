#[derive(Clone, Debug, PartialEq)]
pub struct Model {
    
    pub id: i64,
    pub material_id: i64,
    pub batch_no: String,
    pub source_type: i16,
    pub source_ref: Option<String>,
    pub current_warehouse_id: Option<i64>,
    pub current_location_id: Option<i64>,
    pub quantity: Option<Decimal>,
    pub unit: Option<String>,
    pub status: i16,
    pub remark: Option<String>,
    pub created_time: chrono::DateTime<chrono::Utc>,
    pub updated_time: chrono::DateTime<chrono::Utc>,
    pub is_deleted: i16,
}

