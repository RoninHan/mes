#[derive(Clone, Debug, PartialEq)]
pub struct Model {
    
    pub id: i64,
    pub adjustment_no: String,
    pub warehouse_id: i64,
    pub location_id: Option<i64>,
    pub material_id: i64,
    pub batch_no: Option<String>,
    pub quantity_delta: Decimal,
    pub unit: String,
    pub reason: Option<String>,
    pub adjustment_type: i16,
    pub business_time: chrono::DateTime<chrono::Utc>,
    pub status: i16,
    pub remark: Option<String>,
    pub created_time: chrono::DateTime<chrono::Utc>,
    pub updated_time: chrono::DateTime<chrono::Utc>,
    pub is_deleted: i16,
}

