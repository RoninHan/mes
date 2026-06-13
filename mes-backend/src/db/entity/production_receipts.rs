#[derive(Clone, Debug, PartialEq)]
pub struct Model {
    
    pub id: i64,
    pub receipt_no: String,
    pub production_order_id: i64,
    pub work_order_id: Option<i64>,
    pub material_id: i64,
    pub warehouse_id: i64,
    pub location_id: Option<i64>,
    pub receipt_type: i16,
    pub receipt_date: Option<chrono::NaiveDate>,
    pub quantity: Decimal,
    pub qualified_quantity: Decimal,
    pub unqualified_quantity: Decimal,
    pub unit: String,
    pub remark: Option<String>,
    pub created_time: chrono::DateTime<chrono::Utc>,
    pub updated_time: chrono::DateTime<chrono::Utc>,
    pub is_deleted: i16,
}

