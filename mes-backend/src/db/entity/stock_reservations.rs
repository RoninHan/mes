#[derive(Clone, Debug, PartialEq)]
pub struct Model {
    
    pub id: i64,
    pub reservation_no: String,
    pub material_id: i64,
    pub warehouse_id: i64,
    pub location_id: Option<i64>,
    pub batch_no: Option<String>,
    pub reserved_quantity: Decimal,
    pub unit: String,
    pub source_type: i16,
    pub source_id: Option<i64>,
    pub status: i16,
    pub expire_time: Option<chrono::DateTime<chrono::Utc>>,
    pub remark: Option<String>,
    pub created_time: chrono::DateTime<chrono::Utc>,
    pub updated_time: chrono::DateTime<chrono::Utc>,
    pub is_deleted: i16,
}

