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
    pub plan_status: Option<i32>,
    pub plan_type: Option<i32>,
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
    pub plan_type: i32,
    pub plan_period: Option<String>,
    pub plan_start_date: chrono::NaiveDate,
    pub plan_end_date: chrono::NaiveDate,
    pub remark: Option<String>,
    pub plan_status: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct PlanDto {
    pub id: i64,
    pub plan_no: String,
    pub plan_name: String,
    pub plan_type: i32,
    pub plan_start_date: chrono::NaiveDate,
    pub plan_end_date: chrono::NaiveDate,
    pub plan_status: i32,
    pub completion_rate: f64,
}

// Production Orders
#[derive(Debug, Deserialize)]
pub struct OrderQuery {
    pub order_status: Option<i32>,
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
    pub order_status: Option<i32>,
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
    pub order_status: i32,
    pub plan_start_date: chrono::NaiveDate,
    pub plan_end_date: chrono::NaiveDate,
}

// Work Orders
#[derive(Debug, Deserialize)]
pub struct WorkOrderQuery {
    pub work_order_status: Option<i32>,
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
    pub work_order_status: Option<i32>,
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
    pub work_order_status: i32,
}

// Production Reports
#[derive(Debug, Deserialize)]
pub struct ReportQuery {
    pub report_type: Option<i32>,
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
    pub report_type: i32,
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
    pub report_type: i32,
    pub report_date: chrono::NaiveDate,
    pub report_quantity: f64,
    pub qualified_quantity: f64,
    pub unqualified_quantity: f64,
}

// ---------- Material Requirements ----------

#[derive(Debug, Deserialize)]
pub struct MaterialRequirementQuery {
    pub production_order_id: Option<i64>,
    pub material_id: Option<i64>,
    #[serde(default = "default_page")]
    pub page: u64,
    #[serde(default = "default_page_size")]
    pub page_size: u64,
}

