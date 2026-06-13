#[derive(Clone, Debug, PartialEq)]
pub struct Model {
    
    pub id: i64,
    pub outbound_no: String,
    pub outbound_type: i16,
    pub warehouse_id: i64,
    pub customer_id: Option<i64>,
    pub plan_outbound_date: Option<chrono::NaiveDate>,
    pub actual_outbound_date: Option<chrono::NaiveDate>,
    pub total_quantity: Decimal,
    pub order_status: i16,
    pub remark: Option<String>,
    pub created_time: chrono::DateTime<chrono::Utc>,
    pub updated_time: chrono::DateTime<chrono::Utc>,
    pub is_deleted: i16,
}



impl Related<super::outbound_order_lines::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::OutboundOrderLines.def()
    }
}

