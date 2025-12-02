-- =============================================
-- MES系统 - 质量管理模块数据库表结构
-- 创建日期: 2025-10-29
-- 数据库: MySQL 5.7+
-- =============================================

-- 设置字符集
SET NAMES utf8mb4;
SET FOREIGN_KEY_CHECKS = 0;

-- =============================================
-- 1. 质检任务表 (quality_inspection_tasks)
-- =============================================
DROP TABLE IF EXISTS `quality_inspection_tasks`;
CREATE TABLE `quality_inspection_tasks` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '任务ID',
  `task_no` varchar(50) NOT NULL COMMENT '任务编号',
  `inspection_type` tinyint(1) NOT NULL COMMENT '检验类型：1-来料检验（IQC），2-过程检验（IPQC），3-成品检验（FQC），4-出货检验（OQC），5-委外检验',
  `source_type` tinyint(1) NOT NULL COMMENT '来源类型：1-入库单，2-生产工单，3-完工单，4-出库单',
  `source_order_no` varchar(50) NOT NULL COMMENT '来源单号',
  `material_id` bigint(20) NOT NULL COMMENT '物料ID',
  `batch_no` varchar(50) DEFAULT NULL COMMENT '批次号',
  `supplier_id` bigint(20) DEFAULT NULL COMMENT '供应商ID（来料检验）',
  `production_order_id` bigint(20) DEFAULT NULL COMMENT '生产订单ID',
  `work_order_id` bigint(20) DEFAULT NULL COMMENT '工单ID',
  `process_id` bigint(20) DEFAULT NULL COMMENT '工序ID',
  `standard_id` bigint(20) DEFAULT NULL COMMENT '质检标准ID',
  `inspection_quantity` decimal(12,4) NOT NULL COMMENT '送检数量',
  `sample_quantity` decimal(12,4) DEFAULT 0.0000 COMMENT '抽样数量',
  `qualified_quantity` decimal(12,4) DEFAULT 0.0000 COMMENT '合格数量',
  `unqualified_quantity` decimal(12,4) DEFAULT 0.0000 COMMENT '不合格数量',
  `unit` varchar(20) NOT NULL COMMENT '单位',
  `inspection_level` varchar(20) DEFAULT NULL COMMENT '检验水平：如GL-I, GL-II',
  `aql` decimal(5,2) DEFAULT NULL COMMENT 'AQL值',
  `sampling_plan` varchar(100) DEFAULT NULL COMMENT '抽样方案',
  `plan_start_time` datetime DEFAULT NULL COMMENT '计划开始时间',
  `plan_end_time` datetime DEFAULT NULL COMMENT '计划结束时间',
  `actual_start_time` datetime DEFAULT NULL COMMENT '实际开始时间',
  `actual_end_time` datetime DEFAULT NULL COMMENT '实际结束时间',
  `inspector_id` bigint(20) DEFAULT NULL COMMENT '检验员ID',
  `task_status` tinyint(1) DEFAULT 1 COMMENT '任务状态：1-待检验，2-检验中，3-已完成，4-已取消',
  `inspection_result` tinyint(1) DEFAULT NULL COMMENT '检验结果：1-合格，2-不合格，3-让步接收，4-待定',
  `is_urgent` tinyint(1) DEFAULT 0 COMMENT '是否加急：0-否，1-是',
  `priority` tinyint(1) DEFAULT 3 COMMENT '优先级：1-紧急，2-高，3-普通，4-低',
  `remark` varchar(500) DEFAULT NULL COMMENT '备注',
  `created_by` bigint(20) DEFAULT NULL COMMENT '创建人ID',
  `created_time` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `updated_by` bigint(20) DEFAULT NULL COMMENT '更新人ID',
  `updated_time` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  `is_deleted` tinyint(1) DEFAULT 0 COMMENT '是否删除：0-否，1-是',
  PRIMARY KEY (`id`),
  UNIQUE KEY `uk_task_no` (`task_no`),
  KEY `idx_inspection_type` (`inspection_type`),
  KEY `idx_source_order_no` (`source_order_no`),
  KEY `idx_material_id` (`material_id`),
  KEY `idx_batch_no` (`batch_no`),
  KEY `idx_task_status` (`task_status`),
  KEY `idx_plan_start_time` (`plan_start_time`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='质检任务表';

-- =============================================
-- 2. 质检报告主表 (quality_inspection_reports)
-- =============================================
DROP TABLE IF EXISTS `quality_inspection_reports`;
CREATE TABLE `quality_inspection_reports` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '报告ID',
  `report_no` varchar(50) NOT NULL COMMENT '报告编号',
  `task_id` bigint(20) NOT NULL COMMENT '质检任务ID',
  `inspection_type` tinyint(1) NOT NULL COMMENT '检验类型：1-IQC，2-IPQC，3-FQC，4-OQC，5-委外检验',
  `material_id` bigint(20) NOT NULL COMMENT '物料ID',
  `batch_no` varchar(50) DEFAULT NULL COMMENT '批次号',
  `supplier_id` bigint(20) DEFAULT NULL COMMENT '供应商ID',
  `production_order_id` bigint(20) DEFAULT NULL COMMENT '生产订单ID',
  `inspection_date` date NOT NULL COMMENT '检验日期',
  `inspection_time` datetime NOT NULL COMMENT '检验时间',
  `inspector_id` bigint(20) NOT NULL COMMENT '检验员ID',
  `reviewer_id` bigint(20) DEFAULT NULL COMMENT '审核人ID',
  `review_time` datetime DEFAULT NULL COMMENT '审核时间',
  `inspection_quantity` decimal(12,4) NOT NULL COMMENT '送检数量',
  `sample_quantity` decimal(12,4) NOT NULL COMMENT '抽样数量',
  `qualified_quantity` decimal(12,4) NOT NULL COMMENT '合格数量',
  `unqualified_quantity` decimal(12,4) NOT NULL COMMENT '不合格数量',
  `unit` varchar(20) NOT NULL COMMENT '单位',
  `qualified_rate` decimal(5,2) DEFAULT 0.00 COMMENT '合格率（%）',
  `inspection_result` tinyint(1) NOT NULL COMMENT '检验结果：1-合格，2-不合格，3-让步接收，4-待定',
  `disposition` tinyint(1) DEFAULT NULL COMMENT '处置方式：1-接收，2-退货，3-返工，4-报废，5-降级使用',
  `major_defects` int(11) DEFAULT 0 COMMENT '严重缺陷数',
  `minor_defects` int(11) DEFAULT 0 COMMENT '轻微缺陷数',
  `critical_defects` int(11) DEFAULT 0 COMMENT '致命缺陷数',
  `inspection_environment` varchar(200) DEFAULT NULL COMMENT '检验环境',
  `inspection_equipment` varchar(200) DEFAULT NULL COMMENT '检验设备',
  `report_status` tinyint(1) DEFAULT 1 COMMENT '报告状态：1-待审核，2-已审核，3-已归档',
  `conclusion` varchar(1000) DEFAULT NULL COMMENT '检验结论',
  `improvement_suggestions` varchar(1000) DEFAULT NULL COMMENT '改善建议',
  `attachment_url` varchar(500) DEFAULT NULL COMMENT '附件URL',
  `remark` varchar(500) DEFAULT NULL COMMENT '备注',
  `created_by` bigint(20) DEFAULT NULL COMMENT '创建人ID',
  `created_time` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `updated_by` bigint(20) DEFAULT NULL COMMENT '更新人ID',
  `updated_time` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  `is_deleted` tinyint(1) DEFAULT 0 COMMENT '是否删除：0-否，1-是',
  PRIMARY KEY (`id`),
  UNIQUE KEY `uk_report_no` (`report_no`),
  KEY `idx_task_id` (`task_id`),
  KEY `idx_material_id` (`material_id`),
  KEY `idx_batch_no` (`batch_no`),
  KEY `idx_inspection_date` (`inspection_date`),
  KEY `idx_inspection_result` (`inspection_result`),
  KEY `idx_report_status` (`report_status`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='质检报告主表';

-- =============================================
-- 3. 质检项目明细表 (quality_inspection_items)
-- =============================================
DROP TABLE IF EXISTS `quality_inspection_items`;
CREATE TABLE `quality_inspection_items` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '明细ID',
  `report_id` bigint(20) NOT NULL COMMENT '质检报告ID',
  `item_code` varchar(50) NOT NULL COMMENT '检验项目编码',
  `item_name` varchar(200) NOT NULL COMMENT '检验项目名称',
  `item_type` tinyint(1) NOT NULL COMMENT '项目类型：1-尺寸，2-外观，3-性能，4-功能，5-其他',
  `inspection_method` varchar(200) DEFAULT NULL COMMENT '检验方法',
  `standard_value` varchar(100) DEFAULT NULL COMMENT '标准值',
  `upper_limit` decimal(12,4) DEFAULT NULL COMMENT '上限',
  `lower_limit` decimal(12,4) DEFAULT NULL COMMENT '下限',
  `actual_value` varchar(100) DEFAULT NULL COMMENT '实测值',
  `unit` varchar(20) DEFAULT NULL COMMENT '单位',
  `inspection_equipment` varchar(100) DEFAULT NULL COMMENT '检验设备',
  `item_result` tinyint(1) NOT NULL COMMENT '检验结果：1-合格，2-不合格',
  `defect_quantity` int(11) DEFAULT 0 COMMENT '缺陷数量',
  `defect_code` varchar(50) DEFAULT NULL COMMENT '缺陷代码',
  `defect_level` tinyint(1) DEFAULT NULL COMMENT '缺陷等级：1-致命，2-严重，3-一般，4-轻微',
  `is_key_item` tinyint(1) DEFAULT 0 COMMENT '是否关键项：0-否，1-是',
  `sequence_no` int(11) DEFAULT 0 COMMENT '序号',
  `remark` varchar(500) DEFAULT NULL COMMENT '备注',
  `created_by` bigint(20) DEFAULT NULL COMMENT '创建人ID',
  `created_time` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `updated_by` bigint(20) DEFAULT NULL COMMENT '更新人ID',
  `updated_time` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  `is_deleted` tinyint(1) DEFAULT 0 COMMENT '是否删除：0-否，1-是',
  PRIMARY KEY (`id`),
  KEY `idx_report_id` (`report_id`),
  KEY `idx_item_code` (`item_code`),
  KEY `idx_item_result` (`item_result`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='质检项目明细表';

-- =============================================
-- 4. 不合格品记录表 (nonconforming_products)
-- =============================================
DROP TABLE IF EXISTS `nonconforming_products`;
CREATE TABLE `nonconforming_products` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '记录ID',
  `ncr_no` varchar(50) NOT NULL COMMENT '不合格品编号（NCR）',
  `report_id` bigint(20) DEFAULT NULL COMMENT '质检报告ID',
  `source_type` tinyint(1) NOT NULL COMMENT '来源类型：1-来料检验，2-过程检验，3-成品检验，4-客户退货，5-其他',
  `source_order_no` varchar(50) DEFAULT NULL COMMENT '来源单号',
  `material_id` bigint(20) NOT NULL COMMENT '物料ID',
  `batch_no` varchar(50) DEFAULT NULL COMMENT '批次号',
  `serial_no` varchar(50) DEFAULT NULL COMMENT '序列号',
  `supplier_id` bigint(20) DEFAULT NULL COMMENT '供应商ID',
  `customer_id` bigint(20) DEFAULT NULL COMMENT '客户ID',
  `production_order_id` bigint(20) DEFAULT NULL COMMENT '生产订单ID',
  `work_order_id` bigint(20) DEFAULT NULL COMMENT '工单ID',
  `process_id` bigint(20) DEFAULT NULL COMMENT '工序ID',
  `defect_quantity` decimal(12,4) NOT NULL COMMENT '不合格数量',
  `unit` varchar(20) NOT NULL COMMENT '单位',
  `defect_code` varchar(50) DEFAULT NULL COMMENT '不良代码',
  `defect_name` varchar(200) DEFAULT NULL COMMENT '不良名称',
  `defect_level` tinyint(1) DEFAULT 3 COMMENT '缺陷等级：1-致命，2-严重，3-一般，4-轻微',
  `defect_description` varchar(1000) DEFAULT NULL COMMENT '缺陷描述',
  `defect_location` varchar(200) DEFAULT NULL COMMENT '缺陷位置',
  `defect_images` varchar(1000) DEFAULT NULL COMMENT '缺陷图片（JSON格式）',
  `found_date` date NOT NULL COMMENT '发现日期',
  `found_time` datetime NOT NULL COMMENT '发现时间',
  `finder_id` bigint(20) NOT NULL COMMENT '发现人ID',
  `responsible_dept_id` bigint(20) DEFAULT NULL COMMENT '责任部门ID',
  `responsible_person_id` bigint(20) DEFAULT NULL COMMENT '责任人ID',
  `root_cause` varchar(1000) DEFAULT NULL COMMENT '根本原因',
  `disposition` tinyint(1) DEFAULT NULL COMMENT '处置方式：1-返工，2-报废，3-让步接收，4-退货，5-降级使用，6-挑选',
  `disposition_quantity` decimal(12,4) DEFAULT 0.0000 COMMENT '处置数量',
  `disposition_date` date DEFAULT NULL COMMENT '处置日期',
  `disposition_handler_id` bigint(20) DEFAULT NULL COMMENT '处置人ID',
  `disposition_result` varchar(500) DEFAULT NULL COMMENT '处置结果',
  `rework_order_no` varchar(50) DEFAULT NULL COMMENT '返工单号',
  `corrective_action` varchar(1000) DEFAULT NULL COMMENT '纠正措施',
  `preventive_action` varchar(1000) DEFAULT NULL COMMENT '预防措施',
  `ncr_status` tinyint(1) DEFAULT 1 COMMENT 'NCR状态：1-待处置，2-处置中，3-已处置，4-已验证，5-已关闭',
  `is_repetitive` tinyint(1) DEFAULT 0 COMMENT '是否重复发生：0-否，1-是',
  `closure_date` date DEFAULT NULL COMMENT '关闭日期',
  `remark` varchar(500) DEFAULT NULL COMMENT '备注',
  `created_by` bigint(20) DEFAULT NULL COMMENT '创建人ID',
  `created_time` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `updated_by` bigint(20) DEFAULT NULL COMMENT '更新人ID',
  `updated_time` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  `is_deleted` tinyint(1) DEFAULT 0 COMMENT '是否删除：0-否，1-是',
  PRIMARY KEY (`id`),
  UNIQUE KEY `uk_ncr_no` (`ncr_no`),
  KEY `idx_report_id` (`report_id`),
  KEY `idx_material_id` (`material_id`),
  KEY `idx_batch_no` (`batch_no`),
  KEY `idx_supplier_id` (`supplier_id`),
  KEY `idx_ncr_status` (`ncr_status`),
  KEY `idx_found_date` (`found_date`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='不合格品记录表';

-- =============================================
-- 5. 返工单表 (rework_orders)
-- =============================================
DROP TABLE IF EXISTS `rework_orders`;
CREATE TABLE `rework_orders` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '返工单ID',
  `rework_no` varchar(50) NOT NULL COMMENT '返工单号',
  `ncr_id` bigint(20) NOT NULL COMMENT '不合格品记录ID',
  `material_id` bigint(20) NOT NULL COMMENT '物料ID',
  `batch_no` varchar(50) DEFAULT NULL COMMENT '批次号',
  `rework_quantity` decimal(12,4) NOT NULL COMMENT '返工数量',
  `completed_quantity` decimal(12,4) DEFAULT 0.0000 COMMENT '已完成数量',
  `qualified_quantity` decimal(12,4) DEFAULT 0.0000 COMMENT '返工合格数量',
  `scrap_quantity` decimal(12,4) DEFAULT 0.0000 COMMENT '报废数量',
  `unit` varchar(20) NOT NULL COMMENT '单位',
  `rework_type` tinyint(1) NOT NULL COMMENT '返工类型：1-工序返工，2-全检挑选，3-返修，4-其他',
  `rework_reason` varchar(500) NOT NULL COMMENT '返工原因',
  `rework_plan` varchar(1000) DEFAULT NULL COMMENT '返工方案',
  `rework_process` varchar(500) DEFAULT NULL COMMENT '返工工序',
  `workshop_id` bigint(20) DEFAULT NULL COMMENT '返工车间ID',
  `plan_start_date` date DEFAULT NULL COMMENT '计划开始日期',
  `plan_end_date` date DEFAULT NULL COMMENT '计划完成日期',
  `actual_start_date` date DEFAULT NULL COMMENT '实际开始日期',
  `actual_end_date` date DEFAULT NULL COMMENT '实际完成日期',
  `handler_id` bigint(20) DEFAULT NULL COMMENT '负责人ID',
  `rework_cost` decimal(15,2) DEFAULT 0.00 COMMENT '返工成本',
  `rework_status` tinyint(1) DEFAULT 1 COMMENT '返工状态：1-待返工，2-返工中，3-已完成，4-已取消',
  `inspection_result` tinyint(1) DEFAULT NULL COMMENT '返工后检验结果：1-合格，2-不合格',
  `inspector_id` bigint(20) DEFAULT NULL COMMENT '检验员ID',
  `inspection_time` datetime DEFAULT NULL COMMENT '检验时间',
  `remark` varchar(500) DEFAULT NULL COMMENT '备注',
  `created_by` bigint(20) DEFAULT NULL COMMENT '创建人ID',
  `created_time` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `updated_by` bigint(20) DEFAULT NULL COMMENT '更新人ID',
  `updated_time` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  `is_deleted` tinyint(1) DEFAULT 0 COMMENT '是否删除：0-否，1-是',
  PRIMARY KEY (`id`),
  UNIQUE KEY `uk_rework_no` (`rework_no`),
  KEY `idx_ncr_id` (`ncr_id`),
  KEY `idx_material_id` (`material_id`),
  KEY `idx_batch_no` (`batch_no`),
  KEY `idx_rework_status` (`rework_status`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='返工单表';

-- =============================================
-- 6. 客户投诉表 (customer_complaints)
-- =============================================
DROP TABLE IF EXISTS `customer_complaints`;
CREATE TABLE `customer_complaints` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '投诉ID',
  `complaint_no` varchar(50) NOT NULL COMMENT '投诉编号',
  `customer_id` bigint(20) NOT NULL COMMENT '客户ID',
  `material_id` bigint(20) NOT NULL COMMENT '物料ID',
  `batch_no` varchar(50) DEFAULT NULL COMMENT '批次号',
  `sales_order_no` varchar(50) DEFAULT NULL COMMENT '销售订单号',
  `production_order_no` varchar(50) DEFAULT NULL COMMENT '生产订单号',
  `complaint_type` tinyint(1) NOT NULL COMMENT '投诉类型：1-质量问题，2-交期问题，3-服务问题，4-包装问题，5-其他',
  `complaint_level` tinyint(1) DEFAULT 3 COMMENT '投诉级别：1-严重，2-重要，3-一般',
  `complaint_date` date NOT NULL COMMENT '投诉日期',
  `complaint_time` datetime NOT NULL COMMENT '投诉时间',
  `complaint_quantity` decimal(12,4) DEFAULT NULL COMMENT '投诉数量',
  `unit` varchar(20) DEFAULT NULL COMMENT '单位',
  `complaint_description` varchar(2000) NOT NULL COMMENT '投诉描述',
  `defect_description` varchar(1000) DEFAULT NULL COMMENT '缺陷描述',
  `defect_images` varchar(1000) DEFAULT NULL COMMENT '缺陷图片（JSON格式）',
  `customer_requirement` varchar(500) DEFAULT NULL COMMENT '客户要求',
  `receiver_id` bigint(20) NOT NULL COMMENT '接收人ID',
  `handler_id` bigint(20) DEFAULT NULL COMMENT '处理人ID',
  `response_deadline` date DEFAULT NULL COMMENT '回复期限',
  `response_time` datetime DEFAULT NULL COMMENT '回复时间',
  `response_content` varchar(2000) DEFAULT NULL COMMENT '回复内容',
  `root_cause_analysis` varchar(1000) DEFAULT NULL COMMENT '根本原因分析',
  `corrective_action` varchar(1000) DEFAULT NULL COMMENT '纠正措施',
  `preventive_action` varchar(1000) DEFAULT NULL COMMENT '预防措施',
  `compensation_amount` decimal(15,2) DEFAULT 0.00 COMMENT '赔偿金额',
  `processing_cost` decimal(15,2) DEFAULT 0.00 COMMENT '处理成本',
  `complaint_status` tinyint(1) DEFAULT 1 COMMENT '投诉状态：1-待处理，2-处理中，3-待验证，4-已关闭',
  `is_valid` tinyint(1) DEFAULT 1 COMMENT '是否有效投诉：0-无效，1-有效',
  `customer_satisfaction` tinyint(1) DEFAULT NULL COMMENT '客户满意度：1-非常满意，2-满意，3-一般，4-不满意，5-非常不满意',
  `closure_date` date DEFAULT NULL COMMENT '关闭日期',
  `remark` varchar(500) DEFAULT NULL COMMENT '备注',
  `created_by` bigint(20) DEFAULT NULL COMMENT '创建人ID',
  `created_time` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `updated_by` bigint(20) DEFAULT NULL COMMENT '更新人ID',
  `updated_time` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  `is_deleted` tinyint(1) DEFAULT 0 COMMENT '是否删除：0-否，1-是',
  PRIMARY KEY (`id`),
  UNIQUE KEY `uk_complaint_no` (`complaint_no`),
  KEY `idx_customer_id` (`customer_id`),
  KEY `idx_material_id` (`material_id`),
  KEY `idx_batch_no` (`batch_no`),
  KEY `idx_complaint_status` (`complaint_status`),
  KEY `idx_complaint_date` (`complaint_date`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='客户投诉表';

-- =============================================
-- 7. 供应商质量评估表 (supplier_quality_evaluations)
-- =============================================
DROP TABLE IF EXISTS `supplier_quality_evaluations`;
CREATE TABLE `supplier_quality_evaluations` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '评估ID',
  `evaluation_no` varchar(50) NOT NULL COMMENT '评估编号',
  `supplier_id` bigint(20) NOT NULL COMMENT '供应商ID',
  `evaluation_period` varchar(50) NOT NULL COMMENT '评估周期：如2025-01',
  `evaluation_date` date NOT NULL COMMENT '评估日期',
  `evaluator_id` bigint(20) NOT NULL COMMENT '评估人ID',
  `total_receipts` int(11) DEFAULT 0 COMMENT '总收货批次',
  `total_quantity` decimal(12,4) DEFAULT 0.0000 COMMENT '总收货数量',
  `qualified_receipts` int(11) DEFAULT 0 COMMENT '合格批次',
  `qualified_quantity` decimal(12,4) DEFAULT 0.0000 COMMENT '合格数量',
  `unqualified_receipts` int(11) DEFAULT 0 COMMENT '不合格批次',
  `unqualified_quantity` decimal(12,4) DEFAULT 0.0000 COMMENT '不合格数量',
  `batch_qualified_rate` decimal(5,2) DEFAULT 0.00 COMMENT '批次合格率（%）',
  `quantity_qualified_rate` decimal(5,2) DEFAULT 0.00 COMMENT '数量合格率（%）',
  `on_time_delivery_rate` decimal(5,2) DEFAULT 0.00 COMMENT '准时交付率（%）',
  `response_speed_score` decimal(5,2) DEFAULT 0.00 COMMENT '响应速度评分',
  `service_attitude_score` decimal(5,2) DEFAULT 0.00 COMMENT '服务态度评分',
  `quality_score` decimal(5,2) DEFAULT 0.00 COMMENT '质量得分',
  `delivery_score` decimal(5,2) DEFAULT 0.00 COMMENT '交期得分',
  `service_score` decimal(5,2) DEFAULT 0.00 COMMENT '服务得分',
  `total_score` decimal(5,2) DEFAULT 0.00 COMMENT '总分',
  `evaluation_level` char(1) DEFAULT 'C' COMMENT '评估等级：A-优秀，B-良好，C-合格，D-不合格',
  `major_issues` varchar(1000) DEFAULT NULL COMMENT '主要问题',
  `improvement_requirements` varchar(1000) DEFAULT NULL COMMENT '改进要求',
  `evaluation_conclusion` varchar(1000) DEFAULT NULL COMMENT '评估结论',
  `is_approved` tinyint(1) DEFAULT 0 COMMENT '是否审批：0-未审批，1-已审批',
  `approver_id` bigint(20) DEFAULT NULL COMMENT '审批人ID',
  `approval_time` datetime DEFAULT NULL COMMENT '审批时间',
  `remark` varchar(500) DEFAULT NULL COMMENT '备注',
  `created_by` bigint(20) DEFAULT NULL COMMENT '创建人ID',
  `created_time` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `updated_by` bigint(20) DEFAULT NULL COMMENT '更新人ID',
  `updated_time` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  `is_deleted` tinyint(1) DEFAULT 0 COMMENT '是否删除：0-否，1-是',
  PRIMARY KEY (`id`),
  UNIQUE KEY `uk_evaluation_no` (`evaluation_no`),
  KEY `idx_supplier_id` (`supplier_id`),
  KEY `idx_evaluation_period` (`evaluation_period`),
  KEY `idx_evaluation_date` (`evaluation_date`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='供应商质量评估表';

-- =============================================
-- 8. 质量追溯记录表 (quality_traceability_records)
-- =============================================
DROP TABLE IF EXISTS `quality_traceability_records`;
CREATE TABLE `quality_traceability_records` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '追溯ID',
  `trace_no` varchar(50) NOT NULL COMMENT '追溯编号',
  `trace_type` tinyint(1) NOT NULL COMMENT '追溯类型：1-正向追溯，2-反向追溯',
  `material_id` bigint(20) NOT NULL COMMENT '物料ID',
  `batch_no` varchar(50) NOT NULL COMMENT '批次号',
  `serial_no` varchar(50) DEFAULT NULL COMMENT '序列号',
  `production_order_no` varchar(50) DEFAULT NULL COMMENT '生产订单号',
  `sales_order_no` varchar(50) DEFAULT NULL COMMENT '销售订单号',
  `customer_id` bigint(20) DEFAULT NULL COMMENT '客户ID',
  `supplier_id` bigint(20) DEFAULT NULL COMMENT '供应商ID',
  `supplier_batch_no` varchar(50) DEFAULT NULL COMMENT '供应商批次号',
  `production_date` date DEFAULT NULL COMMENT '生产日期',
  `inspection_report_no` varchar(50) DEFAULT NULL COMMENT '检验报告号',
  `inspection_result` tinyint(1) DEFAULT NULL COMMENT '检验结果：1-合格，2-不合格',
  `workshop_id` bigint(20) DEFAULT NULL COMMENT '生产车间ID',
  `production_line` varchar(100) DEFAULT NULL COMMENT '生产线',
  `operator_ids` varchar(500) DEFAULT NULL COMMENT '操作员ID列表（JSON格式）',
  `equipment_ids` varchar(500) DEFAULT NULL COMMENT '设备ID列表（JSON格式）',
  `raw_material_info` text COMMENT '原材料信息（JSON格式）',
  `process_info` text COMMENT '工序信息（JSON格式）',
  `quality_info` text COMMENT '质量信息（JSON格式）',
  `trace_reason` varchar(500) DEFAULT NULL COMMENT '追溯原因',
  `trace_result` varchar(1000) DEFAULT NULL COMMENT '追溯结果',
  `trace_date` date NOT NULL COMMENT '追溯日期',
  `tracer_id` bigint(20) NOT NULL COMMENT '追溯人ID',
  `remark` varchar(500) DEFAULT NULL COMMENT '备注',
  `created_by` bigint(20) DEFAULT NULL COMMENT '创建人ID',
  `created_time` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `updated_by` bigint(20) DEFAULT NULL COMMENT '更新人ID',
  `updated_time` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  `is_deleted` tinyint(1) DEFAULT 0 COMMENT '是否删除：0-否，1-是',
  PRIMARY KEY (`id`),
  UNIQUE KEY `uk_trace_no` (`trace_no`),
  KEY `idx_material_id` (`material_id`),
  KEY `idx_batch_no` (`batch_no`),
  KEY `idx_serial_no` (`serial_no`),
  KEY `idx_production_order_no` (`production_order_no`),
  KEY `idx_trace_date` (`trace_date`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='质量追溯记录表';

-- =============================================
-- 9. 测量设备管理表 (measuring_equipment)
-- =============================================
DROP TABLE IF EXISTS `measuring_equipment`;
CREATE TABLE `measuring_equipment` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '设备ID',
  `equipment_code` varchar(50) NOT NULL COMMENT '设备编码',
  `equipment_name` varchar(200) NOT NULL COMMENT '设备名称',
  `equipment_model` varchar(100) DEFAULT NULL COMMENT '设备型号',
  `equipment_type` tinyint(1) NOT NULL COMMENT '设备类型：1-量具，2-仪器，3-检测设备，4-其他',
  `manufacturer` varchar(200) DEFAULT NULL COMMENT '制造商',
  `serial_no` varchar(100) DEFAULT NULL COMMENT '出厂编号',
  `purchase_date` date DEFAULT NULL COMMENT '购置日期',
  `accuracy_level` varchar(50) DEFAULT NULL COMMENT '精度等级',
  `measurement_range` varchar(100) DEFAULT NULL COMMENT '测量范围',
  `calibration_cycle` int(11) DEFAULT 365 COMMENT '校准周期（天）',
  `last_calibration_date` date DEFAULT NULL COMMENT '上次校准日期',
  `next_calibration_date` date DEFAULT NULL COMMENT '下次校准日期',
  `calibration_institution` varchar(200) DEFAULT NULL COMMENT '校准机构',
  `calibration_certificate_no` varchar(100) DEFAULT NULL COMMENT '校准证书号',
  `equipment_status` tinyint(1) DEFAULT 1 COMMENT '设备状态：1-正常，2-待校准，3-校准中，4-停用，5-报废',
  `location` varchar(200) DEFAULT NULL COMMENT '存放位置',
  `custodian_id` bigint(20) DEFAULT NULL COMMENT '保管人ID',
  `usage_frequency` varchar(50) DEFAULT NULL COMMENT '使用频率',
  `maintenance_requirements` varchar(500) DEFAULT NULL COMMENT '维护要求',
  `image_url` varchar(500) DEFAULT NULL COMMENT '设备图片URL',
  `remark` varchar(500) DEFAULT NULL COMMENT '备注',
  `created_by` bigint(20) DEFAULT NULL COMMENT '创建人ID',
  `created_time` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `updated_by` bigint(20) DEFAULT NULL COMMENT '更新人ID',
  `updated_time` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  `is_deleted` tinyint(1) DEFAULT 0 COMMENT '是否删除：0-否，1-是',
  PRIMARY KEY (`id`),
  UNIQUE KEY `uk_equipment_code` (`equipment_code`),
  KEY `idx_equipment_type` (`equipment_type`),
  KEY `idx_equipment_status` (`equipment_status`),
  KEY `idx_next_calibration_date` (`next_calibration_date`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='测量设备管理表';

-- =============================================
-- 10. 测量设备校准记录表 (equipment_calibration_records)
-- =============================================
DROP TABLE IF EXISTS `equipment_calibration_records`;
CREATE TABLE `equipment_calibration_records` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '记录ID',
  `record_no` varchar(50) NOT NULL COMMENT '记录编号',
  `equipment_id` bigint(20) NOT NULL COMMENT '设备ID',
  `calibration_type` tinyint(1) NOT NULL COMMENT '校准类型：1-内部校准，2-外部校准',
  `calibration_date` date NOT NULL COMMENT '校准日期',
  `calibration_institution` varchar(200) DEFAULT NULL COMMENT '校准机构',
  `calibrator_id` bigint(20) DEFAULT NULL COMMENT '校准人ID',
  `calibration_standard` varchar(200) DEFAULT NULL COMMENT '校准依据',
  `calibration_result` tinyint(1) NOT NULL COMMENT '校准结果：1-合格，2-不合格',
  `certificate_no` varchar(100) DEFAULT NULL COMMENT '证书编号',
  `certificate_valid_date` date DEFAULT NULL COMMENT '证书有效期',
  `next_calibration_date` date DEFAULT NULL COMMENT '下次校准日期',
  `calibration_cost` decimal(15,2) DEFAULT 0.00 COMMENT '校准费用',
  `deviation_before` varchar(200) DEFAULT NULL COMMENT '校准前偏差',
  `deviation_after` varchar(200) DEFAULT NULL COMMENT '校准后偏差',
  `adjustment_content` varchar(500) DEFAULT NULL COMMENT '调整内容',
  `certificate_url` varchar(500) DEFAULT NULL COMMENT '证书URL',
  `remark` varchar(500) DEFAULT NULL COMMENT '备注',
  `created_by` bigint(20) DEFAULT NULL COMMENT '创建人ID',
  `created_time` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `updated_by` bigint(20) DEFAULT NULL COMMENT '更新人ID',
  `updated_time` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  `is_deleted` tinyint(1) DEFAULT 0 COMMENT '是否删除：0-否，1-是',
  PRIMARY KEY (`id`),
  UNIQUE KEY `uk_record_no` (`record_no`),
  KEY `idx_equipment_id` (`equipment_id`),
  KEY `idx_calibration_date` (`calibration_date`),
  KEY `idx_calibration_result` (`calibration_result`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='测量设备校准记录表';

-- =============================================
-- 11. 质量成本表 (quality_costs)
-- =============================================
DROP TABLE IF EXISTS `quality_costs`;
CREATE TABLE `quality_costs` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '成本ID',
  `cost_no` varchar(50) NOT NULL COMMENT '成本单号',
  `cost_period` varchar(50) NOT NULL COMMENT '成本期间：如2025-01',
  `cost_date` date NOT NULL COMMENT '成本日期',
  `cost_category` tinyint(1) NOT NULL COMMENT '成本类别：1-预防成本，2-鉴定成本，3-内部失败成本，4-外部失败成本',
  `cost_type` varchar(100) NOT NULL COMMENT '成本类型',
  `cost_item` varchar(200) NOT NULL COMMENT '成本项目',
  `material_id` bigint(20) DEFAULT NULL COMMENT '物料ID',
  `production_order_id` bigint(20) DEFAULT NULL COMMENT '生产订单ID',
  `ncr_id` bigint(20) DEFAULT NULL COMMENT '不合格品记录ID',
  `complaint_id` bigint(20) DEFAULT NULL COMMENT '客户投诉ID',
  `cost_amount` decimal(15,2) NOT NULL COMMENT '成本金额',
  `quantity` decimal(12,4) DEFAULT NULL COMMENT '数量',
  `unit` varchar(20) DEFAULT NULL COMMENT '单位',
  `dept_id` bigint(20) DEFAULT NULL COMMENT '部门ID',
  `cost_description` varchar(500) DEFAULT NULL COMMENT '成本描述',
  `handler_id` bigint(20) DEFAULT NULL COMMENT '经办人ID',
  `remark` varchar(500) DEFAULT NULL COMMENT '备注',
  `created_by` bigint(20) DEFAULT NULL COMMENT '创建人ID',
  `created_time` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `updated_by` bigint(20) DEFAULT NULL COMMENT '更新人ID',
  `updated_time` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  `is_deleted` tinyint(1) DEFAULT 0 COMMENT '是否删除：0-否，1-是',
  PRIMARY KEY (`id`),
  UNIQUE KEY `uk_cost_no` (`cost_no`),
  KEY `idx_cost_period` (`cost_period`),
  KEY `idx_cost_category` (`cost_category`),
  KEY `idx_cost_date` (`cost_date`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='质量成本表';

-- =============================================
-- 12. 质量KPI指标表 (quality_kpi)
-- =============================================
DROP TABLE IF EXISTS `quality_kpi`;
CREATE TABLE `quality_kpi` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT 'KPI ID',
  `kpi_date` date NOT NULL COMMENT 'KPI日期',
  `kpi_type` tinyint(1) NOT NULL COMMENT 'KPI类型：1-日报，2-周报，3-月报',
  `dept_id` bigint(20) DEFAULT NULL COMMENT '部门ID',
  `workshop_id` bigint(20) DEFAULT NULL COMMENT '车间ID',
  `total_inspections` int(11) DEFAULT 0 COMMENT '总检验批次',
  `qualified_inspections` int(11) DEFAULT 0 COMMENT '合格批次',
  `unqualified_inspections` int(11) DEFAULT 0 COMMENT '不合格批次',
  `inspection_quantity` decimal(12,4) DEFAULT 0.0000 COMMENT '检验数量',
  `qualified_quantity` decimal(12,4) DEFAULT 0.0000 COMMENT '合格数量',
  `unqualified_quantity` decimal(12,4) DEFAULT 0.0000 COMMENT '不合格数量',
  `batch_qualified_rate` decimal(5,2) DEFAULT 0.00 COMMENT '批次合格率（%）',
  `quantity_qualified_rate` decimal(5,2) DEFAULT 0.00 COMMENT '数量合格率（%）',
  `first_pass_yield` decimal(5,2) DEFAULT 0.00 COMMENT '一次合格率（%）',
  `iqc_qualified_rate` decimal(5,2) DEFAULT 0.00 COMMENT 'IQC合格率（%）',
  `ipqc_qualified_rate` decimal(5,2) DEFAULT 0.00 COMMENT 'IPQC合格率（%）',
  `fqc_qualified_rate` decimal(5,2) DEFAULT 0.00 COMMENT 'FQC合格率（%）',
  `oqc_qualified_rate` decimal(5,2) DEFAULT 0.00 COMMENT 'OQC合格率（%）',
  `rework_quantity` decimal(12,4) DEFAULT 0.0000 COMMENT '返工数量',
  `scrap_quantity` decimal(12,4) DEFAULT 0.0000 COMMENT '报废数量',
  `rework_rate` decimal(5,2) DEFAULT 0.00 COMMENT '返工率（%）',
  `scrap_rate` decimal(5,2) DEFAULT 0.00 COMMENT '报废率（%）',
  `customer_complaints` int(11) DEFAULT 0 COMMENT '客户投诉次数',
  `valid_complaints` int(11) DEFAULT 0 COMMENT '有效投诉次数',
  `complaint_rate` decimal(5,2) DEFAULT 0.00 COMMENT '投诉率（PPM）',
  `ncr_count` int(11) DEFAULT 0 COMMENT 'NCR数量',
  `major_ncr_count` int(11) DEFAULT 0 COMMENT '重大NCR数量',
  `preventive_cost` decimal(15,2) DEFAULT 0.00 COMMENT '预防成本',
  `appraisal_cost` decimal(15,2) DEFAULT 0.00 COMMENT '鉴定成本',
  `internal_failure_cost` decimal(15,2) DEFAULT 0.00 COMMENT '内部失败成本',
  `external_failure_cost` decimal(15,2) DEFAULT 0.00 COMMENT '外部失败成本',
  `total_quality_cost` decimal(15,2) DEFAULT 0.00 COMMENT '质量成本总额',
  `quality_cost_rate` decimal(5,2) DEFAULT 0.00 COMMENT '质量成本率（%）',
  `dppm` decimal(10,2) DEFAULT 0.00 COMMENT 'DPPM（百万件不良率）',
  `cpk` decimal(5,2) DEFAULT NULL COMMENT 'CPK值',
  `sigma_level` decimal(3,1) DEFAULT NULL COMMENT 'Sigma水平',
  `remark` varchar(500) DEFAULT NULL COMMENT '备注',
  `created_by` bigint(20) DEFAULT NULL COMMENT '创建人ID',
  `created_time` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `updated_by` bigint(20) DEFAULT NULL COMMENT '更新人ID',
  `updated_time` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  PRIMARY KEY (`id`),
  KEY `idx_kpi_date` (`kpi_date`),
  KEY `idx_kpi_type` (`kpi_type`),
  KEY `idx_dept_id` (`dept_id`),
  KEY `idx_workshop_id` (`workshop_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='质量KPI指标表';

SET FOREIGN_KEY_CHECKS = 1;

-- =============================================
-- 说明文档
-- =============================================
/*
MES系统质量管理模块表结构说明：

核心表说明：

1. quality_inspection_tasks（质检任务表）
   - 支持IQC、IPQC、FQC、OQC等多种检验类型
   - 任务分配和调度
   - 检验进度跟踪
   - 优先级管理

2. quality_inspection_reports（质检报告主表）
   - 详细的检验结果记录
   - 支持审核流程
   - 处置方式记录
   - 缺陷分类统计

3. quality_inspection_items（质检项目明细表）
   - 具体检验项目记录
   - 实测值与标准值对比
   - 缺陷等级判定
   - 关键项目标识

4. nonconforming_products（不合格品记录表）
   - NCR（不合格品报告）管理
   - 根本原因分析
   - 8D方法支持
   - 纠正和预防措施
   - 闭环管理

5. rework_orders（返工单表）
   - 返工计划和执行
   - 返工成本统计
   - 返工后检验
   - 返工效果追踪

6. customer_complaints（客户投诉表）
   - 客户投诉管理
   - 响应时效跟踪
   - 8D报告
   - 客户满意度评价

7. supplier_quality_evaluations（供应商质量评估表）
   - 定期供应商评估
   - 多维度评分体系
   - 等级评定
   - 改进跟踪

8. quality_traceability_records（质量追溯记录表）
   - 正向追溯（从原材料到成品）
   - 反向追溯（从成品到原材料）
   - 完整的质量信息链
   - 问题批次召回支持

9. measuring_equipment（测量设备管理表）
   - 量具、仪器管理
   - 精度等级管理
   - 校准周期管理
   - 设备状态跟踪

10. equipment_calibration_records（测量设备校准记录表）
    - 校准历史记录
    - 内部/外部校准
    - 证书管理
    - 校准结果分析

11. quality_costs（质量成本表）
    - PAF成本模型：
      * 预防成本（Prevention）
      * 鉴定成本（Appraisal）
      * 内部失败成本（Internal Failure）
      * 外部失败成本（External Failure）
    - 成本分析和控制

12. quality_kpi（质量KPI指标表）
    - 全面的质量指标体系
    - 合格率、一次通过率
    - DPPM（百万件不良率）
    - CPK、Sigma水平
    - 质量成本率

业务流程：

IQC来料检验流程：
1. 入库单触发检验任务
2. 分配检验员
3. 执行检验
4. 填写检验报告
5. 判定结果（合格/不合格/让步接收）
6. 不合格品处理（退货/返工等）

IPQC过程检验流程：
1. 工单触发检验任务
2. 首检/巡检/末检
3. 记录检验数据
4. 异常及时处理
5. SPC分析

FQC成品检验流程：
1. 完工单触发检验任务
2. 全检或抽样检验
3. 功能测试
4. 包装检验
5. 合格品放行入库

NCR处理流程（8D方法）：
D1: 成立小组
D2: 问题描述
D3: 临时措施
D4: 根本原因分析
D5: 选择纠正措施
D6: 实施纠正措施
D7: 预防措施
D8: 关闭NCR

客户投诉处理流程：
1. 接收投诉
2. 48小时响应
3. 原因分析
4. 制定措施
5. 实施验证
6. 客户反馈
7. 关闭投诉

质量指标说明：
- 合格率 = 合格数量 / 检验数量 × 100%
- 一次合格率 = 一次合格数量 / 总数量 × 100%
- DPPM = 不合格数 / 总数 × 1,000,000
- CPK = Min[(USL-μ)/3σ, (μ-LSL)/3σ]
- Sigma水平 = NORMSINV(1 - DPPM/1,000,000) + 1.5
- 质量成本率 = 质量成本 / 销售额 × 100%

数据特点：
1. 所有表都包含软删除字段
2. 所有表都包含完整的审计字段
3. 关键字段都建立了索引
4. 支持完整的质量追溯
5. 丰富的统计分析维度

使用建议：
1. 检验标准要提前维护好，确保检验依据明确
2. 检验数据要真实准确，为质量分析提供基础
3. NCR要及时处理，避免问题扩大
4. 定期进行供应商评估，提升供应链质量
5. 重视质量成本分析，优化质量管理投入
6. 测量设备要按时校准，确保检验结果准确
7. 建立完善的质量追溯体系，快速响应质量问题
8. 定期分析质量KPI，持续改进质量水平
*/
