-- Quality Management Module Tables for MES
-- Converted from MySQL to PostgreSQL

-- 1. Quality Inspection Tasks
CREATE TABLE IF NOT EXISTS quality_inspection_tasks (
    id BIGSERIAL PRIMARY KEY,
    task_no VARCHAR(50) NOT NULL UNIQUE,
    inspection_type SMALLINT NOT NULL, -- 1:IQC, 2:IPQC, 3:FQC, 4:OQC, 5:委外检验
    source_type SMALLINT NOT NULL, -- 1:入库单, 2:生产工单, 3:完工单, 4:出库单
    source_order_no VARCHAR(50) NOT NULL,
    material_id BIGINT NOT NULL,
    batch_no VARCHAR(50),
    supplier_id BIGINT,
    production_order_id BIGINT,
    work_order_id BIGINT,
    process_id BIGINT,
    standard_id BIGINT,
    inspection_quantity NUMERIC(12,4) NOT NULL,
    sample_quantity NUMERIC(12,4) DEFAULT 0.0000,
    qualified_quantity NUMERIC(12,4) DEFAULT 0.0000,
    unqualified_quantity NUMERIC(12,4) DEFAULT 0.0000,
    unit VARCHAR(20) NOT NULL,
    inspection_level VARCHAR(20),
    aql NUMERIC(5,2),
    sampling_plan VARCHAR(100),
    plan_start_time TIMESTAMPTZ,
    plan_end_time TIMESTAMPTZ,
    actual_start_time TIMESTAMPTZ,
    actual_end_time TIMESTAMPTZ,
    inspector_id BIGINT,
    task_status SMALLINT DEFAULT 1, -- 1:待检验, 2:检验中, 3:已完成, 4:已取消
    inspection_result SMALLINT, -- 1:合格, 2:不合格, 3:让步接收, 4:待定
    is_urgent SMALLINT DEFAULT 0,
    priority SMALLINT DEFAULT 3, -- 1:紧急, 2:高, 3:普通, 4:低
    remark VARCHAR(500),
    created_by BIGINT,
    created_time TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_by BIGINT,
    updated_time TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    is_deleted SMALLINT DEFAULT 0
);

CREATE INDEX idx_quality_tasks_inspection_type ON quality_inspection_tasks(inspection_type);
CREATE INDEX idx_quality_tasks_source_order_no ON quality_inspection_tasks(source_order_no);
CREATE INDEX idx_quality_tasks_material_id ON quality_inspection_tasks(material_id);
CREATE INDEX idx_quality_tasks_batch_no ON quality_inspection_tasks(batch_no);
CREATE INDEX idx_quality_tasks_task_status ON quality_inspection_tasks(task_status);
CREATE INDEX idx_quality_tasks_plan_start_time ON quality_inspection_tasks(plan_start_time);

-- 2. Quality Inspection Reports
CREATE TABLE IF NOT EXISTS quality_inspection_reports (
    id BIGSERIAL PRIMARY KEY,
    report_no VARCHAR(50) NOT NULL UNIQUE,
    task_id BIGINT NOT NULL,
    inspection_type SMALLINT NOT NULL,
    material_id BIGINT NOT NULL,
    batch_no VARCHAR(50),
    supplier_id BIGINT,
    production_order_id BIGINT,
    inspection_date DATE NOT NULL,
    inspection_time TIMESTAMPTZ NOT NULL,
    inspector_id BIGINT NOT NULL,
    reviewer_id BIGINT,
    review_time TIMESTAMPTZ,
    inspection_quantity NUMERIC(12,4) NOT NULL,
    sample_quantity NUMERIC(12,4) NOT NULL,
    qualified_quantity NUMERIC(12,4) NOT NULL,
    unqualified_quantity NUMERIC(12,4) NOT NULL,
    unit VARCHAR(20) NOT NULL,
    qualified_rate NUMERIC(5,2) DEFAULT 0.00,
    inspection_result SMALLINT NOT NULL,
    disposition SMALLINT, -- 1:接收, 2:退货, 3:返工, 4:报废, 5:降级使用
    major_defects INTEGER DEFAULT 0,
    minor_defects INTEGER DEFAULT 0,
    critical_defects INTEGER DEFAULT 0,
    inspection_environment VARCHAR(200),
    inspection_equipment VARCHAR(200),
    report_status SMALLINT DEFAULT 1, -- 1:待审核, 2:已审核, 3:已归档
    conclusion TEXT,
    improvement_suggestions TEXT,
    attachment_url VARCHAR(500),
    remark VARCHAR(500),
    created_by BIGINT,
    created_time TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_by BIGINT,
    updated_time TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    is_deleted SMALLINT DEFAULT 0
);

