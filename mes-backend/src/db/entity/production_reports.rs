use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "production_reports")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub report_no: String,
    pub work_order_id: i64,
    pub production_order_id: i64,
    pub process_id: i64,
    pub material_id: i64,
    pub equipment_id: Option<i64>,
    pub workshop_id: Option<i64>,
    pub report_type: i8,
    pub report_date: Date,
    pub report_time: DateTimeWithTimeZone,
    pub shift: Option<String>,
    pub operator_id: i64,
    pub team_members: Option<String>,
    pub report_quantity: Decimal,
    pub qualified_quantity: Decimal,
    pub unqualified_quantity: Decimal,
    pub scrap_quantity: Decimal,
    pub unit: String,
    pub work_hours: Decimal,
    pub standard_hours: Decimal,
    pub efficiency: Decimal,
    pub start_time: Option<DateTimeWithTimeZone>,
    pub end_time: Option<DateTimeWithTimeZone>,
    pub downtime_minutes: i32,
    pub downtime_reason: Option<String>,
    pub quality_issue: Option<String>,
    pub is_approved: i8,
    pub approver_id: Option<i64>,
    pub approval_time: Option<DateTimeWithTimeZone>,
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


