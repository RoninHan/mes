use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "picking_orders")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub picking_no: String,
    pub production_order_id: i64,
    pub warehouse_id: i64,
    pub work_order_id: Option<i64>,
    pub picking_type: i16,
    pub plan_picking_date: Option<Date>,
    pub actual_picking_date: Option<Date>,
    pub total_quantity: Decimal,
    pub order_status: i16,
    pub remark: Option<String>,
    pub created_time: DateTimeWithTimeZone,
    pub updated_time: DateTimeWithTimeZone,
    pub is_deleted: i16,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    #[sea_orm(has_many = "super::picking_order_lines::Entity")]
    PickingOrderLines,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::PickingOrderLines => {
                Entity::has_many(super::picking_order_lines::Entity).into()
            }
        }
    }
}

impl Related<super::picking_order_lines::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PickingOrderLines.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}


