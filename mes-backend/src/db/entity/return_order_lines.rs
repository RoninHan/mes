use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "return_order_lines")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub return_id: i64,
    pub material_id: i64,
    pub warehouse_id: i64,
    pub location_id: Option<i64>,
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
        belongs_to = "super::return_orders::Entity",
        from = "Column::ReturnId",
        to = "super::return_orders::Column::Id"
    )]
    ReturnOrders,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::ReturnOrders => Entity::belongs_to(super::return_orders::Entity)
                .from(Column::ReturnId)
                .to(super::return_orders::Column::Id)
                .into(),
        }
    }
}

impl Related<super::return_orders::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ReturnOrders.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}


