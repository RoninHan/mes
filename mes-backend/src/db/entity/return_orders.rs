use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "return_orders")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub return_no: String,
    pub production_order_id: i64,
    pub warehouse_id: i64,
    pub work_order_id: Option<i64>,
    pub return_type: i16,
    pub plan_return_date: Option<Date>,
    pub actual_return_date: Option<Date>,
    pub total_quantity: Decimal,
    pub order_status: i16,
    pub remark: Option<String>,
    pub created_time: DateTimeWithTimeZone,
    pub updated_time: DateTimeWithTimeZone,
    pub is_deleted: i16,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    #[sea_orm(has_many = "super::return_order_lines::Entity")]
    ReturnOrderLines,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::ReturnOrderLines => {
                Entity::has_many(super::return_order_lines::Entity).into()
            }
        }
    }
}

impl Related<super::return_order_lines::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ReturnOrderLines.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}


