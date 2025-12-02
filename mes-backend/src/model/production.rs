use serde::{Deserialize, Serialize};

fn default_page() -> u64 {
    0
}
fn default_page_size() -> u64 {
    20
}

#[derive(Debug, Serialize)]
pub struct PageResult<T> {
    pub items: Vec<T>,
    pub total: u64,
}

// Production Plans
#[derive(Debug, Deserialize)]
pub struct PlanQuery {
    pub plan_status: Option<i8>,
    pub plan_type: Option<i8>,
    pub keyword: Option<String>,
    #[serde(default = "default_page")]
    pub page: u64,
    #[serde(default = "default_page_size")]
    pub page_size: u64,
}

#[derive(Debug, Deserialize)]
pub struct PlanPayload {
    pub plan_no: String,
    pub plan_name: String,
    pub plan_type: i8,
    pub plan_period: Option<String>,
    pub plan_start_date: chrono::NaiveDate,
    pub plan_end_date: chrono::NaiveDate,
    pub remark: Option<String>,
    pub plan_status: Option<i8>,
}

#[derive(Debug, Serialize)]
pub struct PlanDto {
    pub id: i64,
    pub plan_no: String,
    pub plan_name: String,
    pub plan_type: i8,
    pub plan_start_date: chrono::NaiveDate,
    pub plan_end_date: chrono::NaiveDate,
    pub plan_status: i8,
    pub completion_rate: f64,
}

// Production Orders
#[derive(Debug, Deserialize)]
pub struct OrderQuery {
    pub order_status: Option<i8>,
    pub workshop_id: Option<i64>,
    pub keyword: Option<String>,
    #[serde(default = "default_page")]
    pub page: u64,
    #[serde(default = "default_page_size")]
    pub page_size: u64,
}

#[derive(Debug, Deserialize)]
pub struct OrderPayload {
    pub order_no: String,
    pub plan_id: Option<i64>,
    pub material_id: i64,
    pub plan_quantity: f64,
    pub plan_start_date: chrono::NaiveDate,
    pub plan_end_date: chrono::NaiveDate,
    pub workshop_id: Option<i64>,
    pub order_status: Option<i8>,
    pub remark: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct OrderDto {
    pub id: i64,
    pub order_no: String,
    pub plan_id: Option<i64>,
    pub material_id: i64,
    pub plan_quantity: f64,
    pub actual_quantity: f64,
    pub order_status: i8,
    pub plan_start_date: chrono::NaiveDate,
    pub plan_end_date: chrono::NaiveDate,
}

// Work Orders
#[derive(Debug, Deserialize)]
pub struct WorkOrderQuery {
    pub work_order_status: Option<i8>,
    pub equipment_id: Option<i64>,
    pub keyword: Option<String>,
    #[serde(default = "default_page")]
    pub page: u64,
    #[serde(default = "default_page_size")]
    pub page_size: u64,
}

#[derive(Debug, Deserialize)]
pub struct WorkOrderPayload {
    pub work_order_no: String,
    pub production_order_id: i64,
    pub process_id: i64,
    pub plan_quantity: f64,
    pub work_order_status: Option<i8>,
    pub remark: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct WorkOrderDto {
    pub id: i64,
    pub work_order_no: String,
    pub production_order_id: i64,
    pub process_id: i64,
    pub plan_quantity: f64,
    pub actual_quantity: f64,
    pub work_order_status: i8,
}

// Production Reports
#[derive(Debug, Deserialize)]
pub struct ReportQuery {
    pub report_type: Option<i8>,
    pub operator_id: Option<i64>,
    pub start_date: Option<chrono::NaiveDate>,
    pub end_date: Option<chrono::NaiveDate>,
    #[serde(default = "default_page")]
    pub page: u64,
    #[serde(default = "default_page_size")]
    pub page_size: u64,
}

#[derive(Debug, Deserialize)]
pub struct ReportPayload {
    pub report_no: String,
    pub work_order_id: i64,
    pub production_order_id: i64,
    pub process_id: i64,
    pub material_id: i64,
    pub report_type: i8,
    pub report_date: chrono::NaiveDate,
    pub report_quantity: f64,
    pub qualified_quantity: f64,
    pub unqualified_quantity: f64,
    pub operator_id: i64,
    pub remark: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ReportDto {
    pub id: i64,
    pub report_no: String,
    pub work_order_id: i64,
    pub report_type: i8,
    pub report_date: chrono::NaiveDate,
    pub report_quantity: f64,
    pub qualified_quantity: f64,
    pub unqualified_quantity: f64,
}


