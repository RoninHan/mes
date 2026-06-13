#[derive(Clone, Debug, PartialEq)]
pub struct Model {
    
    pub id: i64,
    pub return_no: String,
    pub production_order_id: i64,
    pub warehouse_id: i64,
    pub work_order_id: Option<i64>,
    pub return_type: i16,
    pub plan_return_date: Option<chrono::NaiveDate>,
    pub actual_return_date: Option<chrono::NaiveDate>,
    pub total_quantity: Decimal,
    pub order_status: i16,
    pub remark: Option<String>,
    pub created_time: chrono::DateTime<chrono::Utc>,
    pub updated_time: chrono::DateTime<chrono::Utc>,
    pub is_deleted: i16,
}



impl Related<super::return_order_lines::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ReturnOrderLines.def()
    }
}

