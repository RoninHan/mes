use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PageResult<T> {
    pub items: Vec<T>,
    pub total: u64,
}

fn default_page() -> u64 {
    0
}
fn default_page_size() -> u64 {
    20
}

// Quality Inspection Tasks
#[derive(Debug, Deserialize)]
pub struct InspectionTaskQuery {
    pub inspection_type: Option<i32>,
    pub source_type: Option<i32>,
    pub source_order_no: Option<String>,
    pub material_id: Option<i64>,
    pub batch_no: Option<String>,
    pub task_status: Option<i32>,
    pub inspector_id: Option<i64>,
    #[serde(default = "default_page")]
    pub page: u64,
    #[serde(default = "default_page_size")]
    pub page_size: u64,
}

#[derive(Debug, Deserialize)]
pub struct InspectionTaskPayload {
    pub task_no: String,
    pub inspection_type: i32,
    pub source_type: i32,
    pub source_order_no: String,
    pub material_id: i64,
    pub batch_no: Option<String>,
    pub supplier_id: Option<i64>,
    pub production_order_id: Option<i64>,
    pub work_order_id: Option<i64>,
    pub inspection_quantity: f64,
    pub unit: String,
    pub plan_start_time: Option<chrono::DateTime<chrono::Utc>>,
    pub plan_end_time: Option<chrono::DateTime<chrono::Utc>>,
    pub inspector_id: Option<i64>,
    pub priority: Option<i32>,
    pub remark: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct InspectionTaskDto {
    pub id: i64,
    pub task_no: String,
    pub inspection_type: i32,
    pub source_type: i32,
    pub source_order_no: String,
    pub material_id: i64,
    pub batch_no: Option<String>,
    pub task_status: i32,
    pub inspection_result: Option<i32>,
    pub inspection_quantity: f64,
    pub qualified_quantity: f64,
    pub unqualified_quantity: f64,
    pub unit: String,
    pub plan_start_time: Option<chrono::DateTime<chrono::Utc>>,
    pub plan_end_time: Option<chrono::DateTime<chrono::Utc>>,
    pub actual_start_time: Option<chrono::DateTime<chrono::Utc>>,
    pub actual_end_time: Option<chrono::DateTime<chrono::Utc>>,
    pub inspector_id: Option<i64>,
    pub priority: i32,
}

// Quality Inspection Reports
#[derive(Debug, Deserialize)]
pub struct InspectionReportQuery {
    pub task_id: Option<i64>,
    pub inspection_type: Option<i32>,
    pub material_id: Option<i64>,
    pub batch_no: Option<String>,
    pub report_status: Option<i32>,
    pub inspection_result: Option<i32>,
    #[serde(default = "default_page")]
    pub page: u64,
    #[serde(default = "default_page_size")]
    pub page_size: u64,
}

#[derive(Debug, Deserialize)]
pub struct InspectionReportPayload {
    pub report_no: String,
    pub task_id: i64,
    pub inspection_type: i32,
    pub material_id: i64,
    pub batch_no: Option<String>,
    pub inspection_date: chrono::NaiveDate,
    pub inspection_time: chrono::DateTime<chrono::Utc>,
    pub inspector_id: i64,
    pub inspection_quantity: f64,
    pub sample_quantity: f64,
    pub qualified_quantity: f64,
    pub unqualified_quantity: f64,
    pub unit: String,
    pub inspection_result: i32,
    pub disposition: Option<i32>,
    pub major_defects: Option<i32>,
    pub minor_defects: Option<i32>,
    pub critical_defects: Option<i32>,
    pub conclusion: Option<String>,
    pub improvement_suggestions: Option<String>,
    pub remark: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct InspectionReportDto {
    pub id: i64,
    pub report_no: String,
    pub task_id: i64,
    pub inspection_type: i32,
    pub material_id: i64,
    pub batch_no: Option<String>,
    pub inspection_date: chrono::NaiveDate,
    pub inspection_time: chrono::DateTime<chrono::Utc>,
    pub inspector_id: i64,
    pub inspection_quantity: f64,
    pub sample_quantity: f64,
    pub qualified_quantity: f64,
    pub unqualified_quantity: f64,
    pub qualified_rate: f64,
    pub inspection_result: i32,
    pub disposition: Option<i32>,
    pub report_status: i32,
}

// Nonconforming Products (NCR)
#[derive(Debug, Deserialize)]
pub struct NcrQuery {
    pub ncr_status: Option<i32>,
    pub material_id: Option<i64>,
    pub batch_no: Option<String>,
    pub defect_level: Option<i32>,
    pub source_type: Option<i32>,
    #[serde(default = "default_page")]
    pub page: u64,
    #[serde(default = "default_page_size")]
    pub page_size: u64,
}

#[derive(Debug, Deserialize)]
pub struct NcrPayload {
    pub ncr_no: String,
    pub report_id: Option<i64>,
    pub source_type: i32,
    pub material_id: i64,
    pub batch_no: Option<String>,
    pub defect_quantity: f64,
    pub unit: String,
    pub defect_code: Option<String>,
    pub defect_name: Option<String>,
    pub defect_level: Option<i32>,
    pub defect_description: Option<String>,
    pub found_date: chrono::NaiveDate,
    pub found_time: chrono::DateTime<chrono::Utc>,
    pub finder_id: i64,
    pub remark: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct NcrDto {
    pub id: i64,
    pub ncr_no: String,
    pub material_id: i64,
    pub batch_no: Option<String>,
    pub defect_quantity: f64,
    pub unit: String,
    pub defect_code: Option<String>,
    pub defect_name: Option<String>,
    pub defect_level: i32,
    pub found_date: chrono::NaiveDate,
    pub ncr_status: i32,
    pub disposition: Option<i32>,
}

// Customer Complaints
#[derive(Debug, Deserialize)]
pub struct ComplaintQuery {
    pub customer_id: Option<i64>,
    pub complaint_status: Option<i32>,
    pub complaint_type: Option<i32>,
    pub material_id: Option<i64>,
    #[serde(default = "default_page")]
    pub page: u64,
    #[serde(default = "default_page_size")]
    pub page_size: u64,
}

#[derive(Debug, Deserialize)]
pub struct ComplaintPayload {
    pub complaint_no: String,
    pub customer_id: i64,
    pub material_id: i64,
    pub batch_no: Option<String>,
    pub complaint_type: i32,
    pub complaint_level: Option<i32>,
    pub complaint_date: chrono::NaiveDate,
    pub complaint_time: chrono::DateTime<chrono::Utc>,
    pub complaint_quantity: Option<f64>,
    pub unit: Option<String>,
    pub complaint_description: String,
    pub defect_description: Option<String>,
    pub receiver_id: i64,
    pub remark: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ComplaintDto {
    pub id: i64,
    pub complaint_no: String,
    pub customer_id: i64,
    pub material_id: i64,
    pub batch_no: Option<String>,
    pub complaint_type: i32,
    pub complaint_level: i32,
    pub complaint_date: chrono::NaiveDate,
    pub complaint_status: i32,
    pub handler_id: Option<i64>,
}

// Rework Orders
#[derive(Debug, Deserialize)]
pub struct ReworkOrderQuery {
    pub ncr_id: Option<i64>,
    pub rework_status: Option<i32>,
    pub material_id: Option<i64>,
    #[serde(default = "default_page")]
    pub page: u64,
    #[serde(default = "default_page_size")]
    pub page_size: u64,
}

#[derive(Debug, Deserialize)]
pub struct ReworkOrderPayload {
    pub rework_no: String,
    pub ncr_id: i64,
    pub material_id: i64,
    pub batch_no: Option<String>,
    pub rework_quantity: f64,
    pub unit: String,
    pub rework_type: i32,
    pub rework_reason: String,
    pub rework_plan: Option<String>,
    pub workshop_id: Option<i64>,
    pub plan_start_date: Option<chrono::NaiveDate>,
    pub plan_end_date: Option<chrono::NaiveDate>,
    pub handler_id: Option<i64>,
    pub remark: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ReworkOrderDto {
    pub id: i64,
    pub rework_no: String,
    pub ncr_id: i64,
    pub material_id: i64,
    pub batch_no: Option<String>,
    pub rework_quantity: f64,
    pub completed_quantity: f64,
    pub qualified_quantity: f64,
    pub unit: String,
    pub rework_type: i32,
    pub rework_status: i32,
}

// Measuring Equipment
#[derive(Debug, Deserialize)]
pub struct MeasuringEquipmentQuery {
    pub equipment_type: Option<i32>,
    pub equipment_status: Option<i32>,
    pub keyword: Option<String>,
    #[serde(default = "default_page")]
    pub page: u64,
    #[serde(default = "default_page_size")]
    pub page_size: u64,
}

#[derive(Debug, Deserialize)]
pub struct MeasuringEquipmentPayload {
    pub equipment_code: String,
    pub equipment_name: String,
    pub equipment_model: Option<String>,
    pub equipment_type: i32,
    pub manufacturer: Option<String>,
    pub calibration_cycle: Option<i32>,
    pub next_calibration_date: Option<chrono::NaiveDate>,
    pub equipment_status: Option<i32>,
    pub location: Option<String>,
    pub remark: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct MeasuringEquipmentDto {
    pub id: i64,
    pub equipment_code: String,
    pub equipment_name: String,
    pub equipment_model: Option<String>,
    pub equipment_type: i32,
    pub equipment_status: i32,
    pub next_calibration_date: Option<chrono::NaiveDate>,
    pub location: Option<String>,
}

// Supplier Quality Evaluations
#[derive(Debug, Deserialize)]
pub struct SupplierQualityEvaluationQuery {
    pub supplier_id: Option<i64>,
    pub evaluation_period: Option<String>,
    #[serde(default = "default_page")]
    pub page: u64,
    #[serde(default = "default_page_size")]
    pub page_size: u64,
}

#[derive(Debug, Deserialize)]
pub struct SupplierQualityEvaluationPayload {
    pub evaluation_no: String,
    pub supplier_id: i64,
    pub evaluation_period: String,
    pub evaluation_date: chrono::NaiveDate,
    pub evaluator_id: i64,
    pub total_receipts: Option<i32>,
    pub qualified_receipts: Option<i32>,
    pub unqualified_receipts: Option<i32>,
    pub batch_qualified_rate: Option<f64>,
    pub quantity_qualified_rate: Option<f64>,
    pub on_time_delivery_rate: Option<f64>,
    pub quality_score: Option<f64>,
    pub delivery_score: Option<f64>,
    pub service_score: Option<f64>,
    pub total_score: Option<f64>,
    pub evaluation_level: Option<String>,
    pub evaluation_conclusion: Option<String>,
    pub remark: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct SupplierQualityEvaluationDto {
    pub id: i64,
    pub evaluation_no: String,
    pub supplier_id: i64,
    pub evaluation_period: String,
    pub evaluation_date: chrono::NaiveDate,
    pub batch_qualified_rate: f64,
    pub quantity_qualified_rate: f64,
    pub total_score: f64,
    pub evaluation_level: String,
    pub is_approved: i32,
}

// Quality Traceability Records
#[derive(Debug, Deserialize)]
pub struct QualityTraceabilityRecordQuery {
    pub material_id: Option<i64>,
    pub batch_no: Option<String>,
    pub trace_type: Option<i32>,
    #[serde(default = "default_page")]
    pub page: u64,
    #[serde(default = "default_page_size")]
    pub page_size: u64,
}

#[derive(Debug, Deserialize)]
pub struct QualityTraceabilityRecordPayload {
    pub trace_no: String,
    pub trace_type: i32,
    pub material_id: i64,
    pub batch_no: String,
    pub serial_no: Option<String>,
    pub production_order_no: Option<String>,
    pub trace_reason: Option<String>,
    pub trace_date: chrono::NaiveDate,
    pub tracer_id: i64,
    pub remark: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct QualityTraceabilityRecordDto {
    pub id: i64,
    pub trace_no: String,
    pub trace_type: i32,
    pub material_id: i64,
    pub batch_no: String,
    pub serial_no: Option<String>,
    pub production_order_no: Option<String>,
    pub trace_date: chrono::NaiveDate,
    pub trace_result: Option<String>,
}

// Quality Costs
#[derive(Debug, Deserialize)]
pub struct QualityCostQuery {
    pub cost_period: Option<String>,
    pub cost_category: Option<i32>,
    #[serde(default = "default_page")]
    pub page: u64,
    #[serde(default = "default_page_size")]
    pub page_size: u64,
}

#[derive(Debug, Deserialize)]
pub struct QualityCostPayload {
    pub cost_no: String,
    pub cost_period: String,
    pub cost_date: chrono::NaiveDate,
    pub cost_category: i32,
    pub cost_type: String,
    pub cost_item: String,
    pub cost_amount: f64,
    pub cost_description: Option<String>,
    pub remark: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct QualityCostDto {
    pub id: i64,
    pub cost_no: String,
    pub cost_period: String,
    pub cost_date: chrono::NaiveDate,
    pub cost_category: i32,
    pub cost_type: String,
    pub cost_item: String,
    pub cost_amount: f64,
}

// Quality KPI
#[derive(Debug, Deserialize)]
pub struct QualityKpiQuery {
    pub kpi_type: Option<i32>,
    pub dept_id: Option<i64>,
    pub workshop_id: Option<i64>,
    pub start_date: Option<chrono::NaiveDate>,
    pub end_date: Option<chrono::NaiveDate>,
    #[serde(default = "default_page")]
    pub page: u64,
    #[serde(default = "default_page_size")]
    pub page_size: u64,
}

#[derive(Debug, Deserialize)]
pub struct QualityKpiPayload {
    pub kpi_date: chrono::NaiveDate,
    pub kpi_type: i32,
    pub dept_id: Option<i64>,
    pub workshop_id: Option<i64>,
    pub batch_qualified_rate: f64,
    pub quantity_qualified_rate: f64,
    pub first_pass_yield: f64,
    pub iqc_qualified_rate: f64,
    pub ipqc_qualified_rate: f64,
    pub fqc_qualified_rate: f64,
    pub oqc_qualified_rate: f64,
    pub rework_rate: f64,
    pub scrap_rate: f64,
    pub complaint_rate: f64,
    pub ncr_count: i32,
    pub total_quality_cost: f64,
    pub quality_cost_rate: f64,
    pub dppm: f64,
    pub cpk: Option<f64>,
    pub sigma_level: Option<f64>,
}

#[derive(Debug, Serialize)]
pub struct QualityKpiDto {
    pub id: i64,
    pub kpi_date: chrono::NaiveDate,
    pub kpi_type: i32,
    pub dept_id: Option<i64>,
    pub workshop_id: Option<i64>,
    pub batch_qualified_rate: f64,
    pub quantity_qualified_rate: f64,
    pub first_pass_yield: f64,
    pub iqc_qualified_rate: f64,
    pub ipqc_qualified_rate: f64,
    pub fqc_qualified_rate: f64,
    pub oqc_qualified_rate: f64,
    pub rework_rate: f64,
    pub scrap_rate: f64,
    pub complaint_rate: f64,
    pub ncr_count: i32,
    pub total_quality_cost: f64,
    pub quality_cost_rate: f64,
    pub dppm: f64,
    pub cpk: Option<f64>,
    pub sigma_level: Option<f64>,
}

