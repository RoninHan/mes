use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "work_orders")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub work_order_no: String,
    pub production_order_id: i64,
    pub process_id: i64,
    pub sequence_no: i32,
    pub material_id: i64,
    pub plan_quantity: Decimal,
    pub actual_quantity: Decimal,
    pub qualified_quantity: Decimal,
    pub unqualified_quantity: Decimal,
    pub scrap_quantity: Decimal,
    pub unit: String,
    pub workshop_id: Option<i64>,
    pub equipment_id: Option<i64>,
    pub plan_start_time: Option<DateTimeWithTimeZone>,
    pub plan_end_time: Option<DateTimeWithTimeZone>,
    pub actual_start_time: Option<DateTimeWithTimeZone>,
    pub actual_end_time: Option<DateTimeWithTimeZone>,
    pub standard_hours: Decimal,
    pub actual_hours: Decimal,
    pub standard_labor_count: i32,
    pub actual_labor_count: i32,
    pub work_order_status: i8,
    pub is_key_process: i8,
    pub is_quality_check: i8,
    pub quality_check_status: Option<i8>,
    pub is_outsourced: i8,
    pub supplier_id: Option<i64>,
    pub operator_ids: Option<String>,
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


