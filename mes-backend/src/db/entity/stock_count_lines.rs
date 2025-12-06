use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "stock_count_lines")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub count_id: i64,
    pub material_id: i64,
    pub warehouse_id: i64,
    pub location_id: Option<i64>,
    pub batch_no: Option<String>,
    pub book_quantity: Decimal,
    pub counted_quantity: Decimal,
    pub diff_quantity: Decimal,
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
        belongs_to = "super::stock_count_orders::Entity",
        from = "Column::CountId",
        to = "super::stock_count_orders::Column::Id"
    )]
    StockCountOrders,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::StockCountOrders => Entity::belongs_to(super::stock_count_orders::Entity)
                .from(Column::CountId)
                .to(super::stock_count_orders::Column::Id)
                .into(),
        }
    }
}

impl Related<super::stock_count_orders::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::StockCountOrders.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}