CREATE INDEX idx_quality_reports_task_id ON quality_inspection_reports(task_id);
CREATE INDEX idx_quality_reports_material_id ON quality_inspection_reports(material_id);
CREATE INDEX idx_quality_reports_batch_no ON quality_inspection_reports(batch_no);
CREATE INDEX idx_quality_reports_inspection_date ON quality_inspection_reports(inspection_date);
CREATE INDEX idx_quality_reports_inspection_result ON quality_inspection_reports(inspection_result);
CREATE INDEX idx_quality_reports_report_status ON quality_inspection_reports(report_status);

-- 3. Quality Inspection Items
CREATE TABLE IF NOT EXISTS quality_inspection_items (
    id BIGSERIAL PRIMARY KEY,
    report_id BIGINT NOT NULL,
    item_code VARCHAR(50) NOT NULL,
    item_name VARCHAR(200) NOT NULL,
    item_type SMALLINT NOT NULL, -- 1:尺寸, 2:外观, 3:性能, 4:功能, 5:其他
    inspection_method VARCHAR(200),
    standard_value VARCHAR(100),
    upper_limit NUMERIC(12,4),
    lower_limit NUMERIC(12,4),
    actual_value VARCHAR(100),
    unit VARCHAR(20),
    inspection_equipment VARCHAR(100),
    item_result SMALLINT NOT NULL, -- 1:合格, 2:不合格
    defect_quantity INTEGER DEFAULT 0,
    defect_code VARCHAR(50),
    defect_level SMALLINT, -- 1:致命, 2:严重, 3:一般, 4:轻微
    is_key_item SMALLINT DEFAULT 0,
    sequence_no INTEGER DEFAULT 0,
    remark VARCHAR(500),
    created_by BIGINT,
    created_time TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_by BIGINT,
    updated_time TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    is_deleted SMALLINT DEFAULT 0
);

CREATE INDEX idx_quality_items_report_id ON quality_inspection_items(report_id);
CREATE INDEX idx_quality_items_item_code ON quality_inspection_items(item_code);
CREATE INDEX idx_quality_items_item_result ON quality_inspection_items(item_result);

-- 4. Nonconforming Products (NCR)
CREATE TABLE IF NOT EXISTS nonconforming_products (
    id BIGSERIAL PRIMARY KEY,
    ncr_no VARCHAR(50) NOT NULL UNIQUE,
    report_id BIGINT,
    source_type SMALLINT NOT NULL, -- 1:来料检验, 2:过程检验, 3:成品检验, 4:客户退货, 5:其他
    source_order_no VARCHAR(50),
    material_id BIGINT NOT NULL,
    batch_no VARCHAR(50),
    serial_no VARCHAR(50),
    supplier_id BIGINT,
    customer_id BIGINT,
    production_order_id BIGINT,
    work_order_id BIGINT,
    process_id BIGINT,
    defect_quantity NUMERIC(12,4) NOT NULL,
    unit VARCHAR(20) NOT NULL,
    defect_code VARCHAR(50),
    defect_name VARCHAR(200),
    defect_level SMALLINT DEFAULT 3,
    defect_description TEXT,
    defect_location VARCHAR(200),
    defect_images TEXT, -- JSON format
    found_date DATE NOT NULL,
    found_time TIMESTAMPTZ NOT NULL,
    finder_id BIGINT NOT NULL,
    responsible_dept_id BIGINT,
    responsible_person_id BIGINT,
    root_cause TEXT,
    disposition SMALLINT, -- 1:返工, 2:报废, 3:让步接收, 4:退货, 5:降级使用, 6:挑选
    disposition_quantity NUMERIC(12,4) DEFAULT 0.0000,
    disposition_date DATE,
    disposition_handler_id BIGINT,
    disposition_result VARCHAR(500),
    rework_order_no VARCHAR(50),
    corrective_action TEXT,
    preventive_action TEXT,
    ncr_status SMALLINT DEFAULT 1, -- 1:待处置, 2:处置中, 3:已处置, 4:已验证, 5:已关闭
    is_repetitive SMALLINT DEFAULT 0,
    closure_date DATE,
    remark VARCHAR(500),
    created_by BIGINT,
    created_time TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_by BIGINT,
    updated_time TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    is_deleted SMALLINT DEFAULT 0
);

