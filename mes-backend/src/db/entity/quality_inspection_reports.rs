use sea_orm::entity::prelude::*;
use sea_orm::Decimal;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "quality_inspection_reports")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub report_no: String,
    pub task_id: i64,
    pub inspection_type: i16,
    pub material_id: i64,
    pub batch_no: Option<String>,
    pub supplier_id: Option<i64>,
    pub production_order_id: Option<i64>,
    pub inspection_date: Date,
    pub inspection_time: DateTimeWithTimeZone,
    pub inspector_id: i64,
    pub reviewer_id: Option<i64>,
    pub review_time: Option<DateTimeWithTimeZone>,
    pub inspection_quantity: Decimal,
    pub sample_quantity: Decimal,
    pub qualified_quantity: Decimal,
    pub unqualified_quantity: Decimal,
    pub unit: String,
    pub qualified_rate: Decimal,
    pub inspection_result: i16, // 1:合格, 2:不合格, 3:让步接收, 4:待定
    pub disposition: Option<i16>, // 1:接收, 2:退货, 3:返工, 4:报废, 5:降级使用
    pub major_defects: i32,
    pub minor_defects: i32,
    pub critical_defects: i32,
    pub inspection_environment: Option<String>,
    pub inspection_equipment: Option<String>,
    pub report_status: i16, // 1:待审核, 2:已审核, 3:已归档
    pub conclusion: Option<String>,
    pub improvement_suggestions: Option<String>,
    pub attachment_url: Option<String>,
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


