#[derive(Clone, Debug, PartialEq)]
pub struct Model {
    
    pub id: i64,
    pub trans_type: i16,
    pub ref_type: i16,
    pub ref_id: Option<i64>,
    pub material_id: i64,
    pub warehouse_id: i64,
    pub location_id: Option<i64>,
    pub batch_no: Option<String>,
    pub quantity_delta: Decimal,
    pub unit: String,
    pub before_quantity: Option<Decimal>,
    pub after_quantity: Option<Decimal>,
    pub business_time: chrono::DateTime<chrono::Utc>,
    pub remark: Option<String>,
    pub created_time: chrono::DateTime<chrono::Utc>,
    pub updated_time: chrono::DateTime<chrono::Utc>,
    pub is_deleted: i16,
}