#[derive(Debug, Deserialize)]
pub struct MaterialRequirementPayload {
    pub production_order_id: i64,
    pub material_id: i64,
    pub required_quantity: f64,
    pub reserved_quantity: Option<f64>,
    pub issued_quantity: Option<f64>,
    pub unit: String,
    pub remark: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct MaterialRequirementDto {
    pub id: i64,
    pub production_order_id: i64,
    pub material_id: i64,
    pub required_quantity: f64,
    pub reserved_quantity: f64,
    pub issued_quantity: f64,
    pub unit: String,
    pub remark: Option<String>,
}

// ---------- Picking Orders ----------

#[derive(Debug, Deserialize)]
pub struct PickingOrderQuery {
    pub production_order_id: Option<i64>,
    pub warehouse_id: Option<i64>,
    pub order_status: Option<i32>,
    #[serde(default = "default_page")]
    pub page: u64,
    #[serde(default = "default_page_size")]
    pub page_size: u64,
}

#[derive(Debug, Deserialize)]
pub struct PickingOrderDetailPayload {
    pub material_id: i64,
    pub warehouse_id: i64,
    pub location_id: Option<i64>,
    pub batch_no: Option<String>,
    pub plan_quantity: f64,
    pub unit: String,
}

#[derive(Debug, Deserialize)]
pub struct PickingOrderPayload {
    pub picking_no: String,
    pub production_order_id: i64,
    pub warehouse_id: i64,
    pub work_order_id: Option<i64>,
    pub picking_type: i32,
    pub plan_picking_date: Option<chrono::NaiveDate>,
    pub remark: Option<String>,
    pub details: Vec<PickingOrderDetailPayload>,
}

#[derive(Debug, Serialize)]
pub struct PickingOrderSummaryDto {
    pub id: i64,
    pub picking_no: String,
    pub production_order_id: i64,
    pub warehouse_id: i64,
    pub work_order_id: Option<i64>,
    pub picking_type: i32,
    pub plan_picking_date: Option<chrono::NaiveDate>,
    pub actual_picking_date: Option<chrono::NaiveDate>,
    pub total_quantity: f64,
    pub order_status: i32,
}

#[derive(Debug, Serialize)]
pub struct PickingOrderDetailDto {
    pub id: i64,
    pub material_id: i64,
    pub warehouse_id: i64,
    pub location_id: Option<i64>,
    pub batch_no: Option<String>,
    pub plan_quantity: f64,
    pub actual_quantity: f64,
    pub unit: String,
    pub line_status: i32,
}

#[derive(Debug, Serialize)]
pub struct PickingOrderWithDetailsDto {
    pub header: PickingOrderSummaryDto,
    pub details: Vec<PickingOrderDetailDto>,
}

// ---------- Return Orders ----------

#[derive(Debug, Deserialize)]
pub struct ReturnOrderQuery {
    pub production_order_id: Option<i64>,
    pub warehouse_id: Option<i64>,
    pub order_status: Option<i32>,
    #[serde(default = "default_page")]
    pub page: u64,
    #[serde(default = "default_page_size")]
    pub page_size: u64,
}

#[derive(Debug, Deserialize)]
pub struct ReturnOrderDetailPayload {
    pub material_id: i64,
    pub warehouse_id: i64,
    pub location_id: Option<i64>,
    pub batch_no: Option<String>,
    pub plan_quantity: f64,
    pub unit: String,
}

#[derive(Debug, Deserialize)]
pub struct ReturnOrderPayload {
    pub return_no: String,
    pub production_order_id: i64,
    pub warehouse_id: i64,
    pub work_order_id: Option<i64>,
    pub return_type: i32,
    pub plan_return_date: Option<chrono::NaiveDate>,
    pub remark: Option<String>,
    pub details: Vec<ReturnOrderDetailPayload>,
}

#[derive(Debug, Serialize)]
pub struct ReturnOrderSummaryDto {
    pub id: i64,
    pub return_no: String,
    pub production_order_id: i64,
    pub warehouse_id: i64,
    pub work_order_id: Option<i64>,
    pub return_type: i32,
    pub plan_return_date: Option<chrono::NaiveDate>,
    pub actual_return_date: Option<chrono::NaiveDate>,
    pub total_quantity: f64,
    pub order_status: i32,
}

#[derive(Debug, Serialize)]
pub struct ReturnOrderDetailDto {
    pub id: i64,
    pub material_id: i64,
    pub warehouse_id: i64,
    pub location_id: Option<i64>,
    pub batch_no: Option<String>,
    pub plan_quantity: f64,
    pub actual_quantity: f64,
    pub unit: String,
    pub line_status: i32,
}

#[derive(Debug, Serialize)]
pub struct ReturnOrderWithDetailsDto {
    pub header: ReturnOrderSummaryDto,
    pub details: Vec<ReturnOrderDetailDto>,
}

// ---------- Production Receipts (完工入库) ----------

#[derive(Debug, Deserialize)]
pub struct ProductionReceiptQuery {
    pub production_order_id: Option<i64>,
    pub warehouse_id: Option<i64>,
    #[serde(default = "default_page")]
    pub page: u64,
    #[serde(default = "default_page_size")]
    pub page_size: u64,
}

#[derive(Debug, Deserialize)]
pub struct ProductionReceiptPayload {
    pub receipt_no: String,
    pub production_order_id: i64,
    pub work_order_id: Option<i64>,
    pub material_id: i64,
    pub warehouse_id: i64,
    pub location_id: Option<i64>,
    pub receipt_type: i32,
    pub receipt_date: Option<chrono::NaiveDate>,
    pub quantity: f64,
    pub qualified_quantity: f64,
    pub unqualified_quantity: f64,
    pub unit: String,
    pub remark: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ProductionReceiptDto {
    pub id: i64,
    pub receipt_no: String,
    pub production_order_id: i64,
    pub work_order_id: Option<i64>,
    pub material_id: i64,
    pub warehouse_id: i64,
    pub location_id: Option<i64>,
    pub receipt_type: i32,
    pub receipt_date: Option<chrono::NaiveDate>,
    pub quantity: f64,
    pub qualified_quantity: f64,
    pub unqualified_quantity: f64,
    pub unit: String,
    pub remark: Option<String>,
}


