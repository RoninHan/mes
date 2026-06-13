#[derive(Clone, Debug, PartialEq)]
pub struct Model {
    
    pub id: i64,
    pub transfer_no: String,
    pub from_warehouse_id: i64,
    pub to_warehouse_id: i64,
    pub plan_transfer_date: Option<chrono::NaiveDate>,
    pub actual_transfer_date: Option<chrono::NaiveDate>,
    pub total_quantity: Decimal,
    pub order_status: i16,
    pub remark: Option<String>,
    pub created_time: chrono::DateTime<chrono::Utc>,
    pub updated_time: chrono::DateTime<chrono::Utc>,
    pub is_deleted: i16,
}



impl Related<super::transfer_order_lines::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TransferOrderLines.def()
    }
}

