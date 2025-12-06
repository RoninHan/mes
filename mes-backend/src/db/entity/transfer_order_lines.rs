use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "transfer_order_lines")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub transfer_id: i64,
    pub material_id: i64,
    pub from_warehouse_id: i64,
    pub from_location_id: Option<i64>,
    pub to_warehouse_id: i64,
    pub to_location_id: Option<i64>,
    pub batch_no: Option<String>,
    pub plan_quantity: Decimal,
    pub actual_quantity: Decimal,
    pub unit: String,
    pub line_status: i16,
    pub remark: Option<String>,
    pub created_time: DateTimeWithTimeZone,
    pub updated_time: DateTimeWithTimeZone,
    pub is_deleted: i16,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::transfer_orders::Entity",
        from = "Column::TransferId",
        to = "super::transfer_orders::Column::Id"
    )]
    TransferOrders,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::TransferOrders => Entity::belongs_to(super::transfer_orders::Entity)
                .from(Column::TransferId)
                .to(super::transfer_orders::Column::Id)
                .into(),
        }
    }
}

impl Related<super::transfer_orders::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TransferOrders.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}


