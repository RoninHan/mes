use sea_orm::entity::prelude::*;
use chrono;
use rust_decimal::Decimal;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "equipment_kpi")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    pub equipment_id: i64,
    pub stat_date: chrono::NaiveDate,
    pub runtime_minutes: Option<i32>,
    pub downtime_minutes: Option<i32>,
    pub fault_count: Option<i32>,
    pub mtbf_minutes: Option<i32>,
    pub mttr_minutes: Option<i32>,
    pub oee: Option<Decimal>,
    pub remark: Option<String>,
    pub created_time: chrono::DateTime<chrono::Utc>,
    pub updated_time: chrono::DateTime<chrono::Utc>,
    pub is_deleted: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
