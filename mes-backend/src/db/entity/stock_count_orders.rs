use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "stock_count_orders")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub count_no: String,
    pub warehouse_id: i64,
    pub count_type: i16,
    pub plan_count_date: Option<Date>,
    pub actual_count_date: Option<Date>,
    pub order_status: i16,
    pub remark: Option<String>,
    pub created_time: DateTimeWithTimeZone,
    pub updated_time: DateTimeWithTimeZone,
    pub is_deleted: i16,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    #[sea_orm(has_many = "super::stock_count_lines::Entity")]
    StockCountLines,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::StockCountLines => {
                Entity::has_many(super::stock_count_lines::Entity).into()
            }
        }
    }
}

impl Related<super::stock_count_lines::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::StockCountLines.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}


