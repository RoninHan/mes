#[derive(Clone, Debug, PartialEq)]
pub struct Model {
    
    pub id: i64,
    pub record_no: String,
    pub equipment_id: i64,
    pub calibration_type: i16, // 1:内部校准, 2:外部校准
    pub calibration_date: Date,
    pub calibration_institution: Option<String>,
    pub calibrator_id: Option<i64>,
    pub calibration_standard: Option<String>,
    pub calibration_result: i16, // 1:合格, 2:不合格
    pub certificate_no: Option<String>,
    pub certificate_valid_date: Option<chrono::NaiveDate>,
    pub next_calibration_date: Option<chrono::NaiveDate>,
    pub calibration_cost: Decimal,
    pub deviation_before: Option<String>,
    pub deviation_after: Option<String>,
    pub adjustment_content: Option<String>,
    pub certificate_url: Option<String>,
    pub remark: Option<String>,
    pub created_by: Option<i64>,
    pub created_time: chrono::DateTime<chrono::Utc>,
    pub updated_by: Option<i64>,
    pub updated_time: chrono::DateTime<chrono::Utc>,
    pub is_deleted: i16,
}

