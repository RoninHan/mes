use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "outbound_orders")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub outbound_no: String,
    pub outbound_type: i16,
    pub warehouse_id: i64,
    pub customer_id: Option<i64>,
    pub plan_outbound_date: Option<Date>,
    pub actual_outbound_date: Option<Date>,
    pub total_quantity: Decimal,
    pub order_status: i16,
    pub remark: Option<String>,
    pub created_time: DateTimeWithTimeZone,
    pub updated_time: DateTimeWithTimeZone,
    pub is_deleted: i16,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    #[sea_orm(has_many = "super::outbound_order_lines::Entity")]
    OutboundOrderLines,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::OutboundOrderLines => {
                Entity::has_many(super::outbound_order_lines::Entity).into()
            }
        }
    }
}

impl Related<super::outbound_order_lines::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::OutboundOrderLines.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}


