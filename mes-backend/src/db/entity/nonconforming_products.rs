#[derive(Clone, Debug, PartialEq)]
pub struct Model {
    
    pub id: i64,
    pub ncr_no: String,
    pub report_id: Option<i64>,
    pub source_type: i16, // 1:来料检验, 2:过程检验, 3:成品检验, 4:客户退货, 5:其他
    pub source_order_no: Option<String>,
    pub material_id: i64,
    pub batch_no: Option<String>,
    pub serial_no: Option<String>,
    pub supplier_id: Option<i64>,
    pub customer_id: Option<i64>,
    pub production_order_id: Option<i64>,
    pub work_order_id: Option<i64>,
    pub process_id: Option<i64>,
    pub defect_quantity: Decimal,
    pub unit: String,
    pub defect_code: Option<String>,
    pub defect_name: Option<String>,
    pub defect_level: i16, // 1:致命, 2:严重, 3:一般, 4:轻微
    pub defect_description: Option<String>,
    pub defect_location: Option<String>,
    pub defect_images: Option<String>, // JSON format
    pub found_date: Date,
    pub found_time: chrono::DateTime<chrono::Utc>,
    pub finder_id: i64,
    pub responsible_dept_id: Option<i64>,
    pub responsible_person_id: Option<i64>,
    pub root_cause: Option<String>,
    pub disposition: Option<i16>, // 1:返工, 2:报废, 3:让步接收, 4:退货, 5:降级使用, 6:挑选
    pub disposition_quantity: Decimal,
    pub disposition_date: Option<chrono::NaiveDate>,
    pub disposition_handler_id: Option<i64>,
    pub disposition_result: Option<String>,
    pub rework_order_no: Option<String>,
    pub corrective_action: Option<String>,
    pub preventive_action: Option<String>,
    pub ncr_status: i16, // 1:待处置, 2:处置中, 3:已处置, 4:已验证, 5:已关闭
    pub is_repetitive: i16,
    pub closure_date: Option<chrono::NaiveDate>,
    pub remark: Option<String>,
    pub created_by: Option<i64>,
    pub created_time: chrono::DateTime<chrono::Utc>,
    pub updated_by: Option<i64>,
    pub updated_time: chrono::DateTime<chrono::Utc>,
    pub is_deleted: i16,
}

