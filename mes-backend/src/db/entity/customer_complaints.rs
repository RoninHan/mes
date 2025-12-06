use sea_orm::entity::prelude::*;
use sea_orm::Decimal;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "customer_complaints")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub complaint_no: String,
    pub customer_id: i64,
    pub material_id: i64,
    pub batch_no: Option<String>,
    pub sales_order_no: Option<String>,
    pub production_order_no: Option<String>,
    pub complaint_type: i16, // 1:质量问题, 2:交期问题, 3:服务问题, 4:包装问题, 5:其他
    pub complaint_level: i16, // 1:严重, 2:重要, 3:一般
    pub complaint_date: Date,
    pub complaint_time: DateTimeWithTimeZone,
    pub complaint_quantity: Option<Decimal>,
    pub unit: Option<String>,
    pub complaint_description: String,
    pub defect_description: Option<String>,
    pub defect_images: Option<String>, // JSON format
    pub customer_requirement: Option<String>,
    pub receiver_id: i64,
    pub handler_id: Option<i64>,
    pub response_deadline: Option<Date>,
    pub response_time: Option<DateTimeWithTimeZone>,
    pub response_content: Option<String>,
    pub root_cause_analysis: Option<String>,
    pub corrective_action: Option<String>,
    pub preventive_action: Option<String>,
    pub compensation_amount: Decimal,
    pub processing_cost: Decimal,
    pub complaint_status: i16, // 1:待处理, 2:处理中, 3:待验证, 4:已关闭
    pub is_valid: i16,
    pub customer_satisfaction: Option<i16>, // 1-5
    pub closure_date: Option<Date>,
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


