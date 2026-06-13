#[derive(Clone, Debug, PartialEq)]
pub struct Model {
    
    pub id: i64,
    pub task_no: String,
    pub inspection_type: i16, // 1:IQC, 2:IPQC, 3:FQC, 4:OQC, 5:委外检验
    pub source_type: i16, // 1:入库单, 2:生产工单, 3:完工单, 4:出库单
    pub source_order_no: String,
    pub material_id: i64,
    pub batch_no: Option<String>,
    pub supplier_id: Option<i64>,
    pub production_order_id: Option<i64>,
    pub work_order_id: Option<i64>,
    pub process_id: Option<i64>,
    pub standard_id: Option<i64>,
    pub inspection_quantity: Decimal,
    pub sample_quantity: Decimal,
    pub qualified_quantity: Decimal,
    pub unqualified_quantity: Decimal,
    pub unit: String,
    pub inspection_level: Option<String>,
    pub aql: Option<Decimal>,
    pub sampling_plan: Option<String>,
    pub plan_start_time: Option<chrono::DateTime<chrono::Utc>>,
    pub plan_end_time: Option<chrono::DateTime<chrono::Utc>>,
    pub actual_start_time: Option<chrono::DateTime<chrono::Utc>>,
    pub actual_end_time: Option<chrono::DateTime<chrono::Utc>>,
    pub inspector_id: Option<i64>,
    pub task_status: i16, // 1:待检验, 2:检验中, 3:已完成, 4:已取消
    pub inspection_result: Option<i16>, // 1:合格, 2:不合格, 3:让步接收, 4:待定
    pub is_urgent: i16,
    pub priority: i16, // 1:紧急, 2:高, 3:普通, 4:低
    pub remark: Option<String>,
    pub created_by: Option<i64>,
    pub created_time: chrono::DateTime<chrono::Utc>,
    pub updated_by: Option<i64>,
    pub updated_time: chrono::DateTime<chrono::Utc>,
    pub is_deleted: i16,
}

