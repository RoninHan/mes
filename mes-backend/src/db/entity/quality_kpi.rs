use sea_orm::entity::prelude::*;
use chrono;
use rust_decimal::Decimal;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "quality_kpi")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    pub kpi_date: chrono::NaiveDate,
    pub kpi_type: i32,
    pub dept_id: Option<i64>,
    pub workshop_id: Option<i64>,
    pub total_inspections: i32,
    pub qualified_inspections: i32,
    pub unqualified_inspections: i32,
    pub inspection_quantity: Decimal,
    pub qualified_quantity: Decimal,
    pub unqualified_quantity: Decimal,
    pub batch_qualified_rate: Decimal,
    pub quantity_qualified_rate: Decimal,
    pub first_pass_yield: Decimal,
    pub iqc_qualified_rate: Decimal,
    pub ipqc_qualified_rate: Decimal,
    pub fqc_qualified_rate: Decimal,
    pub oqc_qualified_rate: Decimal,
    pub rework_quantity: Decimal,
    pub scrap_quantity: Decimal,
    pub rework_rate: Decimal,
    pub scrap_rate: Decimal,
    pub customer_complaints: i32,
    pub valid_complaints: i32,
    pub complaint_rate: Decimal,
    pub ncr_count: i32,
    pub major_ncr_count: i32,
    pub preventive_cost: Decimal,
    pub appraisal_cost: Decimal,
    pub internal_failure_cost: Decimal,
    pub external_failure_cost: Decimal,
    pub total_quality_cost: Decimal,
    pub quality_cost_rate: Decimal,
    pub dppm: Decimal,
    pub cpk: Option<Decimal>,
    pub sigma_level: Option<Decimal>,
    pub remark: Option<String>,
    pub created_by: Option<i64>,
    pub created_time: chrono::DateTime<chrono::Utc>,
    pub updated_by: Option<i64>,
    pub updated_time: chrono::DateTime<chrono::Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
