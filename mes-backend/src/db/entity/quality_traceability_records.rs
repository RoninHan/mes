use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "quality_traceability_records")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub trace_no: String,
    pub trace_type: i16, // 1:正向追溯, 2:反向追溯
    pub material_id: i64,
    pub batch_no: String,
    pub serial_no: Option<String>,
    pub production_order_no: Option<String>,
    pub sales_order_no: Option<String>,
    pub customer_id: Option<i64>,
    pub supplier_id: Option<i64>,
    pub supplier_batch_no: Option<String>,
    pub production_date: Option<Date>,
    pub inspection_report_no: Option<String>,
    pub inspection_result: Option<i16>, // 1:合格, 2:不合格
    pub workshop_id: Option<i64>,
    pub production_line: Option<String>,
    pub operator_ids: Option<String>, // JSON format
    pub equipment_ids: Option<String>, // JSON format
    pub raw_material_info: Option<String>, // JSON format
    pub process_info: Option<String>, // JSON format
    pub quality_info: Option<String>, // JSON format
    pub trace_reason: Option<String>,
    pub trace_result: Option<String>,
    pub trace_date: Date,
    pub tracer_id: i64,
    pub remark: Option<String>,
    pub created_by: Option<i64>,
    pub created_time: DateTimeWithTimeZone,
    pub updated_by: Option<i64>,
    pub updated_time: DateTimeWithTimeZone,
    pub is_deleted: i16,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No relations")
    }
}

impl ActiveModelBehavior for ActiveModel {}


