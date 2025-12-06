use sea_orm::entity::prelude::*;
use sea_orm::Decimal;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "rework_orders")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub rework_no: String,
    pub ncr_id: i64,
    pub material_id: i64,
    pub batch_no: Option<String>,
    pub rework_quantity: Decimal,
    pub completed_quantity: Decimal,
    pub qualified_quantity: Decimal,
    pub scrap_quantity: Decimal,
    pub unit: String,
    pub rework_type: i16, // 1:工序返工, 2:全检挑选, 3:返修, 4:其他
    pub rework_reason: String,
    pub rework_plan: Option<String>,
    pub rework_process: Option<String>,
    pub workshop_id: Option<i64>,
    pub plan_start_date: Option<Date>,
    pub plan_end_date: Option<Date>,
    pub actual_start_date: Option<Date>,
    pub actual_end_date: Option<Date>,
    pub handler_id: Option<i64>,
    pub rework_cost: Decimal,
    pub rework_status: i16, // 1:待返工, 2:返工中, 3:已完成, 4:已取消
    pub inspection_result: Option<i16>, // 1:合格, 2:不合格
    pub inspector_id: Option<i64>,
    pub inspection_time: Option<DateTimeWithTimeZone>,
    pub remark: Option<String>,
    pub created_by: Option<i64>,
    pub created_time: DateTimeWithTimeZone,
    pub updated_by: Option<i64>,
    pub updated_time: DateTimeWithTimeZone,
    pub is_deleted: i16,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No relations")
    }
}

impl ActiveModelBehavior for ActiveModel {}


