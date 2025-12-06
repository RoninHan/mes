use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "transfer_orders")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub transfer_no: String,
    pub from_warehouse_id: i64,
    pub to_warehouse_id: i64,
    pub plan_transfer_date: Option<Date>,
    pub actual_transfer_date: Option<Date>,
    pub total_quantity: Decimal,
    pub order_status: i16,
    pub remark: Option<String>,
    pub created_time: DateTimeWithTimeZone,
    pub updated_time: DateTimeWithTimeZone,
    pub is_deleted: i16,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    #[sea_orm(has_many = "super::transfer_order_lines::Entity")]
    TransferOrderLines,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::TransferOrderLines => {
                Entity::has_many(super::transfer_order_lines::Entity).into()
            }
        }
    }
}

impl Related<super::transfer_order_lines::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TransferOrderLines.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}