CREATE INDEX idx_ncr_report_id ON nonconforming_products(report_id);
CREATE INDEX idx_ncr_material_id ON nonconforming_products(material_id);
CREATE INDEX idx_ncr_batch_no ON nonconforming_products(batch_no);
CREATE INDEX idx_ncr_supplier_id ON nonconforming_products(supplier_id);
CREATE INDEX idx_ncr_ncr_status ON nonconforming_products(ncr_status);
CREATE INDEX idx_ncr_found_date ON nonconforming_products(found_date);

-- 5. Rework Orders
CREATE TABLE IF NOT EXISTS rework_orders (
    id BIGSERIAL PRIMARY KEY,
    rework_no VARCHAR(50) NOT NULL UNIQUE,
    ncr_id BIGINT NOT NULL,
    material_id BIGINT NOT NULL,
    batch_no VARCHAR(50),
    rework_quantity NUMERIC(12,4) NOT NULL,
    completed_quantity NUMERIC(12,4) DEFAULT 0.0000,
    qualified_quantity NUMERIC(12,4) DEFAULT 0.0000,
    scrap_quantity NUMERIC(12,4) DEFAULT 0.0000,
    unit VARCHAR(20) NOT NULL,
    rework_type SMALLINT NOT NULL, -- 1:工序返工, 2:全检挑选, 3:返修, 4:其他
    rework_reason VARCHAR(500) NOT NULL,
    rework_plan TEXT,
    rework_process VARCHAR(500),
    workshop_id BIGINT,
    plan_start_date DATE,
    plan_end_date DATE,
    actual_start_date DATE,
    actual_end_date DATE,
    handler_id BIGINT,
    rework_cost NUMERIC(15,2) DEFAULT 0.00,
    rework_status SMALLINT DEFAULT 1, -- 1:待返工, 2:返工中, 3:已完成, 4:已取消
    inspection_result SMALLINT, -- 1:合格, 2:不合格
    inspector_id BIGINT,
    inspection_time TIMESTAMPTZ,
    remark VARCHAR(500),
    created_by BIGINT,
    created_time TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_by BIGINT,
    updated_time TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    is_deleted SMALLINT DEFAULT 0
);

CREATE INDEX idx_rework_ncr_id ON rework_orders(ncr_id);
CREATE INDEX idx_rework_material_id ON rework_orders(material_id);
CREATE INDEX idx_rework_batch_no ON rework_orders(batch_no);
CREATE INDEX idx_rework_rework_status ON rework_orders(rework_status);

-- 6. Customer Complaints
CREATE TABLE IF NOT EXISTS customer_complaints (
    id BIGSERIAL PRIMARY KEY,
    complaint_no VARCHAR(50) NOT NULL UNIQUE,
    customer_id BIGINT NOT NULL,
    material_id BIGINT NOT NULL,
    batch_no VARCHAR(50),
    sales_order_no VARCHAR(50),
    production_order_no VARCHAR(50),
    complaint_type SMALLINT NOT NULL, -- 1:质量问题, 2:交期问题, 3:服务问题, 4:包装问题, 5:其他
    complaint_level SMALLINT DEFAULT 3, -- 1:严重, 2:重要, 3:一般
    complaint_date DATE NOT NULL,
    complaint_time TIMESTAMPTZ NOT NULL,
    complaint_quantity NUMERIC(12,4),
    unit VARCHAR(20),
    complaint_description TEXT NOT NULL,
    defect_description TEXT,
    defect_images TEXT, -- JSON format
    customer_requirement VARCHAR(500),
    receiver_id BIGINT NOT NULL,
    handler_id BIGINT,
    response_deadline DATE,
    response_time TIMESTAMPTZ,
    response_content TEXT,
    root_cause_analysis TEXT,
    corrective_action TEXT,
    preventive_action TEXT,
    compensation_amount NUMERIC(15,2) DEFAULT 0.00,
    processing_cost NUMERIC(15,2) DEFAULT 0.00,
    complaint_status SMALLINT DEFAULT 1, -- 1:待处理, 2:处理中, 3:待验证, 4:已关闭
    is_valid SMALLINT DEFAULT 1,
    customer_satisfaction SMALLINT, -- 1-5
    closure_date DATE,
    remark VARCHAR(500),
    created_by BIGINT,
    created_time TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_by BIGINT,
    updated_time TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    is_deleted SMALLINT DEFAULT 0
);

