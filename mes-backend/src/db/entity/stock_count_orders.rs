#[derive(Clone, Debug, PartialEq)]
pub struct Model {
    
    pub id: i64,
    pub count_no: String,
    pub warehouse_id: i64,
    pub count_type: i16,
    pub plan_count_date: Option<chrono::NaiveDate>,
    pub actual_count_date: Option<chrono::NaiveDate>,
    pub order_status: i16,
    pub remark: Option<String>,
    pub created_time: chrono::DateTime<chrono::Utc>,
    pub updated_time: chrono::DateTime<chrono::Utc>,
    pub is_deleted: i16,
}



impl Related<super::stock_count_lines::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::StockCountLines.def()
    }
}

