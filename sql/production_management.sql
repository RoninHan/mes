-- =============================================
-- MES系统 - 生产管理模块数据库表结构
-- 创建日期: 2025-10-29
-- 数据库: MySQL 5.7+
-- =============================================

-- 设置字符集
SET NAMES utf8mb4;
SET FOREIGN_KEY_CHECKS = 0;

-- =============================================
-- 1. 生产计划主表 (production_plans)
-- =============================================
DROP TABLE IF EXISTS `production_plans`;
CREATE TABLE `production_plans` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '计划ID',
  `plan_no` varchar(50) NOT NULL COMMENT '计划编号',
  `plan_name` varchar(200) NOT NULL COMMENT '计划名称',
  `plan_type` tinyint(1) NOT NULL COMMENT '计划类型：1-月度计划，2-周计划，3-日计划，4-临时计划',
  `plan_period` varchar(50) DEFAULT NULL COMMENT '计划周期：如2025-01',
  `plan_start_date` date NOT NULL COMMENT '计划开始日期',
  `plan_end_date` date NOT NULL COMMENT '计划结束日期',
  `total_orders` int(11) DEFAULT 0 COMMENT '订单总数',
  `completed_orders` int(11) DEFAULT 0 COMMENT '已完成订单数',
  `total_quantity` decimal(12,4) DEFAULT 0.0000 COMMENT '计划总数量',
  `completed_quantity` decimal(12,4) DEFAULT 0.0000 COMMENT '已完成数量',
  `plan_status` tinyint(1) DEFAULT 1 COMMENT '计划状态：1-未开始，2-执行中，3-已完成，4-已取消',
  `completion_rate` decimal(5,2) DEFAULT 0.00 COMMENT '完成率（%）',
  `planner_id` bigint(20) DEFAULT NULL COMMENT '计划员ID',
  `dept_id` bigint(20) DEFAULT NULL COMMENT '部门ID',
  `remark` varchar(500) DEFAULT NULL COMMENT '备注',
  `created_by` bigint(20) DEFAULT NULL COMMENT '创建人ID',
  `created_time` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `updated_by` bigint(20) DEFAULT NULL COMMENT '更新人ID',
  `updated_time` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  `is_deleted` tinyint(1) DEFAULT 0 COMMENT '是否删除：0-否，1-是',
  PRIMARY KEY (`id`),
  UNIQUE KEY `uk_plan_no` (`plan_no`),
  KEY `idx_plan_type` (`plan_type`),
  KEY `idx_plan_status` (`plan_status`),
  KEY `idx_plan_period` (`plan_period`),
  KEY `idx_start_date` (`plan_start_date`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='生产计划主表';

-- =============================================
-- 2. 生产订单主表 (production_orders)
-- =============================================
DROP TABLE IF EXISTS `production_orders`;
CREATE TABLE `production_orders` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '生产订单ID',
  `order_no` varchar(50) NOT NULL COMMENT '生产订单号',
  `plan_id` bigint(20) DEFAULT NULL COMMENT '生产计划ID',
  `source_type` tinyint(1) NOT NULL COMMENT '来源类型：1-销售订单，2-库存补货，3-预测生产，4-其他',
  `source_order_no` varchar(50) DEFAULT NULL COMMENT '来源单号',
  `material_id` bigint(20) NOT NULL COMMENT '生产物料ID',
  `bom_id` bigint(20) DEFAULT NULL COMMENT 'BOM ID',
  `routing_id` bigint(20) DEFAULT NULL COMMENT '工艺路线ID',
  `plan_quantity` decimal(12,4) NOT NULL COMMENT '计划生产数量',
  `actual_quantity` decimal(12,4) DEFAULT 0.0000 COMMENT '实际生产数量',
  `qualified_quantity` decimal(12,4) DEFAULT 0.0000 COMMENT '合格数量',
  `unqualified_quantity` decimal(12,4) DEFAULT 0.0000 COMMENT '不合格数量',
  `scrap_quantity` decimal(12,4) DEFAULT 0.0000 COMMENT '报废数量',
  `unit` varchar(20) NOT NULL COMMENT '单位',
  `priority` tinyint(1) DEFAULT 3 COMMENT '优先级：1-紧急，2-高，3-普通，4-低',
  `plan_start_date` date NOT NULL COMMENT '计划开始日期',
  `plan_end_date` date NOT NULL COMMENT '计划完成日期',
  `actual_start_date` date DEFAULT NULL COMMENT '实际开始日期',
  `actual_end_date` date DEFAULT NULL COMMENT '实际完成日期',
  `workshop_id` bigint(20) DEFAULT NULL COMMENT '生产车间ID',
  `production_line` varchar(100) DEFAULT NULL COMMENT '生产线',
  `batch_no` varchar(50) DEFAULT NULL COMMENT '生产批次号',
  `customer_id` bigint(20) DEFAULT NULL COMMENT '客户ID',
  `order_status` tinyint(1) DEFAULT 1 COMMENT '订单状态：1-待发布，2-已发布，3-已下达，4-生产中，5-已完工，6-已入库，7-已取消',
  `is_locked` tinyint(1) DEFAULT 0 COMMENT '是否锁定：0-否，1-是',
  `is_urgent` tinyint(1) DEFAULT 0 COMMENT '是否加急：0-否，1-是',
  `standard_hours` decimal(10,2) DEFAULT 0.00 COMMENT '标准工时（小时）',
  `actual_hours` decimal(10,2) DEFAULT 0.00 COMMENT '实际工时（小时）',
  `leader_id` bigint(20) DEFAULT NULL COMMENT '生产负责人ID',
  `planner_id` bigint(20) DEFAULT NULL COMMENT '计划员ID',
  `remark` varchar(500) DEFAULT NULL COMMENT '备注',
  `created_by` bigint(20) DEFAULT NULL COMMENT '创建人ID',
  `created_time` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `updated_by` bigint(20) DEFAULT NULL COMMENT '更新人ID',
  `updated_time` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  `is_deleted` tinyint(1) DEFAULT 0 COMMENT '是否删除：0-否，1-是',
  PRIMARY KEY (`id`),
  UNIQUE KEY `uk_order_no` (`order_no`),
  KEY `idx_plan_id` (`plan_id`),
  KEY `idx_material_id` (`material_id`),
  KEY `idx_workshop_id` (`workshop_id`),
  KEY `idx_order_status` (`order_status`),
  KEY `idx_plan_start_date` (`plan_start_date`),
  KEY `idx_batch_no` (`batch_no`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='生产订单主表';

-- =============================================
-- 3. 生产工单主表 (work_orders)
-- =============================================
DROP TABLE IF EXISTS `work_orders`;
CREATE TABLE `work_orders` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '工单ID',
  `work_order_no` varchar(50) NOT NULL COMMENT '工单号',
  `production_order_id` bigint(20) NOT NULL COMMENT '生产订单ID',
  `process_id` bigint(20) NOT NULL COMMENT '工序ID',
  `sequence_no` int(11) NOT NULL COMMENT '工序顺序号',
  `material_id` bigint(20) NOT NULL COMMENT '物料ID',
  `plan_quantity` decimal(12,4) NOT NULL COMMENT '计划数量',
  `actual_quantity` decimal(12,4) DEFAULT 0.0000 COMMENT '实际完成数量',
  `qualified_quantity` decimal(12,4) DEFAULT 0.0000 COMMENT '合格数量',
  `unqualified_quantity` decimal(12,4) DEFAULT 0.0000 COMMENT '不合格数量',
  `scrap_quantity` decimal(12,4) DEFAULT 0.0000 COMMENT '报废数量',
  `unit` varchar(20) NOT NULL COMMENT '单位',
  `workshop_id` bigint(20) DEFAULT NULL COMMENT '车间ID',
  `equipment_id` bigint(20) DEFAULT NULL COMMENT '设备ID',
  `plan_start_time` datetime DEFAULT NULL COMMENT '计划开始时间',
  `plan_end_time` datetime DEFAULT NULL COMMENT '计划结束时间',
  `actual_start_time` datetime DEFAULT NULL COMMENT '实际开始时间',
  `actual_end_time` datetime DEFAULT NULL COMMENT '实际结束时间',
  `standard_hours` decimal(10,2) DEFAULT 0.00 COMMENT '标准工时（小时）',
  `actual_hours` decimal(10,2) DEFAULT 0.00 COMMENT '实际工时（小时）',
  `standard_labor_count` int(11) DEFAULT 1 COMMENT '标准人数',
  `actual_labor_count` int(11) DEFAULT 0 COMMENT '实际人数',
  `work_order_status` tinyint(1) DEFAULT 1 COMMENT '工单状态：1-待开工，2-生产中，3-暂停，4-已完工，5-已取消',
  `is_key_process` tinyint(1) DEFAULT 0 COMMENT '是否关键工序：0-否，1-是',
  `is_quality_check` tinyint(1) DEFAULT 0 COMMENT '是否需要质检：0-否，1-是',
  `quality_check_status` tinyint(1) DEFAULT NULL COMMENT '质检状态：1-待检，2-合格，3-不合格',
  `is_outsourced` tinyint(1) DEFAULT 0 COMMENT '是否外协：0-否，1-是',
  `supplier_id` bigint(20) DEFAULT NULL COMMENT '外协供应商ID',
  `operator_ids` varchar(500) DEFAULT NULL COMMENT '操作员ID列表（JSON格式）',
  `remark` varchar(500) DEFAULT NULL COMMENT '备注',
  `created_by` bigint(20) DEFAULT NULL COMMENT '创建人ID',
  `created_time` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `updated_by` bigint(20) DEFAULT NULL COMMENT '更新人ID',
  `updated_time` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  `is_deleted` tinyint(1) DEFAULT 0 COMMENT '是否删除：0-否，1-是',
  PRIMARY KEY (`id`),
  UNIQUE KEY `uk_work_order_no` (`work_order_no`),
  KEY `idx_production_order_id` (`production_order_id`),
  KEY `idx_process_id` (`process_id`),
  KEY `idx_material_id` (`material_id`),
  KEY `idx_workshop_id` (`workshop_id`),
  KEY `idx_equipment_id` (`equipment_id`),
  KEY `idx_work_order_status` (`work_order_status`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='生产工单主表';

-- =============================================
-- 4. 生产报工记录表 (production_reports)
-- =============================================
DROP TABLE IF EXISTS `production_reports`;
CREATE TABLE `production_reports` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '报工ID',
  `report_no` varchar(50) NOT NULL COMMENT '报工单号',
  `work_order_id` bigint(20) NOT NULL COMMENT '工单ID',
  `production_order_id` bigint(20) NOT NULL COMMENT '生产订单ID',
  `process_id` bigint(20) NOT NULL COMMENT '工序ID',
  `material_id` bigint(20) NOT NULL COMMENT '物料ID',
  `equipment_id` bigint(20) DEFAULT NULL COMMENT '设备ID',
  `workshop_id` bigint(20) DEFAULT NULL COMMENT '车间ID',
  `report_type` tinyint(1) NOT NULL COMMENT '报工类型：1-开工报工，2-完工报工，3-进度报工，4-返工报工',
  `report_date` date NOT NULL COMMENT '报工日期',
  `report_time` datetime NOT NULL COMMENT '报工时间',
  `shift` varchar(20) DEFAULT NULL COMMENT '班次：早班/中班/晚班',
  `operator_id` bigint(20) NOT NULL COMMENT '操作员ID',
  `team_members` varchar(500) DEFAULT NULL COMMENT '班组成员（JSON格式）',
  `report_quantity` decimal(12,4) NOT NULL COMMENT '报工数量',
  `qualified_quantity` decimal(12,4) DEFAULT 0.0000 COMMENT '合格数量',
  `unqualified_quantity` decimal(12,4) DEFAULT 0.0000 COMMENT '不合格数量',
  `scrap_quantity` decimal(12,4) DEFAULT 0.0000 COMMENT '报废数量',
  `unit` varchar(20) NOT NULL COMMENT '单位',
  `work_hours` decimal(10,2) DEFAULT 0.00 COMMENT '工作工时（小时）',
  `standard_hours` decimal(10,2) DEFAULT 0.00 COMMENT '标准工时（小时）',
  `efficiency` decimal(5,2) DEFAULT 0.00 COMMENT '效率（%）',
  `start_time` datetime DEFAULT NULL COMMENT '开始时间',
  `end_time` datetime DEFAULT NULL COMMENT '结束时间',
  `downtime_minutes` int(11) DEFAULT 0 COMMENT '停机时间（分钟）',
  `downtime_reason` varchar(500) DEFAULT NULL COMMENT '停机原因',
  `quality_issue` varchar(500) DEFAULT NULL COMMENT '质量问题',
  `is_approved` tinyint(1) DEFAULT 0 COMMENT '是否审核：0-未审核，1-已审核',
  `approver_id` bigint(20) DEFAULT NULL COMMENT '审核人ID',
  `approval_time` datetime DEFAULT NULL COMMENT '审核时间',
  `remark` varchar(500) DEFAULT NULL COMMENT '备注',
  `created_by` bigint(20) DEFAULT NULL COMMENT '创建人ID',
  `created_time` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `updated_by` bigint(20) DEFAULT NULL COMMENT '更新人ID',
  `updated_time` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  `is_deleted` tinyint(1) DEFAULT 0 COMMENT '是否删除：0-否，1-是',
  PRIMARY KEY (`id`),
  UNIQUE KEY `uk_report_no` (`report_no`),
  KEY `idx_work_order_id` (`work_order_id`),
  KEY `idx_production_order_id` (`production_order_id`),
  KEY `idx_process_id` (`process_id`),
  KEY `idx_operator_id` (`operator_id`),
  KEY `idx_report_date` (`report_date`),
  KEY `idx_report_type` (`report_type`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='生产报工记录表';

-- =============================================
-- 5. 物料需求计划表 (material_requirements)
-- =============================================
DROP TABLE IF EXISTS `material_requirements`;
CREATE TABLE `material_requirements` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '需求ID',
  `requirement_no` varchar(50) NOT NULL COMMENT '需求单号',
  `production_order_id` bigint(20) NOT NULL COMMENT '生产订单ID',
  `work_order_id` bigint(20) DEFAULT NULL COMMENT '工单ID',
  `material_id` bigint(20) NOT NULL COMMENT '物料ID',
  `required_quantity` decimal(12,4) NOT NULL COMMENT '需求数量',
  `issued_quantity` decimal(12,4) DEFAULT 0.0000 COMMENT '已发放数量',
  `returned_quantity` decimal(12,4) DEFAULT 0.0000 COMMENT '已退料数量',
  `actual_quantity` decimal(12,4) DEFAULT 0.0000 COMMENT '实际用量',
  `unit` varchar(20) NOT NULL COMMENT '单位',
  `warehouse_id` bigint(20) DEFAULT NULL COMMENT '仓库ID',
  `location_id` bigint(20) DEFAULT NULL COMMENT '库位ID',
  `required_date` date NOT NULL COMMENT '需求日期',
  `issue_date` date DEFAULT NULL COMMENT '发放日期',
  `requirement_status` tinyint(1) DEFAULT 1 COMMENT '需求状态：1-待发料，2-部分发料，3-已发料，4-已完成',
  `is_key_material` tinyint(1) DEFAULT 0 COMMENT '是否关键物料：0-否，1-是',
  `batch_no` varchar(50) DEFAULT NULL COMMENT '指定批次号',
  `remark` varchar(500) DEFAULT NULL COMMENT '备注',
  `created_by` bigint(20) DEFAULT NULL COMMENT '创建人ID',
  `created_time` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `updated_by` bigint(20) DEFAULT NULL COMMENT '更新人ID',
  `updated_time` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  `is_deleted` tinyint(1) DEFAULT 0 COMMENT '是否删除：0-否，1-是',
  PRIMARY KEY (`id`),
  UNIQUE KEY `uk_requirement_no` (`requirement_no`),
  KEY `idx_production_order_id` (`production_order_id`),
  KEY `idx_work_order_id` (`work_order_id`),
  KEY `idx_material_id` (`material_id`),
  KEY `idx_requirement_status` (`requirement_status`),
  KEY `idx_required_date` (`required_date`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='物料需求计划表';

-- =============================================
-- 6. 生产领料单表 (material_picking_orders)
-- =============================================
DROP TABLE IF EXISTS `material_picking_orders`;
CREATE TABLE `material_picking_orders` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '领料单ID',
  `picking_no` varchar(50) NOT NULL COMMENT '领料单号',
  `production_order_id` bigint(20) NOT NULL COMMENT '生产订单ID',
  `work_order_id` bigint(20) DEFAULT NULL COMMENT '工单ID',
  `picking_type` tinyint(1) NOT NULL COMMENT '领料类型：1-正常领料，2-补料，3-超额领料',
  `warehouse_id` bigint(20) NOT NULL COMMENT '仓库ID',
  `workshop_id` bigint(20) DEFAULT NULL COMMENT '车间ID',
  `picking_date` date NOT NULL COMMENT '领料日期',
  `plan_picking_time` datetime DEFAULT NULL COMMENT '计划领料时间',
  `actual_picking_time` datetime DEFAULT NULL COMMENT '实际领料时间',
  `picker_id` bigint(20) DEFAULT NULL COMMENT '领料人ID',
  `issuer_id` bigint(20) DEFAULT NULL COMMENT '发料人ID',
  `total_items` int(11) DEFAULT 0 COMMENT '物料种类数',
  `picking_status` tinyint(1) DEFAULT 1 COMMENT '领料状态：1-待审核，2-待发料，3-部分发料，4-已完成，5-已取消',
  `is_returned` tinyint(1) DEFAULT 0 COMMENT '是否已退料：0-否，1-是',
  `remark` varchar(500) DEFAULT NULL COMMENT '备注',
  `created_by` bigint(20) DEFAULT NULL COMMENT '创建人ID',
  `created_time` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `updated_by` bigint(20) DEFAULT NULL COMMENT '更新人ID',
  `updated_time` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  `is_deleted` tinyint(1) DEFAULT 0 COMMENT '是否删除：0-否，1-是',
  PRIMARY KEY (`id`),
  UNIQUE KEY `uk_picking_no` (`picking_no`),
  KEY `idx_production_order_id` (`production_order_id`),
  KEY `idx_work_order_id` (`work_order_id`),
  KEY `idx_warehouse_id` (`warehouse_id`),
  KEY `idx_picking_status` (`picking_status`),
  KEY `idx_picking_date` (`picking_date`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='生产领料单表';

-- =============================================
-- 7. 生产领料单明细表 (material_picking_details)
-- =============================================
DROP TABLE IF EXISTS `material_picking_details`;
CREATE TABLE `material_picking_details` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '明细ID',
  `picking_order_id` bigint(20) NOT NULL COMMENT '领料单ID',
  `requirement_id` bigint(20) DEFAULT NULL COMMENT '物料需求ID',
  `material_id` bigint(20) NOT NULL COMMENT '物料ID',
  `location_id` bigint(20) DEFAULT NULL COMMENT '库位ID',
  `batch_no` varchar(50) DEFAULT NULL COMMENT '批次号',
  `serial_no` varchar(50) DEFAULT NULL COMMENT '序列号',
  `required_quantity` decimal(12,4) NOT NULL COMMENT '需求数量',
  `issued_quantity` decimal(12,4) DEFAULT 0.0000 COMMENT '发料数量',
  `unit` varchar(20) NOT NULL COMMENT '单位',
  `unit_cost` decimal(12,4) DEFAULT 0.0000 COMMENT '单位成本',
  `total_cost` decimal(15,2) DEFAULT 0.00 COMMENT '总成本',
  `line_status` tinyint(1) DEFAULT 1 COMMENT '明细状态：1-待发料，2-已发料',
  `remark` varchar(500) DEFAULT NULL COMMENT '备注',
  `created_by` bigint(20) DEFAULT NULL COMMENT '创建人ID',
  `created_time` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `updated_by` bigint(20) DEFAULT NULL COMMENT '更新人ID',
  `updated_time` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  `is_deleted` tinyint(1) DEFAULT 0 COMMENT '是否删除：0-否，1-是',
  PRIMARY KEY (`id`),
  KEY `idx_picking_order_id` (`picking_order_id`),
  KEY `idx_requirement_id` (`requirement_id`),
  KEY `idx_material_id` (`material_id`),
  KEY `idx_line_status` (`line_status`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='生产领料单明细表';

-- =============================================
-- 8. 生产退料单表 (material_return_orders)
-- =============================================
DROP TABLE IF EXISTS `material_return_orders`;
CREATE TABLE `material_return_orders` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '退料单ID',
  `return_no` varchar(50) NOT NULL COMMENT '退料单号',
  `picking_order_id` bigint(20) DEFAULT NULL COMMENT '关联领料单ID',
  `production_order_id` bigint(20) NOT NULL COMMENT '生产订单ID',
  `work_order_id` bigint(20) DEFAULT NULL COMMENT '工单ID',
  `return_type` tinyint(1) NOT NULL COMMENT '退料类型：1-剩余退料，2-不良品退料，3-其他退料',
  `warehouse_id` bigint(20) NOT NULL COMMENT '仓库ID',
  `workshop_id` bigint(20) DEFAULT NULL COMMENT '车间ID',
  `return_date` date NOT NULL COMMENT '退料日期',
  `actual_return_time` datetime DEFAULT NULL COMMENT '实际退料时间',
  `returner_id` bigint(20) DEFAULT NULL COMMENT '退料人ID',
  `receiver_id` bigint(20) DEFAULT NULL COMMENT '接收人ID',
  `total_items` int(11) DEFAULT 0 COMMENT '物料种类数',
  `return_status` tinyint(1) DEFAULT 1 COMMENT '退料状态：1-待审核，2-待入库，3-已完成，4-已取消',
  `return_reason` varchar(500) DEFAULT NULL COMMENT '退料原因',
  `remark` varchar(500) DEFAULT NULL COMMENT '备注',
  `created_by` bigint(20) DEFAULT NULL COMMENT '创建人ID',
  `created_time` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `updated_by` bigint(20) DEFAULT NULL COMMENT '更新人ID',
  `updated_time` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  `is_deleted` tinyint(1) DEFAULT 0 COMMENT '是否删除：0-否，1-是',
  PRIMARY KEY (`id`),
  UNIQUE KEY `uk_return_no` (`return_no`),
  KEY `idx_picking_order_id` (`picking_order_id`),
  KEY `idx_production_order_id` (`production_order_id`),
  KEY `idx_work_order_id` (`work_order_id`),
  KEY `idx_return_status` (`return_status`),
  KEY `idx_return_date` (`return_date`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='生产退料单表';

-- =============================================
-- 9. 生产退料单明细表 (material_return_details)
-- =============================================
DROP TABLE IF EXISTS `material_return_details`;
CREATE TABLE `material_return_details` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '明细ID',
  `return_order_id` bigint(20) NOT NULL COMMENT '退料单ID',
  `picking_detail_id` bigint(20) DEFAULT NULL COMMENT '领料明细ID',
  `material_id` bigint(20) NOT NULL COMMENT '物料ID',
  `location_id` bigint(20) DEFAULT NULL COMMENT '库位ID',
  `batch_no` varchar(50) DEFAULT NULL COMMENT '批次号',
  `serial_no` varchar(50) DEFAULT NULL COMMENT '序列号',
  `return_quantity` decimal(12,4) NOT NULL COMMENT '退料数量',
  `unit` varchar(20) NOT NULL COMMENT '单位',
  `unit_cost` decimal(12,4) DEFAULT 0.0000 COMMENT '单位成本',
  `total_cost` decimal(15,2) DEFAULT 0.00 COMMENT '总成本',
  `quality_status` tinyint(1) DEFAULT 1 COMMENT '质量状态：1-合格，2-不合格',
  `line_status` tinyint(1) DEFAULT 1 COMMENT '明细状态：1-待入库，2-已入库',
  `remark` varchar(500) DEFAULT NULL COMMENT '备注',
  `created_by` bigint(20) DEFAULT NULL COMMENT '创建人ID',
  `created_time` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `updated_by` bigint(20) DEFAULT NULL COMMENT '更新人ID',
  `updated_time` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  `is_deleted` tinyint(1) DEFAULT 0 COMMENT '是否删除：0-否，1-是',
  PRIMARY KEY (`id`),
  KEY `idx_return_order_id` (`return_order_id`),
  KEY `idx_picking_detail_id` (`picking_detail_id`),
  KEY `idx_material_id` (`material_id`),
  KEY `idx_line_status` (`line_status`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='生产退料单明细表';

-- =============================================
-- 10. 设备使用记录表 (equipment_usage_records)
-- =============================================
DROP TABLE IF EXISTS `equipment_usage_records`;
CREATE TABLE `equipment_usage_records` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '记录ID',
  `record_no` varchar(50) NOT NULL COMMENT '记录编号',
  `equipment_id` bigint(20) NOT NULL COMMENT '设备ID',
  `work_order_id` bigint(20) DEFAULT NULL COMMENT '工单ID',
  `production_order_id` bigint(20) DEFAULT NULL COMMENT '生产订单ID',
  `usage_type` tinyint(1) NOT NULL COMMENT '使用类型：1-生产，2-调试，3-维修，4-其他',
  `start_time` datetime NOT NULL COMMENT '开始时间',
  `end_time` datetime DEFAULT NULL COMMENT '结束时间',
  `usage_hours` decimal(10,2) DEFAULT 0.00 COMMENT '使用时长（小时）',
  `operator_id` bigint(20) DEFAULT NULL COMMENT '操作员ID',
  `workshop_id` bigint(20) DEFAULT NULL COMMENT '车间ID',
  `production_quantity` decimal(12,4) DEFAULT 0.0000 COMMENT '生产数量',
  `downtime_minutes` int(11) DEFAULT 0 COMMENT '停机时间（分钟）',
  `downtime_reason` varchar(500) DEFAULT NULL COMMENT '停机原因',
  `equipment_status` tinyint(1) DEFAULT 1 COMMENT '设备状态：1-正常，2-异常',
  `fault_description` varchar(500) DEFAULT NULL COMMENT '故障描述',
  `meter_reading_start` decimal(12,2) DEFAULT NULL COMMENT '起始读数',
  `meter_reading_end` decimal(12,2) DEFAULT NULL COMMENT '结束读数',
  `power_consumption` decimal(12,2) DEFAULT NULL COMMENT '耗电量（kWh）',
  `remark` varchar(500) DEFAULT NULL COMMENT '备注',
  `created_by` bigint(20) DEFAULT NULL COMMENT '创建人ID',
  `created_time` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `updated_by` bigint(20) DEFAULT NULL COMMENT '更新人ID',
  `updated_time` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  `is_deleted` tinyint(1) DEFAULT 0 COMMENT '是否删除：0-否，1-是',
  PRIMARY KEY (`id`),
  UNIQUE KEY `uk_record_no` (`record_no`),
  KEY `idx_equipment_id` (`equipment_id`),
  KEY `idx_work_order_id` (`work_order_id`),
  KEY `idx_production_order_id` (`production_order_id`),
  KEY `idx_start_time` (`start_time`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='设备使用记录表';

-- =============================================
-- 11. 生产异常记录表 (production_exceptions)
-- =============================================
DROP TABLE IF EXISTS `production_exceptions`;
CREATE TABLE `production_exceptions` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '异常ID',
  `exception_no` varchar(50) NOT NULL COMMENT '异常编号',
  `exception_type` tinyint(1) NOT NULL COMMENT '异常类型：1-质量异常，2-设备异常，3-物料异常，4-人员异常，5-工艺异常，6-其他',
  `exception_level` tinyint(1) DEFAULT 3 COMMENT '异常级别：1-严重，2-重要，3-一般',
  `production_order_id` bigint(20) DEFAULT NULL COMMENT '生产订单ID',
  `work_order_id` bigint(20) DEFAULT NULL COMMENT '工单ID',
  `process_id` bigint(20) DEFAULT NULL COMMENT '工序ID',
  `equipment_id` bigint(20) DEFAULT NULL COMMENT '设备ID',
  `material_id` bigint(20) DEFAULT NULL COMMENT '物料ID',
  `workshop_id` bigint(20) DEFAULT NULL COMMENT '车间ID',
  `exception_time` datetime NOT NULL COMMENT '异常发生时间',
  `exception_description` varchar(1000) NOT NULL COMMENT '异常描述',
  `exception_quantity` decimal(12,4) DEFAULT NULL COMMENT '异常数量',
  `impact_description` varchar(500) DEFAULT NULL COMMENT '影响描述',
  `reporter_id` bigint(20) NOT NULL COMMENT '报告人ID',
  `exception_status` tinyint(1) DEFAULT 1 COMMENT '异常状态：1-待处理，2-处理中，3-已处理，4-已关闭',
  `handler_id` bigint(20) DEFAULT NULL COMMENT '处理人ID',
  `handle_time` datetime DEFAULT NULL COMMENT '处理时间',
  `handle_method` varchar(500) DEFAULT NULL COMMENT '处理方法',
  `handle_result` varchar(500) DEFAULT NULL COMMENT '处理结果',
  `root_cause` varchar(500) DEFAULT NULL COMMENT '根本原因',
  `corrective_action` varchar(500) DEFAULT NULL COMMENT '纠正措施',
  `preventive_action` varchar(500) DEFAULT NULL COMMENT '预防措施',
  `is_recurrence` tinyint(1) DEFAULT 0 COMMENT '是否重复发生：0-否，1-是',
  `closure_time` datetime DEFAULT NULL COMMENT '关闭时间',
  `remark` varchar(500) DEFAULT NULL COMMENT '备注',
  `created_by` bigint(20) DEFAULT NULL COMMENT '创建人ID',
  `created_time` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `updated_by` bigint(20) DEFAULT NULL COMMENT '更新人ID',
  `updated_time` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  `is_deleted` tinyint(1) DEFAULT 0 COMMENT '是否删除：0-否，1-是',
  PRIMARY KEY (`id`),
  UNIQUE KEY `uk_exception_no` (`exception_no`),
  KEY `idx_exception_type` (`exception_type`),
  KEY `idx_exception_level` (`exception_level`),
  KEY `idx_production_order_id` (`production_order_id`),
  KEY `idx_work_order_id` (`work_order_id`),
  KEY `idx_exception_status` (`exception_status`),
  KEY `idx_exception_time` (`exception_time`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='生产异常记录表';

-- =============================================
-- 12. 生产暂停记录表 (production_pause_records)
-- =============================================
DROP TABLE IF EXISTS `production_pause_records`;
CREATE TABLE `production_pause_records` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '记录ID',
  `record_no` varchar(50) NOT NULL COMMENT '记录编号',
  `work_order_id` bigint(20) NOT NULL COMMENT '工单ID',
  `production_order_id` bigint(20) NOT NULL COMMENT '生产订单ID',
  `pause_type` tinyint(1) NOT NULL COMMENT '暂停类型：1-设备故障，2-物料短缺，3-质量问题，4-换模换线，5-人员休息，6-其他',
  `pause_time` datetime NOT NULL COMMENT '暂停时间',
  `resume_time` datetime DEFAULT NULL COMMENT '恢复时间',
  `pause_duration` int(11) DEFAULT 0 COMMENT '暂停时长（分钟）',
  `pause_reason` varchar(500) NOT NULL COMMENT '暂停原因',
  `operator_id` bigint(20) DEFAULT NULL COMMENT '操作员ID',
  `approver_id` bigint(20) DEFAULT NULL COMMENT '批准人ID',
  `pause_status` tinyint(1) DEFAULT 1 COMMENT '暂停状态：1-暂停中，2-已恢复',
  `remark` varchar(500) DEFAULT NULL COMMENT '备注',
  `created_by` bigint(20) DEFAULT NULL COMMENT '创建人ID',
  `created_time` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `updated_by` bigint(20) DEFAULT NULL COMMENT '更新人ID',
  `updated_time` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  `is_deleted` tinyint(1) DEFAULT 0 COMMENT '是否删除：0-否，1-是',
  PRIMARY KEY (`id`),
  UNIQUE KEY `uk_record_no` (`record_no`),
  KEY `idx_work_order_id` (`work_order_id`),
  KEY `idx_production_order_id` (`production_order_id`),
  KEY `idx_pause_type` (`pause_type`),
  KEY `idx_pause_time` (`pause_time`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='生产暂停记录表';

-- =============================================
-- 13. 生产完工入库单表 (production_completion_orders)
-- =============================================
DROP TABLE IF EXISTS `production_completion_orders`;
CREATE TABLE `production_completion_orders` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '完工单ID',
  `completion_no` varchar(50) NOT NULL COMMENT '完工单号',
  `production_order_id` bigint(20) NOT NULL COMMENT '生产订单ID',
  `material_id` bigint(20) NOT NULL COMMENT '物料ID',
  `batch_no` varchar(50) NOT NULL COMMENT '生产批次号',
  `completion_quantity` decimal(12,4) NOT NULL COMMENT '完工数量',
  `qualified_quantity` decimal(12,4) NOT NULL COMMENT '合格数量',
  `unqualified_quantity` decimal(12,4) DEFAULT 0.0000 COMMENT '不合格数量',
  `unit` varchar(20) NOT NULL COMMENT '单位',
  `warehouse_id` bigint(20) NOT NULL COMMENT '入库仓库ID',
  `location_id` bigint(20) DEFAULT NULL COMMENT '库位ID',
  `completion_date` date NOT NULL COMMENT '完工日期',
  `completion_time` datetime NOT NULL COMMENT '完工时间',
  `quality_inspector_id` bigint(20) DEFAULT NULL COMMENT '质检员ID',
  `inspect_result` tinyint(1) DEFAULT NULL COMMENT '检验结果：1-合格，2-不合格，3-让步接收',
  `inspect_report_no` varchar(50) DEFAULT NULL COMMENT '检验报告单号',
  `warehouse_keeper_id` bigint(20) DEFAULT NULL COMMENT '仓管员ID',
  `inbound_time` datetime DEFAULT NULL COMMENT '入库时间',
  `completion_status` tinyint(1) DEFAULT 1 COMMENT '完工状态：1-待质检，2-待入库，3-已入库，4-已取消',
  `production_cost` decimal(15,2) DEFAULT 0.00 COMMENT '生产成本',
  `remark` varchar(500) DEFAULT NULL COMMENT '备注',
  `created_by` bigint(20) DEFAULT NULL COMMENT '创建人ID',
  `created_time` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `updated_by` bigint(20) DEFAULT NULL COMMENT '更新人ID',
  `updated_time` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  `is_deleted` tinyint(1) DEFAULT 0 COMMENT '是否删除：0-否，1-是',
  PRIMARY KEY (`id`),
  UNIQUE KEY `uk_completion_no` (`completion_no`),
  KEY `idx_production_order_id` (`production_order_id`),
  KEY `idx_material_id` (`material_id`),
  KEY `idx_batch_no` (`batch_no`),
  KEY `idx_completion_status` (`completion_status`),
  KEY `idx_completion_date` (`completion_date`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='生产完工入库单表';

-- =============================================
-- 14. 生产成本核算表 (production_costs)
-- =============================================
DROP TABLE IF EXISTS `production_costs`;
CREATE TABLE `production_costs` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '成本ID',
  `cost_no` varchar(50) NOT NULL COMMENT '成本单号',
  `production_order_id` bigint(20) NOT NULL COMMENT '生产订单ID',
  `material_id` bigint(20) NOT NULL COMMENT '物料ID',
  `batch_no` varchar(50) DEFAULT NULL COMMENT '生产批次号',
  `production_quantity` decimal(12,4) NOT NULL COMMENT '生产数量',
  `unit` varchar(20) NOT NULL COMMENT '单位',
  `material_cost` decimal(15,2) DEFAULT 0.00 COMMENT '直接材料成本',
  `labor_cost` decimal(15,2) DEFAULT 0.00 COMMENT '直接人工成本',
  `equipment_cost` decimal(15,2) DEFAULT 0.00 COMMENT '设备折旧成本',
  `manufacturing_overhead` decimal(15,2) DEFAULT 0.00 COMMENT '制造费用',
  `outsourcing_cost` decimal(15,2) DEFAULT 0.00 COMMENT '外协加工费',
  `quality_cost` decimal(15,2) DEFAULT 0.00 COMMENT '质量成本（不良品损失）',
  `other_cost` decimal(15,2) DEFAULT 0.00 COMMENT '其他成本',
  `total_cost` decimal(15,2) DEFAULT 0.00 COMMENT '总成本',
  `unit_cost` decimal(12,4) DEFAULT 0.0000 COMMENT '单位成本',
  `standard_cost` decimal(12,4) DEFAULT 0.0000 COMMENT '标准成本',
  `cost_variance` decimal(15,2) DEFAULT 0.00 COMMENT '成本差异',
  `calculation_date` date NOT NULL COMMENT '核算日期',
  `accountant_id` bigint(20) DEFAULT NULL COMMENT '核算人ID',
  `cost_status` tinyint(1) DEFAULT 1 COMMENT '核算状态：1-待核算，2-已核算，3-已审核',
  `remark` varchar(500) DEFAULT NULL COMMENT '备注',
  `created_by` bigint(20) DEFAULT NULL COMMENT '创建人ID',
  `created_time` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `updated_by` bigint(20) DEFAULT NULL COMMENT '更新人ID',
  `updated_time` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  `is_deleted` tinyint(1) DEFAULT 0 COMMENT '是否删除：0-否，1-是',
  PRIMARY KEY (`id`),
  UNIQUE KEY `uk_cost_no` (`cost_no`),
  KEY `idx_production_order_id` (`production_order_id`),
  KEY `idx_material_id` (`material_id`),
  KEY `idx_batch_no` (`batch_no`),
  KEY `idx_calculation_date` (`calculation_date`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='生产成本核算表';

-- =============================================
-- 15. 生产KPI指标表 (production_kpi)
-- =============================================
DROP TABLE IF EXISTS `production_kpi`;
CREATE TABLE `production_kpi` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT 'KPI ID',
  `kpi_date` date NOT NULL COMMENT 'KPI日期',
  `kpi_type` tinyint(1) NOT NULL COMMENT 'KPI类型：1-日报，2-周报，3-月报',
  `workshop_id` bigint(20) DEFAULT NULL COMMENT '车间ID',
  `production_line` varchar(100) DEFAULT NULL COMMENT '生产线',
  `plan_quantity` decimal(12,4) DEFAULT 0.0000 COMMENT '计划生产数量',
  `actual_quantity` decimal(12,4) DEFAULT 0.0000 COMMENT '实际生产数量',
  `qualified_quantity` decimal(12,4) DEFAULT 0.0000 COMMENT '合格数量',
  `unqualified_quantity` decimal(12,4) DEFAULT 0.0000 COMMENT '不合格数量',
  `completion_rate` decimal(5,2) DEFAULT 0.00 COMMENT '完成率（%）',
  `qualified_rate` decimal(5,2) DEFAULT 0.00 COMMENT '合格率（%）',
  `first_pass_yield` decimal(5,2) DEFAULT 0.00 COMMENT '一次合格率（%）',
  `plan_hours` decimal(10,2) DEFAULT 0.00 COMMENT '计划工时',
  `actual_hours` decimal(10,2) DEFAULT 0.00 COMMENT '实际工时',
  `efficiency` decimal(5,2) DEFAULT 0.00 COMMENT '效率（%）',
  `equipment_utilization` decimal(5,2) DEFAULT 0.00 COMMENT '设备利用率（%）',
  `downtime_hours` decimal(10,2) DEFAULT 0.00 COMMENT '停机时长（小时）',
  `downtime_rate` decimal(5,2) DEFAULT 0.00 COMMENT '停机率（%）',
  `oee` decimal(5,2) DEFAULT 0.00 COMMENT 'OEE设备综合效率（%）',
  `on_time_delivery_rate` decimal(5,2) DEFAULT 0.00 COMMENT '准时交付率（%）',
  `average_cycle_time` decimal(10,2) DEFAULT 0.00 COMMENT '平均生产周期（小时）',
  `exception_count` int(11) DEFAULT 0 COMMENT '异常次数',
  `rework_quantity` decimal(12,4) DEFAULT 0.0000 COMMENT '返工数量',
  `scrap_quantity` decimal(12,4) DEFAULT 0.0000 COMMENT '报废数量',
  `material_utilization` decimal(5,2) DEFAULT 0.00 COMMENT '物料利用率（%）',
  `labor_productivity` decimal(12,4) DEFAULT 0.0000 COMMENT '人均产量',
  `remark` varchar(500) DEFAULT NULL COMMENT '备注',
  `created_by` bigint(20) DEFAULT NULL COMMENT '创建人ID',
  `created_time` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `updated_by` bigint(20) DEFAULT NULL COMMENT '更新人ID',
  `updated_time` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  PRIMARY KEY (`id`),
  KEY `idx_kpi_date` (`kpi_date`),
  KEY `idx_kpi_type` (`kpi_type`),
  KEY `idx_workshop_id` (`workshop_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='生产KPI指标表';

SET FOREIGN_KEY_CHECKS = 1;

-- =============================================
-- 说明文档
-- =============================================
/*
MES系统生产管理模块表结构说明：

核心表说明：

1. production_plans（生产计划主表）
   - 支持月度、周、日计划及临时计划
   - 计划执行跟踪
   - 完成率自动计算

2. production_orders（生产订单主表）
   - 系统核心表，生产执行的基础
   - 支持多种来源：销售订单、库存补货、预测生产
   - 完整的订单状态流转
   - 关联BOM和工艺路线

3. work_orders（生产工单主表）
   - 按工序拆分的作业单元
   - 支持并行工序和外协工序
   - 工单级别的生产执行管理
   - 质检控制点管理

4. production_reports（生产报工记录表）
   - 开工、完工、进度、返工报工
   - 班次管理
   - 实时产量统计
   - 效率计算

5. material_requirements（物料需求计划表）
   - 基于BOM自动计算物料需求
   - 支持物料发放跟踪
   - 关键物料标识

6. material_picking_orders & material_picking_details（领料单）
   - 生产领料管理
   - 支持正常领料、补料、超额领料
   - 完整的发料流程

7. material_return_orders & material_return_details（退料单）
   - 剩余物料退库
   - 不良品退料
   - 与领料单关联

8. equipment_usage_records（设备使用记录表）
   - 设备运行时间统计
   - 生产数量记录
   - 停机原因分析
   - 能耗统计

9. production_exceptions（生产异常记录表）
   - 多类型异常管理
   - 异常处理流程
   - 根本原因分析
   - 纠正和预防措施

10. production_pause_records（生产暂停记录表）
    - 停机原因记录
    - 停机时长统计
    - 恢复时间追踪

11. production_completion_orders（生产完工入库单表）
    - 完工产品入库
    - 质检流程集成
    - 批次管理

12. production_costs（生产成本核算表）
    - 多维度成本核算
    - 成本差异分析
    - 支持标准成本对比

13. production_kpi（生产KPI指标表）
    - 全面的生产指标
    - OEE设备综合效率
    - 合格率、效率、准时交付率等
    - 支持日报、周报、月报

业务流程：

生产计划流程：
1. 创建生产计划
2. 生成生产订单
3. 拆分工单
4. 物料需求计算
5. 执行监控
6. 完成统计

生产执行流程：
1. 生产订单下达
2. 物料领用
3. 开工报工
4. 进度报工
5. 完工报工
6. 质量检验
7. 完工入库
8. 剩余退料

物料管理流程：
1. MRP计算物料需求
2. 创建领料单
3. 仓库发料
4. 生产使用
5. 剩余退料

异常处理流程：
1. 异常报告
2. 异常分析
3. 处理措施
4. 效果验证
5. 关闭异常

KPI指标说明：
- OEE = 可用率 × 性能率 × 质量率
- 完成率 = 实际产量 / 计划产量 × 100%
- 合格率 = 合格数量 / 实际产量 × 100%
- 效率 = 标准工时 / 实际工时 × 100%
- 准时交付率 = 准时完成订单数 / 总订单数 × 100%

数据特点：
1. 所有表都包含软删除字段
2. 所有表都包含完整的审计字段
3. 关键字段都建立了索引
4. 支持完整的生产追溯
5. 实时数据统计和分析

使用建议：
1. 生产订单是核心，工单是执行单元
2. 报工数据要及时准确，影响KPI计算
3. 物料需求要提前计算，保证生产连续性
4. 异常要及时记录和处理，避免重复发生
5. 定期分析KPI数据，持续改进生产效率
6. 成本核算要及时，为决策提供依据
7. 建议使用批次号进行产品追溯管理
*/
