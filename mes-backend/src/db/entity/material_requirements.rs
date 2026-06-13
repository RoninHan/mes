#[derive(Clone, Debug, PartialEq)]
pub struct Model {
    
    pub id: i64,
    pub production_order_id: i64,
    pub material_id: i64,
    pub required_quantity: Decimal,
    pub reserved_quantity: Decimal,
    pub issued_quantity: Decimal,
    pub unit: String,
    pub remark: Option<String>,
    pub created_time: chrono::DateTime<chrono::Utc>,
    pub updated_time: chrono::DateTime<chrono::Utc>,
    pub is_deleted: i16,
}

