#[derive(Clone, Debug, PartialEq)]
pub struct Model {
    
    pub id: i64,
    pub outbound_id: i64,
    pub material_id: i64,
    pub warehouse_id: i64,
    pub location_id: Option<i64>,
    pub batch_no: Option<String>,
    pub plan_quantity: Decimal,
    pub actual_quantity: Decimal,
    pub unit: String,
    pub line_status: i16,
    pub remark: Option<String>,
    pub created_time: chrono::DateTime<chrono::Utc>,
    pub updated_time: chrono::DateTime<chrono::Utc>,
    pub is_deleted: i16,
}



impl Related<super::outbound_orders::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::OutboundOrders.def()
    }
}

