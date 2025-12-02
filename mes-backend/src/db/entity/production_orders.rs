use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "production_orders")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub order_no: String,
    pub plan_id: Option<i64>,
    pub source_type: i8,
    pub source_order_no: Option<String>,
    pub material_id: i64,
    pub bom_id: Option<i64>,
    pub routing_id: Option<i64>,
    pub plan_quantity: Decimal,
    pub actual_quantity: Decimal,
    pub qualified_quantity: Decimal,
    pub unqualified_quantity: Decimal,
    pub scrap_quantity: Decimal,
    pub unit: String,
    pub priority: i8,
    pub plan_start_date: Date,
    pub plan_end_date: Date,
    pub actual_start_date: Option<Date>,
    pub actual_end_date: Option<Date>,
    pub workshop_id: Option<i64>,
    pub production_line: Option<String>,
    pub batch_no: Option<String>,
    pub customer_id: Option<i64>,
    pub order_status: i8,
    pub is_locked: i8,
    pub is_urgent: i8,
    pub standard_hours: Decimal,
    pub actual_hours: Decimal,
    pub leader_id: Option<i64>,
    pub planner_id: Option<i64>,
    pub remark: Option<String>,
    pub created_by: Option<i64>,
    pub created_time: DateTimeWithTimeZone,
    pub updated_by: Option<i64>,
    pub updated_time: DateTimeWithTimeZone,
    pub is_deleted: i8,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No Relation")
    }
}

impl ActiveModelBehavior for ActiveModel {}


