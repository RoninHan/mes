use sea_orm::entity::prelude::*;
use sea_orm::Decimal;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "quality_costs")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub cost_no: String,
    pub cost_period: String,
    pub cost_date: Date,
    pub cost_category: i16, // 1:预防成本, 2:鉴定成本, 3:内部失败成本, 4:外部失败成本
    pub cost_type: String,
    pub cost_item: String,
    pub material_id: Option<i64>,
    pub production_order_id: Option<i64>,
    pub ncr_id: Option<i64>,
    pub complaint_id: Option<i64>,
    pub cost_amount: Decimal,
    pub quantity: Option<Decimal>,
    pub unit: Option<String>,
    pub dept_id: Option<i64>,
    pub cost_description: Option<String>,
    pub handler_id: Option<i64>,
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


