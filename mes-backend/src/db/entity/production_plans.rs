#[derive(Clone, Debug, PartialEq)]
pub struct Model {
    
    pub id: i64,
    pub plan_no: String,
    pub plan_name: String,
    pub plan_type: i8,
    pub plan_period: Option<String>,
    pub plan_start_date: Date,
    pub plan_end_date: Date,
    pub total_orders: i32,
    pub completed_orders: i32,
    pub total_quantity: Decimal,
    pub completed_quantity: Decimal,
    pub plan_status: i8,
    pub completion_rate: Decimal,
    pub planner_id: Option<i64>,
    pub dept_id: Option<i64>,
    pub remark: Option<String>,
    pub created_by: Option<i64>,
    pub created_time: chrono::DateTime<chrono::Utc>,
    pub updated_by: Option<i64>,
    pub updated_time: chrono::DateTime<chrono::Utc>,
    pub is_deleted: i8,
}