CREATE INDEX idx_complaints_customer_id ON customer_complaints(customer_id);
CREATE INDEX idx_complaints_material_id ON customer_complaints(material_id);
CREATE INDEX idx_complaints_batch_no ON customer_complaints(batch_no);
CREATE INDEX idx_complaints_complaint_status ON customer_complaints(complaint_status);
CREATE INDEX idx_complaints_complaint_date ON customer_complaints(complaint_date);

-- 7. Supplier Quality Evaluations
CREATE TABLE IF NOT EXISTS supplier_quality_evaluations (
    id BIGSERIAL PRIMARY KEY,
    evaluation_no VARCHAR(50) NOT NULL UNIQUE,
    supplier_id BIGINT NOT NULL,
    evaluation_period VARCHAR(50) NOT NULL,
    evaluation_date DATE NOT NULL,
    evaluator_id BIGINT NOT NULL,
    total_receipts INTEGER DEFAULT 0,
    total_quantity NUMERIC(12,4) DEFAULT 0.0000,
    qualified_receipts INTEGER DEFAULT 0,
    qualified_quantity NUMERIC(12,4) DEFAULT 0.0000,
    unqualified_receipts INTEGER DEFAULT 0,
    unqualified_quantity NUMERIC(12,4) DEFAULT 0.0000,
    batch_qualified_rate NUMERIC(5,2) DEFAULT 0.00,
    quantity_qualified_rate NUMERIC(5,2) DEFAULT 0.00,
    on_time_delivery_rate NUMERIC(5,2) DEFAULT 0.00,
    response_speed_score NUMERIC(5,2) DEFAULT 0.00,
    service_attitude_score NUMERIC(5,2) DEFAULT 0.00,
    quality_score NUMERIC(5,2) DEFAULT 0.00,
    delivery_score NUMERIC(5,2) DEFAULT 0.00,
    service_score NUMERIC(5,2) DEFAULT 0.00,
    total_score NUMERIC(5,2) DEFAULT 0.00,
    evaluation_level CHAR(1) DEFAULT 'C', -- A/B/C/D
    major_issues TEXT,
    improvement_requirements TEXT,
    evaluation_conclusion TEXT,
    is_approved SMALLINT DEFAULT 0,
    approver_id BIGINT,
    approval_time TIMESTAMPTZ,
    remark VARCHAR(500),
    created_by BIGINT,
    created_time TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_by BIGINT,
    updated_time TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    is_deleted SMALLINT DEFAULT 0
);

CREATE INDEX idx_supplier_eval_supplier_id ON supplier_quality_evaluations(supplier_id);
CREATE INDEX idx_supplier_eval_evaluation_period ON supplier_quality_evaluations(evaluation_period);
CREATE INDEX idx_supplier_eval_evaluation_date ON supplier_quality_evaluations(evaluation_date);

-- 8. Quality Traceability Records
CREATE TABLE IF NOT EXISTS quality_traceability_records (
    id BIGSERIAL PRIMARY KEY,
    trace_no VARCHAR(50) NOT NULL UNIQUE,
    trace_type SMALLINT NOT NULL, -- 1:正向追溯, 2:反向追溯
    material_id BIGINT NOT NULL,
    batch_no VARCHAR(50) NOT NULL,
    serial_no VARCHAR(50),
    production_order_no VARCHAR(50),
    sales_order_no VARCHAR(50),
    customer_id BIGINT,
    supplier_id BIGINT,
    supplier_batch_no VARCHAR(50),
    production_date DATE,
    inspection_report_no VARCHAR(50),
    inspection_result SMALLINT, -- 1:合格, 2:不合格
    workshop_id BIGINT,
    production_line VARCHAR(100),
    operator_ids TEXT, -- JSON format
    equipment_ids TEXT, -- JSON format
    raw_material_info TEXT, -- JSON format
    process_info TEXT, -- JSON format
    quality_info TEXT, -- JSON format
    trace_reason VARCHAR(500),
    trace_result TEXT,
    trace_date DATE NOT NULL,
    tracer_id BIGINT NOT NULL,
    remark VARCHAR(500),
    created_by BIGINT,
    created_time TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_by BIGINT,
    updated_time TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    is_deleted SMALLINT DEFAULT 0
);

CREATE INDEX idx_trace_material_id ON quality_traceability_records(material_id);
CREATE INDEX idx_trace_batch_no ON quality_traceability_records(batch_no);
CREATE INDEX idx_trace_serial_no ON quality_traceability_records(serial_no);
CREATE INDEX idx_trace_production_order_no ON quality_traceability_records(production_order_no);
CREATE INDEX idx_trace_trace_date ON quality_traceability_records(trace_date);

-- 9. Measuring Equipment
CREATE TABLE IF NOT EXISTS measuring_equipment (
    id BIGSERIAL PRIMARY KEY,
    equipment_code VARCHAR(50) NOT NULL UNIQUE,
    equipment_name VARCHAR(200) NOT NULL,
    equipment_model VARCHAR(100),
    equipment_type SMALLINT NOT NULL, -- 1:量具, 2:仪器, 3:检测设备, 4:其他
    manufacturer VARCHAR(200),
    serial_no VARCHAR(100),
    purchase_date DATE,
    accuracy_level VARCHAR(50),
    measurement_range VARCHAR(100),
    calibration_cycle INTEGER DEFAULT 365,
    last_calibration_date DATE,
    next_calibration_date DATE,
    calibration_institution VARCHAR(200),
    calibration_certificate_no VARCHAR(100),
    equipment_status SMALLINT DEFAULT 1, -- 1:正常, 2:待校准, 3:校准中, 4:停用, 5:报废
    location VARCHAR(200),
    custodian_id BIGINT,
    usage_frequency VARCHAR(50),
    maintenance_requirements VARCHAR(500),
    image_url VARCHAR(500),
    remark VARCHAR(500),
    created_by BIGINT,
    created_time TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_by BIGINT,
    updated_time TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    is_deleted SMALLINT DEFAULT 0
);

CREATE INDEX idx_measuring_equipment_type ON measuring_equipment(equipment_type);
CREATE INDEX idx_measuring_equipment_status ON measuring_equipment(equipment_status);
CREATE INDEX idx_measuring_equipment_next_calibration ON measuring_equipment(next_calibration_date);

-- 10. Equipment Calibration Records
CREATE TABLE IF NOT EXISTS equipment_calibration_records (
    id BIGSERIAL PRIMARY KEY,
    record_no VARCHAR(50) NOT NULL UNIQUE,
    equipment_id BIGINT NOT NULL,
    calibration_type SMALLINT NOT NULL, -- 1:内部校准, 2:外部校准
    calibration_date DATE NOT NULL,
    calibration_institution VARCHAR(200),
    calibrator_id BIGINT,
    calibration_standard VARCHAR(200),
    calibration_result SMALLINT NOT NULL, -- 1:合格, 2:不合格
    certificate_no VARCHAR(100),
    certificate_valid_date DATE,
    next_calibration_date DATE,
    calibration_cost NUMERIC(15,2) DEFAULT 0.00,
    deviation_before VARCHAR(200),
    deviation_after VARCHAR(200),
    adjustment_content VARCHAR(500),
    certificate_url VARCHAR(500),
    remark VARCHAR(500),
    created_by BIGINT,
    created_time TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_by BIGINT,
    updated_time TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    is_deleted SMALLINT DEFAULT 0
);

CREATE INDEX idx_calibration_equipment_id ON equipment_calibration_records(equipment_id);
CREATE INDEX idx_calibration_calibration_date ON equipment_calibration_records(calibration_date);
CREATE INDEX idx_calibration_calibration_result ON equipment_calibration_records(calibration_result);

-- 11. Quality Costs
CREATE TABLE IF NOT EXISTS quality_costs (
    id BIGSERIAL PRIMARY KEY,
    cost_no VARCHAR(50) NOT NULL UNIQUE,
    cost_period VARCHAR(50) NOT NULL,
    cost_date DATE NOT NULL,
    cost_category SMALLINT NOT NULL, -- 1:预防成本, 2:鉴定成本, 3:内部失败成本, 4:外部失败成本
    cost_type VARCHAR(100) NOT NULL,
    cost_item VARCHAR(200) NOT NULL,
    material_id BIGINT,
    production_order_id BIGINT,
    ncr_id BIGINT,
    complaint_id BIGINT,
    cost_amount NUMERIC(15,2) NOT NULL,
    quantity NUMERIC(12,4),
    unit VARCHAR(20),
    dept_id BIGINT,
    cost_description VARCHAR(500),
    handler_id BIGINT,
    remark VARCHAR(500),
    created_by BIGINT,
    created_time TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_by BIGINT,
    updated_time TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    is_deleted SMALLINT DEFAULT 0
);

CREATE INDEX idx_quality_costs_cost_period ON quality_costs(cost_period);
CREATE INDEX idx_quality_costs_cost_category ON quality_costs(cost_category);
CREATE INDEX idx_quality_costs_cost_date ON quality_costs(cost_date);

-- 12. Quality KPI
CREATE TABLE IF NOT EXISTS quality_kpi (
    id BIGSERIAL PRIMARY KEY,
    kpi_date DATE NOT NULL,
    kpi_type SMALLINT NOT NULL, -- 1:日报, 2:周报, 3:月报
    dept_id BIGINT,
    workshop_id BIGINT,
    total_inspections INTEGER DEFAULT 0,
    qualified_inspections INTEGER DEFAULT 0,
    unqualified_inspections INTEGER DEFAULT 0,
    inspection_quantity NUMERIC(12,4) DEFAULT 0.0000,
    qualified_quantity NUMERIC(12,4) DEFAULT 0.0000,
    unqualified_quantity NUMERIC(12,4) DEFAULT 0.0000,
    batch_qualified_rate NUMERIC(5,2) DEFAULT 0.00,
    quantity_qualified_rate NUMERIC(5,2) DEFAULT 0.00,
    first_pass_yield NUMERIC(5,2) DEFAULT 0.00,
    iqc_qualified_rate NUMERIC(5,2) DEFAULT 0.00,
    ipqc_qualified_rate NUMERIC(5,2) DEFAULT 0.00,
    fqc_qualified_rate NUMERIC(5,2) DEFAULT 0.00,
    oqc_qualified_rate NUMERIC(5,2) DEFAULT 0.00,
    rework_quantity NUMERIC(12,4) DEFAULT 0.0000,
    scrap_quantity NUMERIC(12,4) DEFAULT 0.0000,
    rework_rate NUMERIC(5,2) DEFAULT 0.00,
    scrap_rate NUMERIC(5,2) DEFAULT 0.00,
    customer_complaints INTEGER DEFAULT 0,
    valid_complaints INTEGER DEFAULT 0,
    complaint_rate NUMERIC(5,2) DEFAULT 0.00,
    ncr_count INTEGER DEFAULT 0,
    major_ncr_count INTEGER DEFAULT 0,
    preventive_cost NUMERIC(15,2) DEFAULT 0.00,
    appraisal_cost NUMERIC(15,2) DEFAULT 0.00,
    internal_failure_cost NUMERIC(15,2) DEFAULT 0.00,
    external_failure_cost NUMERIC(15,2) DEFAULT 0.00,
    total_quality_cost NUMERIC(15,2) DEFAULT 0.00,
    quality_cost_rate NUMERIC(5,2) DEFAULT 0.00,
    dppm NUMERIC(10,2) DEFAULT 0.00,
    cpk NUMERIC(5,2),
    sigma_level NUMERIC(3,1),
    remark VARCHAR(500),
    created_by BIGINT,
    created_time TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_by BIGINT,
    updated_time TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_quality_kpi_kpi_date ON quality_kpi(kpi_date);
CREATE INDEX idx_quality_kpi_kpi_type ON quality_kpi(kpi_type);
CREATE INDEX idx_quality_kpi_dept_id ON quality_kpi(dept_id);
CREATE INDEX idx_quality_kpi_workshop_id ON quality_kpi(workshop_id);


