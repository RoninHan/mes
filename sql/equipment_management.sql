-- =============================================
-- MES系统 - 设备管理模块数据库表结构
-- 创建日期: 2025-10-29
-- 数据库: MySQL 5.7+
-- =============================================

-- 设置字符集
SET NAMES utf8mb4;
SET FOREIGN_KEY_CHECKS = 0;

-- =============================================
-- 1. 设备维护计划表 (equipment_maintenance_plans)
-- =============================================
DROP TABLE IF EXISTS `equipment_maintenance_plans`;
CREATE TABLE `equipment_maintenance_plans` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '计划ID',
  `plan_no` varchar(50) NOT NULL COMMENT '计划编号',
  `plan_name` varchar(200) NOT NULL COMMENT '计划名称',
  `equipment_id` bigint(20) NOT NULL COMMENT '设备ID',
  `maintenance_type` tinyint(1) NOT NULL COMMENT '维护类型：1-日常保养，2-一级保养，3-二级保养，4-三级保养，5-大修',
  `maintenance_level` tinyint(1) DEFAULT 1 COMMENT '维护级别：1-日常，2-一级，3-二级，4-三级',
  `cycle_type` tinyint(1) NOT NULL COMMENT '周期类型：1-按时间，2-按运行时长，3-按生产数量',
  `cycle_value` int(11) NOT NULL COMMENT '周期值',
  `cycle_unit` varchar(20) NOT NULL COMMENT '周期单位：天/小时/件',
  `advance_days` int(11) DEFAULT 3 COMMENT '提前提醒天数',
  `standard_hours` decimal(10,2) DEFAULT 0.00 COMMENT '标准工时（小时）',
  `maintenance_content` text COMMENT '维护内容',
  `maintenance_items` text COMMENT '维护项目（JSON格式）',
  `required_parts` text COMMENT '所需备件（JSON格式）',
  `required_tools` varchar(500) DEFAULT NULL COMMENT '所需工具',
  `safety_requirements` varchar(1000) DEFAULT NULL COMMENT '安全要求',
  `responsible_person_id` bigint(20) DEFAULT NULL COMMENT '负责人ID',
  `maintenance_team` varchar(500) DEFAULT NULL COMMENT '维护班组',
  `is_active` tinyint(1) DEFAULT 1 COMMENT '是否启用：0-否，1-是',
  `last_maintenance_date` date DEFAULT NULL COMMENT '上次维护日期',
  `next_maintenance_date` date DEFAULT NULL COMMENT '下次维护日期',
  `remark` varchar(500) DEFAULT NULL COMMENT '备注',
  `created_by` bigint(20) DEFAULT NULL COMMENT '创建人ID',
  `created_time` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `updated_by` bigint(20) DEFAULT NULL COMMENT '更新人ID',
  `updated_time` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  `is_deleted` tinyint(1) DEFAULT 0 COMMENT '是否删除：0-否，1-是',
  PRIMARY KEY (`id`),
  UNIQUE KEY `uk_plan_no` (`plan_no`),
  KEY `idx_equipment_id` (`equipment_id`),
  KEY `idx_maintenance_type` (`maintenance_type`),
  KEY `idx_next_maintenance_date` (`next_maintenance_date`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='设备维护计划表';

-- =============================================
-- 2. 设备维护任务表 (equipment_maintenance_tasks)
-- =============================================
DROP TABLE IF EXISTS `equipment_maintenance_tasks`;
CREATE TABLE `equipment_maintenance_tasks` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '任务ID',
  `task_no` varchar(50) NOT NULL COMMENT '任务编号',
  `plan_id` bigint(20) DEFAULT NULL COMMENT '维护计划ID',
  `equipment_id` bigint(20) NOT NULL COMMENT '设备ID',
  `maintenance_type` tinyint(1) NOT NULL COMMENT '维护类型：1-日常保养，2-一级保养，3-二级保养，4-三级保养，5-大修，6-临时维护',
  `task_source` tinyint(1) DEFAULT 1 COMMENT '任务来源：1-计划生成，2-手工创建，3-故障触发',
  `plan_start_date` date NOT NULL COMMENT '计划开始日期',
  `plan_end_date` date NOT NULL COMMENT '计划结束日期',
  `actual_start_date` date DEFAULT NULL COMMENT '实际开始日期',
  `actual_end_date` date DEFAULT NULL COMMENT '实际结束日期',
  `standard_hours` decimal(10,2) DEFAULT 0.00 COMMENT '标准工时（小时）',
  `actual_hours` decimal(10,2) DEFAULT 0.00 COMMENT '实际工时（小时）',
  `maintenance_content` text COMMENT '维护内容',
  `maintenance_result` text COMMENT '维护结果',
  `parts_used` text COMMENT '使用备件（JSON格式）',
  `parts_cost` decimal(15,2) DEFAULT 0.00 COMMENT '备件费用',
  `labor_cost` decimal(15,2) DEFAULT 0.00 COMMENT '人工费用',
  `other_cost` decimal(15,2) DEFAULT 0.00 COMMENT '其他费用',
  `total_cost` decimal(15,2) DEFAULT 0.00 COMMENT '总费用',
  `responsible_person_id` bigint(20) DEFAULT NULL COMMENT '负责人ID',
  `maintenance_team` varchar(500) DEFAULT NULL COMMENT '维护人员（JSON格式）',
  `task_status` tinyint(1) DEFAULT 1 COMMENT '任务状态：1-待执行，2-执行中，3-待验收，4-已完成，5-已取消',
  `priority` tinyint(1) DEFAULT 3 COMMENT '优先级：1-紧急，2-高，3-普通，4-低',
  `is_shutdown` tinyint(1) DEFAULT 1 COMMENT '是否停机：0-否，1-是',
  `downtime_minutes` int(11) DEFAULT 0 COMMENT '停机时长（分钟）',
  `quality_score` decimal(5,2) DEFAULT NULL COMMENT '维护质量评分',
  `inspector_id` bigint(20) DEFAULT NULL COMMENT '验收人ID',
  `inspection_time` datetime DEFAULT NULL COMMENT '验收时间',
  `inspection_result` tinyint(1) DEFAULT NULL COMMENT '验收结果：1-合格，2-不合格',
  `problems_found` varchar(1000) DEFAULT NULL COMMENT '发现的问题',
  `improvement_suggestions` varchar(1000) DEFAULT NULL COMMENT '改善建议',
  `before_images` varchar(1000) DEFAULT NULL COMMENT '维护前图片（JSON格式）',
  `after_images` varchar(1000) DEFAULT NULL COMMENT '维护后图片（JSON格式）',
  `remark` varchar(500) DEFAULT NULL COMMENT '备注',
  `created_by` bigint(20) DEFAULT NULL COMMENT '创建人ID',
  `created_time` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `updated_by` bigint(20) DEFAULT NULL COMMENT '更新人ID',
  `updated_time` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  `is_deleted` tinyint(1) DEFAULT 0 COMMENT '是否删除：0-否，1-是',
  PRIMARY KEY (`id`),
  UNIQUE KEY `uk_task_no` (`task_no`),
  KEY `idx_plan_id` (`plan_id`),
  KEY `idx_equipment_id` (`equipment_id`),
  KEY `idx_task_status` (`task_status`),
  KEY `idx_plan_start_date` (`plan_start_date`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='设备维护任务表';

-- =============================================
-- 3. 设备故障报修表 (equipment_fault_reports)
-- =============================================
DROP TABLE IF EXISTS `equipment_fault_reports`;
CREATE TABLE `equipment_fault_reports` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '报修ID',
  `report_no` varchar(50) NOT NULL COMMENT '报修单号',
  `equipment_id` bigint(20) NOT NULL COMMENT '设备ID',
  `fault_type` tinyint(1) NOT NULL COMMENT '故障类型：1-机械故障，2-电气故障，3-液压故障，4-气动故障，5-控制系统故障，6-其他',
  `fault_level` tinyint(1) DEFAULT 3 COMMENT '故障级别：1-停机故障，2-严重故障，3-一般故障，4-轻微故障',
  `fault_time` datetime NOT NULL COMMENT '故障时间',
  `fault_description` varchar(2000) NOT NULL COMMENT '故障描述',
  `fault_phenomenon` varchar(1000) DEFAULT NULL COMMENT '故障现象',
  `fault_location` varchar(200) DEFAULT NULL COMMENT '故障位置',
  `fault_images` varchar(1000) DEFAULT NULL COMMENT '故障图片（JSON格式）',
  `fault_videos` varchar(1000) DEFAULT NULL COMMENT '故障视频（JSON格式）',
  `reporter_id` bigint(20) NOT NULL COMMENT '报修人ID',
  `workshop_id` bigint(20) DEFAULT NULL COMMENT '车间ID',
  `production_order_id` bigint(20) DEFAULT NULL COMMENT '生产订单ID',
  `work_order_id` bigint(20) DEFAULT NULL COMMENT '工单ID',
  `is_affecting_production` tinyint(1) DEFAULT 1 COMMENT '是否影响生产：0-否，1-是',
  `expected_response_time` datetime DEFAULT NULL COMMENT '期望响应时间',
  `actual_response_time` datetime DEFAULT NULL COMMENT '实际响应时间',
  `response_minutes` int(11) DEFAULT 0 COMMENT '响应时长（分钟）',
  `report_status` tinyint(1) DEFAULT 1 COMMENT '报修状态：1-待接单，2-已接单，3-维修中，4-待验收，5-已完成，6-已取消',
  `remark` varchar(500) DEFAULT NULL COMMENT '备注',
  `created_by` bigint(20) DEFAULT NULL COMMENT '创建人ID',
  `created_time` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `updated_by` bigint(20) DEFAULT NULL COMMENT '更新人ID',
  `updated_time` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  `is_deleted` tinyint(1) DEFAULT 0 COMMENT '是否删除：0-否，1-是',
  PRIMARY KEY (`id`),
  UNIQUE KEY `uk_report_no` (`report_no`),
  KEY `idx_equipment_id` (`equipment_id`),
  KEY `idx_fault_type` (`fault_type`),
  KEY `idx_fault_level` (`fault_level`),
  KEY `idx_report_status` (`report_status`),
  KEY `idx_fault_time` (`fault_time`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='设备故障报修表';

-- =============================================
-- 4. 设备维修工单表 (equipment_repair_orders)
-- =============================================
DROP TABLE IF EXISTS `equipment_repair_orders`;
CREATE TABLE `equipment_repair_orders` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '维修单ID',
  `repair_no` varchar(50) NOT NULL COMMENT '维修单号',
  `fault_report_id` bigint(20) NOT NULL COMMENT '故障报修ID',
  `equipment_id` bigint(20) NOT NULL COMMENT '设备ID',
  `repair_type` tinyint(1) NOT NULL COMMENT '维修类型：1-应急维修，2-计划维修，3-预防维修，4-改造升级',
  `fault_type` tinyint(1) NOT NULL COMMENT '故障类型：1-机械故障，2-电气故障，3-液压故障，4-气动故障，5-控制系统故障，6-其他',
  `fault_cause` varchar(1000) DEFAULT NULL COMMENT '故障原因',
  `repair_plan` varchar(2000) DEFAULT NULL COMMENT '维修方案',
  `plan_start_time` datetime DEFAULT NULL COMMENT '计划开始时间',
  `plan_end_time` datetime DEFAULT NULL COMMENT '计划结束时间',
  `actual_start_time` datetime DEFAULT NULL COMMENT '实际开始时间',
  `actual_end_time` datetime DEFAULT NULL COMMENT '实际结束时间',
  `repair_duration` int(11) DEFAULT 0 COMMENT '维修时长（分钟）',
  `downtime_minutes` int(11) DEFAULT 0 COMMENT '停机时长（分钟）',
  `repair_content` text COMMENT '维修内容',
  `repair_process` text COMMENT '维修过程记录',
  `repair_result` varchar(1000) DEFAULT NULL COMMENT '维修结果',
  `parts_replaced` text COMMENT '更换部件（JSON格式）',
  `parts_cost` decimal(15,2) DEFAULT 0.00 COMMENT '配件费用',
  `labor_cost` decimal(15,2) DEFAULT 0.00 COMMENT '人工费用',
  `outsource_cost` decimal(15,2) DEFAULT 0.00 COMMENT '外协费用',
  `other_cost` decimal(15,2) DEFAULT 0.00 COMMENT '其他费用',
  `total_cost` decimal(15,2) DEFAULT 0.00 COMMENT '总费用',
  `repair_leader_id` bigint(20) DEFAULT NULL COMMENT '维修负责人ID',
  `repair_team` varchar(500) DEFAULT NULL COMMENT '维修人员（JSON格式）',
  `is_outsourced` tinyint(1) DEFAULT 0 COMMENT '是否外协：0-否，1-是',
  `outsource_company` varchar(200) DEFAULT NULL COMMENT '外协单位',
  `repair_status` tinyint(1) DEFAULT 1 COMMENT '维修状态：1-待维修，2-维修中，3-待验收，4-已完成，5-已取消',
  `inspector_id` bigint(20) DEFAULT NULL COMMENT '验收人ID',
  `inspection_time` datetime DEFAULT NULL COMMENT '验收时间',
  `inspection_result` tinyint(1) DEFAULT NULL COMMENT '验收结果：1-合格，2-不合格',
  `is_recurrence` tinyint(1) DEFAULT 0 COMMENT '是否重复故障：0-否，1-是',
  `previous_repair_no` varchar(50) DEFAULT NULL COMMENT '上次维修单号',
  `improvement_measures` varchar(1000) DEFAULT NULL COMMENT '改进措施',
  `preventive_measures` varchar(1000) DEFAULT NULL COMMENT '预防措施',
  `before_images` varchar(1000) DEFAULT NULL COMMENT '维修前图片（JSON格式）',
  `after_images` varchar(1000) DEFAULT NULL COMMENT '维修后图片（JSON格式）',
  `remark` varchar(500) DEFAULT NULL COMMENT '备注',
  `created_by` bigint(20) DEFAULT NULL COMMENT '创建人ID',
  `created_time` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `updated_by` bigint(20) DEFAULT NULL COMMENT '更新人ID',
  `updated_time` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  `is_deleted` tinyint(1) DEFAULT 0 COMMENT '是否删除：0-否，1-是',
  PRIMARY KEY (`id`),
  UNIQUE KEY `uk_repair_no` (`repair_no`),
  KEY `idx_fault_report_id` (`fault_report_id`),
  KEY `idx_equipment_id` (`equipment_id`),
  KEY `idx_repair_type` (`repair_type`),
  KEY `idx_repair_status` (`repair_status`),
  KEY `idx_actual_start_time` (`actual_start_time`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='设备维修工单表';

-- =============================================
-- 5. 设备点检表 (equipment_inspections)
-- =============================================
DROP TABLE IF EXISTS `equipment_inspections`;
CREATE TABLE `equipment_inspections` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '点检ID',
  `inspection_no` varchar(50) NOT NULL COMMENT '点检单号',
  `equipment_id` bigint(20) NOT NULL COMMENT '设备ID',
  `inspection_type` tinyint(1) NOT NULL COMMENT '点检类型：1-日常点检，2-定期点检，3-专项点检，4-巡检',
  `inspection_date` date NOT NULL COMMENT '点检日期',
  `inspection_time` datetime NOT NULL COMMENT '点检时间',
  `shift` varchar(20) DEFAULT NULL COMMENT '班次',
  `inspector_id` bigint(20) NOT NULL COMMENT '点检人ID',
  `inspection_items` text COMMENT '点检项目（JSON格式）',
  `total_items` int(11) DEFAULT 0 COMMENT '点检项目总数',
  `normal_items` int(11) DEFAULT 0 COMMENT '正常项目数',
  `abnormal_items` int(11) DEFAULT 0 COMMENT '异常项目数',
  `inspection_result` tinyint(1) NOT NULL COMMENT '点检结果：1-正常，2-异常',
  `abnormal_description` varchar(1000) DEFAULT NULL COMMENT '异常描述',
  `abnormal_images` varchar(1000) DEFAULT NULL COMMENT '异常图片（JSON格式）',
  `is_reported` tinyint(1) DEFAULT 0 COMMENT '是否已报修：0-否，1-是',
  `fault_report_id` bigint(20) DEFAULT NULL COMMENT '关联故障报修ID',
  `handling_measures` varchar(500) DEFAULT NULL COMMENT '处理措施',
  `remark` varchar(500) DEFAULT NULL COMMENT '备注',
  `created_by` bigint(20) DEFAULT NULL COMMENT '创建人ID',
  `created_time` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `updated_by` bigint(20) DEFAULT NULL COMMENT '更新人ID',
  `updated_time` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  `is_deleted` tinyint(1) DEFAULT 0 COMMENT '是否删除：0-否，1-是',
  PRIMARY KEY (`id`),
  UNIQUE KEY `uk_inspection_no` (`inspection_no`),
  KEY `idx_equipment_id` (`equipment_id`),
  KEY `idx_inspection_type` (`inspection_type`),
  KEY `idx_inspection_date` (`inspection_date`),
  KEY `idx_inspection_result` (`inspection_result`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='设备点检表';

-- =============================================
-- 6. 设备点检标准表 (equipment_inspection_standards)
-- =============================================
DROP TABLE IF EXISTS `equipment_inspection_standards`;
CREATE TABLE `equipment_inspection_standards` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '标准ID',
  `standard_no` varchar(50) NOT NULL COMMENT '标准编号',
  `equipment_id` bigint(20) DEFAULT NULL COMMENT '设备ID（为空表示通用标准）',
  `equipment_category_id` bigint(20) DEFAULT NULL COMMENT '设备分类ID',
  `inspection_type` tinyint(1) NOT NULL COMMENT '点检类型：1-日常点检，2-定期点检，3-专项点检',
  `inspection_frequency` varchar(50) DEFAULT NULL COMMENT '点检频率：如每天/每周/每月',
  `inspection_items` text COMMENT '点检项目列表（JSON格式）',
  `version` varchar(50) DEFAULT '1.0' COMMENT '版本号',
  `is_active` tinyint(1) DEFAULT 1 COMMENT '是否启用：0-否，1-是',
  `remark` varchar(500) DEFAULT NULL COMMENT '备注',
  `created_by` bigint(20) DEFAULT NULL COMMENT '创建人ID',
  `created_time` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `updated_by` bigint(20) DEFAULT NULL COMMENT '更新人ID',
  `updated_time` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  `is_deleted` tinyint(1) DEFAULT 0 COMMENT '是否删除：0-否，1-是',
  PRIMARY KEY (`id`),
  UNIQUE KEY `uk_standard_no` (`standard_no`),
  KEY `idx_equipment_id` (`equipment_id`),
  KEY `idx_equipment_category_id` (`equipment_category_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='设备点检标准表';

-- =============================================
-- 7. 设备备件库存表 (equipment_spare_parts)
-- =============================================
DROP TABLE IF EXISTS `equipment_spare_parts`;
CREATE TABLE `equipment_spare_parts` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '备件ID',
  `part_code` varchar(50) NOT NULL COMMENT '备件编码',
  `part_name` varchar(200) NOT NULL COMMENT '备件名称',
  `part_spec` varchar(200) DEFAULT NULL COMMENT '规格型号',
  `part_category` varchar(100) DEFAULT NULL COMMENT '备件分类',
  `equipment_id` bigint(20) DEFAULT NULL COMMENT '适用设备ID（为空表示通用）',
  `equipment_model` varchar(100) DEFAULT NULL COMMENT '适用设备型号',
  `unit` varchar(20) NOT NULL COMMENT '单位',
  `safety_stock` decimal(12,4) DEFAULT 0.0000 COMMENT '安全库存',
  `max_stock` decimal(12,4) DEFAULT 0.0000 COMMENT '最大库存',
  `current_stock` decimal(12,4) DEFAULT 0.0000 COMMENT '当前库存',
  `available_stock` decimal(12,4) DEFAULT 0.0000 COMMENT '可用库存',
  `locked_stock` decimal(12,4) DEFAULT 0.0000 COMMENT '锁定库存',
  `unit_price` decimal(12,4) DEFAULT 0.0000 COMMENT '单价',
  `total_value` decimal(15,2) DEFAULT 0.00 COMMENT '库存总值',
  `warehouse_id` bigint(20) DEFAULT NULL COMMENT '仓库ID',
  `location_id` bigint(20) DEFAULT NULL COMMENT '库位ID',
  `supplier_id` bigint(20) DEFAULT NULL COMMENT '供应商ID',
  `manufacturer` varchar(200) DEFAULT NULL COMMENT '生产厂家',
  `is_critical` tinyint(1) DEFAULT 0 COMMENT '是否关键备件：0-否，1-是',
  `lead_time` int(11) DEFAULT 0 COMMENT '采购提前期（天）',
  `last_purchase_date` date DEFAULT NULL COMMENT '最后采购日期',
  `last_use_date` date DEFAULT NULL COMMENT '最后使用日期',
  `image_url` varchar(500) DEFAULT NULL COMMENT '备件图片URL',
  `status` tinyint(1) DEFAULT 1 COMMENT '状态：0-停用，1-正常，2-预警',
  `remark` varchar(500) DEFAULT NULL COMMENT '备注',
  `created_by` bigint(20) DEFAULT NULL COMMENT '创建人ID',
  `created_time` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `updated_by` bigint(20) DEFAULT NULL COMMENT '更新人ID',
  `updated_time` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  `is_deleted` tinyint(1) DEFAULT 0 COMMENT '是否删除：0-否，1-是',
  PRIMARY KEY (`id`),
  UNIQUE KEY `uk_part_code` (`part_code`),
  KEY `idx_equipment_id` (`equipment_id`),
  KEY `idx_warehouse_id` (`warehouse_id`),
  KEY `idx_status` (`status`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='设备备件库存表';

-- =============================================
-- 8. 备件出入库记录表 (spare_parts_transactions)
-- =============================================
DROP TABLE IF EXISTS `spare_parts_transactions`;
CREATE TABLE `spare_parts_transactions` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '记录ID',
  `transaction_no` varchar(50) NOT NULL COMMENT '流水号',
  `transaction_type` tinyint(1) NOT NULL COMMENT '交易类型：1-入库，2-出库，3-调拨，4-盘盈，5-盘亏',
  `business_type` varchar(50) DEFAULT NULL COMMENT '业务类型：维修领用/保养领用/退库等',
  `spare_part_id` bigint(20) NOT NULL COMMENT '备件ID',
  `warehouse_id` bigint(20) NOT NULL COMMENT '仓库ID',
  `location_id` bigint(20) DEFAULT NULL COMMENT '库位ID',
  `quantity` decimal(12,4) NOT NULL COMMENT '数量（正数为增加，负数为减少）',
  `before_quantity` decimal(12,4) DEFAULT 0.0000 COMMENT '交易前数量',
  `after_quantity` decimal(12,4) DEFAULT 0.0000 COMMENT '交易后数量',
  `unit` varchar(20) NOT NULL COMMENT '单位',
  `unit_price` decimal(12,4) DEFAULT 0.0000 COMMENT '单价',
  `amount` decimal(15,2) DEFAULT 0.00 COMMENT '金额',
  `equipment_id` bigint(20) DEFAULT NULL COMMENT '设备ID',
  `maintenance_task_id` bigint(20) DEFAULT NULL COMMENT '维护任务ID',
  `repair_order_id` bigint(20) DEFAULT NULL COMMENT '维修单ID',
  `order_no` varchar(50) DEFAULT NULL COMMENT '业务单号',
  `transaction_date` datetime NOT NULL COMMENT '交易时间',
  `handler_id` bigint(20) DEFAULT NULL COMMENT '经办人ID',
  `receiver_id` bigint(20) DEFAULT NULL COMMENT '领用人/接收人ID',
  `remark` varchar(500) DEFAULT NULL COMMENT '备注',
  `created_by` bigint(20) DEFAULT NULL COMMENT '创建人ID',
  `created_time` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  PRIMARY KEY (`id`),
  UNIQUE KEY `uk_transaction_no` (`transaction_no`),
  KEY `idx_spare_part_id` (`spare_part_id`),
  KEY `idx_transaction_type` (`transaction_type`),
  KEY `idx_equipment_id` (`equipment_id`),
  KEY `idx_transaction_date` (`transaction_date`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='备件出入库记录表';

-- =============================================
-- 9. 设备运行记录表 (equipment_running_records)
-- =============================================
DROP TABLE IF EXISTS `equipment_running_records`;
CREATE TABLE `equipment_running_records` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '记录ID',
  `record_no` varchar(50) NOT NULL COMMENT '记录编号',
  `equipment_id` bigint(20) NOT NULL COMMENT '设备ID',
  `record_date` date NOT NULL COMMENT '记录日期',
  `shift` varchar(20) DEFAULT NULL COMMENT '班次',
  `operator_id` bigint(20) DEFAULT NULL COMMENT '操作员ID',
  `start_time` datetime DEFAULT NULL COMMENT '开机时间',
  `end_time` datetime DEFAULT NULL COMMENT '停机时间',
  `running_minutes` int(11) DEFAULT 0 COMMENT '运行时长（分钟）',
  `idle_minutes` int(11) DEFAULT 0 COMMENT '待机时长（分钟）',
  `downtime_minutes` int(11) DEFAULT 0 COMMENT '停机时长（分钟）',
  `maintenance_minutes` int(11) DEFAULT 0 COMMENT '维护时长（分钟）',
  `production_quantity` decimal(12,4) DEFAULT 0.0000 COMMENT '生产数量',
  `qualified_quantity` decimal(12,4) DEFAULT 0.0000 COMMENT '合格数量',
  `unqualified_quantity` decimal(12,4) DEFAULT 0.0000 COMMENT '不合格数量',
  `unit` varchar(20) DEFAULT NULL COMMENT '单位',
  `meter_reading_start` decimal(12,2) DEFAULT NULL COMMENT '起始读数',
  `meter_reading_end` decimal(12,2) DEFAULT NULL COMMENT '结束读数',
  `power_consumption` decimal(12,2) DEFAULT NULL COMMENT '耗电量（kWh）',
  `utilization_rate` decimal(5,2) DEFAULT 0.00 COMMENT '利用率（%）',
  `oee` decimal(5,2) DEFAULT 0.00 COMMENT 'OEE设备综合效率（%）',
  `equipment_status` tinyint(1) DEFAULT 1 COMMENT '设备状态：1-正常，2-异常，3-故障',
  `abnormal_times` int(11) DEFAULT 0 COMMENT '异常次数',
  `fault_times` int(11) DEFAULT 0 COMMENT '故障次数',
  `downtime_reasons` text COMMENT '停机原因（JSON格式）',
  `production_orders` varchar(500) DEFAULT NULL COMMENT '生产订单列表（JSON格式）',
  `remark` varchar(500) DEFAULT NULL COMMENT '备注',
  `created_by` bigint(20) DEFAULT NULL COMMENT '创建人ID',
  `created_time` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `updated_by` bigint(20) DEFAULT NULL COMMENT '更新人ID',
  `updated_time` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  `is_deleted` tinyint(1) DEFAULT 0 COMMENT '是否删除：0-否，1-是',
  PRIMARY KEY (`id`),
  UNIQUE KEY `uk_record_no` (`record_no`),
  KEY `idx_equipment_id` (`equipment_id`),
  KEY `idx_record_date` (`record_date`),
  KEY `idx_equipment_status` (`equipment_status`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='设备运行记录表';

-- =============================================
-- 10. 设备状态监控表 (equipment_status_monitoring)
-- =============================================
DROP TABLE IF EXISTS `equipment_status_monitoring`;
CREATE TABLE `equipment_status_monitoring` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '监控ID',
  `equipment_id` bigint(20) NOT NULL COMMENT '设备ID',
  `monitoring_time` datetime NOT NULL COMMENT '监控时间',
  `equipment_status` tinyint(1) NOT NULL COMMENT '设备状态：1-运行中，2-待机，3-停机，4-故障，5-维护中',
  `running_speed` decimal(10,2) DEFAULT NULL COMMENT '运行速度',
  `temperature` decimal(10,2) DEFAULT NULL COMMENT '温度（℃）',
  `pressure` decimal(10,2) DEFAULT NULL COMMENT '压力（MPa）',
  `vibration` decimal(10,2) DEFAULT NULL COMMENT '振动值',
  `current` decimal(10,2) DEFAULT NULL COMMENT '电流（A）',
  `voltage` decimal(10,2) DEFAULT NULL COMMENT '电压（V）',
  `power` decimal(10,2) DEFAULT NULL COMMENT '功率（kW）',
  `oil_pressure` decimal(10,2) DEFAULT NULL COMMENT '油压（MPa）',
  `oil_temperature` decimal(10,2) DEFAULT NULL COMMENT '油温（℃）',
  `alarm_count` int(11) DEFAULT 0 COMMENT '报警次数',
  `alarm_codes` varchar(500) DEFAULT NULL COMMENT '报警代码（JSON格式）',
  `sensor_data` text COMMENT '传感器数据（JSON格式）',
  `is_abnormal` tinyint(1) DEFAULT 0 COMMENT '是否异常：0-否，1-是',
  `abnormal_indicators` varchar(500) DEFAULT NULL COMMENT '异常指标',
  `remark` varchar(500) DEFAULT NULL COMMENT '备注',
  `created_time` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  PRIMARY KEY (`id`),
  KEY `idx_equipment_id` (`equipment_id`),
  KEY `idx_monitoring_time` (`monitoring_time`),
  KEY `idx_equipment_status` (`equipment_status`),
  KEY `idx_is_abnormal` (`is_abnormal`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='设备状态监控表';

-- =============================================
-- 11. 设备技术档案表 (equipment_technical_files)
-- =============================================
DROP TABLE IF EXISTS `equipment_technical_files`;
CREATE TABLE `equipment_technical_files` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '档案ID',
  `equipment_id` bigint(20) NOT NULL COMMENT '设备ID',
  `file_type` tinyint(1) NOT NULL COMMENT '档案类型：1-使用说明书，2-维护手册，3-电气图纸，4-机械图纸，5-操作规程，6-安全规程，7-其他',
  `file_name` varchar(200) NOT NULL COMMENT '文件名称',
  `file_no` varchar(100) DEFAULT NULL COMMENT '文件编号',
  `version` varchar(50) DEFAULT NULL COMMENT '版本号',
  `file_url` varchar(500) NOT NULL COMMENT '文件URL',
  `file_size` bigint(20) DEFAULT NULL COMMENT '文件大小（字节）',
  `file_format` varchar(20) DEFAULT NULL COMMENT '文件格式：pdf/doc/dwg等',
  `upload_date` date DEFAULT NULL COMMENT '上传日期',
  `is_confidential` tinyint(1) DEFAULT 0 COMMENT '是否保密：0-否，1-是',
  `description` varchar(500) DEFAULT NULL COMMENT '文档描述',
  `remark` varchar(500) DEFAULT NULL COMMENT '备注',
  `created_by` bigint(20) DEFAULT NULL COMMENT '创建人ID',
  `created_time` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `updated_by` bigint(20) DEFAULT NULL COMMENT '更新人ID',
  `updated_time` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  `is_deleted` tinyint(1) DEFAULT 0 COMMENT '是否删除：0-否，1-是',
  PRIMARY KEY (`id`),
  KEY `idx_equipment_id` (`equipment_id`),
  KEY `idx_file_type` (`file_type`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='设备技术档案表';

-- =============================================
-- 12. 设备折旧记录表 (equipment_depreciation_records)
-- =============================================
DROP TABLE IF EXISTS `equipment_depreciation_records`;
CREATE TABLE `equipment_depreciation_records` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '记录ID',
  `equipment_id` bigint(20) NOT NULL COMMENT '设备ID',
  `depreciation_period` varchar(50) NOT NULL COMMENT '折旧期间：如2025-01',
  `depreciation_date` date NOT NULL COMMENT '折旧日期',
  `depreciation_method` tinyint(1) DEFAULT 1 COMMENT '折旧方法：1-直线法，2-双倍余额递减法，3-年数总和法',
  `original_value` decimal(15,2) NOT NULL COMMENT '原值',
  `accumulated_depreciation` decimal(15,2) DEFAULT 0.00 COMMENT '累计折旧',
  `current_depreciation` decimal(15,2) NOT NULL COMMENT '当期折旧',
  `net_value` decimal(15,2) NOT NULL COMMENT '净值',
  `depreciation_rate` decimal(5,2) DEFAULT 0.00 COMMENT '折旧率（%）',
  `remaining_months` int(11) DEFAULT 0 COMMENT '剩余折旧月数',
  `is_fully_depreciated` tinyint(1) DEFAULT 0 COMMENT '是否折旧完毕：0-否，1-是',
  `remark` varchar(500) DEFAULT NULL COMMENT '备注',
  `created_by` bigint(20) DEFAULT NULL COMMENT '创建人ID',
  `created_time` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  PRIMARY KEY (`id`),
  KEY `idx_equipment_id` (`equipment_id`),
  KEY `idx_depreciation_period` (`depreciation_period`),
  KEY `idx_depreciation_date` (`depreciation_date`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='设备折旧记录表';

-- =============================================
-- 13. 设备能耗统计表 (equipment_energy_consumption)
-- =============================================
DROP TABLE IF EXISTS `equipment_energy_consumption`;
CREATE TABLE `equipment_energy_consumption` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '统计ID',
  `equipment_id` bigint(20) NOT NULL COMMENT '设备ID',
  `stat_date` date NOT NULL COMMENT '统计日期',
  `stat_type` tinyint(1) NOT NULL COMMENT '统计类型：1-日统计，2-周统计，3-月统计',
  `electricity_consumption` decimal(12,2) DEFAULT 0.00 COMMENT '耗电量（kWh）',
  `water_consumption` decimal(12,2) DEFAULT 0.00 COMMENT '耗水量（吨）',
  `gas_consumption` decimal(12,2) DEFAULT 0.00 COMMENT '耗气量（m³）',
  `oil_consumption` decimal(12,2) DEFAULT 0.00 COMMENT '耗油量（升）',
  `steam_consumption` decimal(12,2) DEFAULT 0.00 COMMENT '耗蒸汽量（吨）',
  `running_hours` decimal(10,2) DEFAULT 0.00 COMMENT '运行时长（小时）',
  `production_quantity` decimal(12,4) DEFAULT 0.0000 COMMENT '生产数量',
  `unit` varchar(20) DEFAULT NULL COMMENT '产量单位',
  `electricity_cost` decimal(15,2) DEFAULT 0.00 COMMENT '电费',
  `water_cost` decimal(15,2) DEFAULT 0.00 COMMENT '水费',
  `gas_cost` decimal(15,2) DEFAULT 0.00 COMMENT '气费',
  `oil_cost` decimal(15,2) DEFAULT 0.00 COMMENT '油费',
  `steam_cost` decimal(15,2) DEFAULT 0.00 COMMENT '蒸汽费',
  `total_energy_cost` decimal(15,2) DEFAULT 0.00 COMMENT '总能耗成本',
  `unit_energy_consumption` decimal(12,4) DEFAULT 0.0000 COMMENT '单位能耗',
  `energy_efficiency` decimal(5,2) DEFAULT 0.00 COMMENT '能效（%）',
  `remark` varchar(500) DEFAULT NULL COMMENT '备注',
  `created_by` bigint(20) DEFAULT NULL COMMENT '创建人ID',
  `created_time` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `updated_by` bigint(20) DEFAULT NULL COMMENT '更新人ID',
  `updated_time` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  PRIMARY KEY (`id`),
  KEY `idx_equipment_id` (`equipment_id`),
  KEY `idx_stat_date` (`stat_date`),
  KEY `idx_stat_type` (`stat_type`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='设备能耗统计表';

-- =============================================
-- 14. 设备KPI指标表 (equipment_kpi)
-- =============================================
DROP TABLE IF EXISTS `equipment_kpi`;
CREATE TABLE `equipment_kpi` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT 'KPI ID',
  `kpi_date` date NOT NULL COMMENT 'KPI日期',
  `kpi_type` tinyint(1) NOT NULL COMMENT 'KPI类型：1-日报，2-周报，3-月报',
  `equipment_id` bigint(20) DEFAULT NULL COMMENT '设备ID（为空表示汇总）',
  `workshop_id` bigint(20) DEFAULT NULL COMMENT '车间ID',
  `equipment_category_id` bigint(20) DEFAULT NULL COMMENT '设备分类ID',
  `total_equipments` int(11) DEFAULT 0 COMMENT '设备总数',
  `running_equipments` int(11) DEFAULT 0 COMMENT '运行中设备数',
  `idle_equipments` int(11) DEFAULT 0 COMMENT '待机设备数',
  `maintenance_equipments` int(11) DEFAULT 0 COMMENT '维护中设备数',
  `fault_equipments` int(11) DEFAULT 0 COMMENT '故障设备数',
  `total_hours` decimal(12,2) DEFAULT 0.00 COMMENT '总时间（小时）',
  `running_hours` decimal(12,2) DEFAULT 0.00 COMMENT '运行时间（小时）',
  `downtime_hours` decimal(12,2) DEFAULT 0.00 COMMENT '停机时间（小时）',
  `maintenance_hours` decimal(12,2) DEFAULT 0.00 COMMENT '维护时间（小时）',
  `fault_hours` decimal(12,2) DEFAULT 0.00 COMMENT '故障时间（小时）',
  `utilization_rate` decimal(5,2) DEFAULT 0.00 COMMENT '利用率（%）',
  `availability` decimal(5,2) DEFAULT 0.00 COMMENT '可用率（%）',
  `performance_rate` decimal(5,2) DEFAULT 0.00 COMMENT '性能率（%）',
  `quality_rate` decimal(5,2) DEFAULT 0.00 COMMENT '质量率（%）',
  `oee` decimal(5,2) DEFAULT 0.00 COMMENT 'OEE综合效率（%）',
  `mtbf` decimal(12,2) DEFAULT 0.00 COMMENT 'MTBF平均故障间隔时间（小时）',
  `mttr` decimal(12,2) DEFAULT 0.00 COMMENT 'MTTR平均修复时间（小时）',
  `fault_count` int(11) DEFAULT 0 COMMENT '故障次数',
  `fault_rate` decimal(5,2) DEFAULT 0.00 COMMENT '故障率（次/百小时）',
  `maintenance_count` int(11) DEFAULT 0 COMMENT '维护次数',
  `maintenance_completion_rate` decimal(5,2) DEFAULT 0.00 COMMENT '维护完成率（%）',
  `on_time_maintenance_rate` decimal(5,2) DEFAULT 0.00 COMMENT '按时维护率（%）',
  `maintenance_cost` decimal(15,2) DEFAULT 0.00 COMMENT '维护成本',
  `repair_cost` decimal(15,2) DEFAULT 0.00 COMMENT '维修成本',
  `spare_parts_cost` decimal(15,2) DEFAULT 0.00 COMMENT '备件成本',
  `total_maintenance_cost` decimal(15,2) DEFAULT 0.00 COMMENT '总维护成本',
  `energy_consumption` decimal(15,2) DEFAULT 0.00 COMMENT '能耗',
  `energy_cost` decimal(15,2) DEFAULT 0.00 COMMENT '能源成本',
  `production_quantity` decimal(12,4) DEFAULT 0.0000 COMMENT '生产数量',
  `qualified_quantity` decimal(12,4) DEFAULT 0.0000 COMMENT '合格数量',
  `remark` varchar(500) DEFAULT NULL COMMENT '备注',
  `created_by` bigint(20) DEFAULT NULL COMMENT '创建人ID',
  `created_time` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `updated_by` bigint(20) DEFAULT NULL COMMENT '更新人ID',
  `updated_time` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  PRIMARY KEY (`id`),
  KEY `idx_kpi_date` (`kpi_date`),
  KEY `idx_kpi_type` (`kpi_type`),
  KEY `idx_equipment_id` (`equipment_id`),
  KEY `idx_workshop_id` (`workshop_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='设备KPI指标表';

SET FOREIGN_KEY_CHECKS = 1;

-- =============================================
-- 说明文档
-- =============================================
/*
MES系统设备管理模块表结构说明：

核心表说明：

1. equipment_maintenance_plans（设备维护计划表）
   - TPM全面生产维护支持
   - 支持多种周期类型：时间、运行时长、生产数量
   - 自动生成维护任务
   - 提前提醒功能

2. equipment_maintenance_tasks（设备维护任务表）
   - 维护任务执行跟踪
   - 支持日常保养到大修的多级维护
   - 维护质量评分
   - 成本核算
   - 维护前后对比图片

3. equipment_fault_reports（设备故障报修表）
   - 故障报修管理
   - 故障分级：停机/严重/一般/轻微
   - 响应时效跟踪
   - 图片视频记录

4. equipment_repair_orders（设备维修工单表）
   - 完整的维修流程管理
   - 故障原因分析
   - 维修成本核算
   - 重复故障标识
   - 改进和预防措施

5. equipment_inspections（设备点检表）
   - TPM自主保全
   - 日常点检、定期点检、专项点检
   - 异常快速响应
   - 点检结果统计

6. equipment_inspection_standards（设备点检标准表）
   - 点检标准化管理
   - 支持设备级和分类级标准
   - 版本管理

7. equipment_spare_parts（设备备件库存表）
   - 备件库存管理
   - 安全库存预警
   - 关键备件标识
   - 适用设备匹配

8. spare_parts_transactions（备件出入库记录表）
   - 完整的备件流水记录
   - 与维修维护单据关联
   - 成本追溯

9. equipment_running_records（设备运行记录表）
   - 设备运行日志
   - OEE计算基础数据
   - 停机原因分析
   - 能耗记录

10. equipment_status_monitoring（设备状态监控表）
    - 实时状态监控
    - IoT传感器数据
    - 异常预警
    - 多参数监控

11. equipment_technical_files（设备技术档案表）
    - 技术文档管理
    - 说明书、图纸、规程
    - 版本控制

12. equipment_depreciation_records（设备折旧记录表）
    - 设备资产管理
    - 多种折旧方法
    - 净值计算

13. equipment_energy_consumption（设备能耗统计表）
    - 能耗精细化管理
    - 多种能源类型
    - 单位能耗分析
    - 成本核算

14. equipment_kpi（设备KPI指标表）
    - 全面的设备管理指标
    - OEE、MTBF、MTTR
    - 利用率、可用率
    - 维护成本分析

业务流程：

预防性维护流程：
1. 制定维护计划
2. 自动生成维护任务
3. 提前提醒
4. 执行维护
5. 记录维护结果
6. 验收评价
7. 更新下次维护时间

故障维修流程：
1. 发现故障并报修
2. 接单派工
3. 故障诊断
4. 制定维修方案
5. 领用备件
6. 执行维修
7. 验收测试
8. 记录归档

点检流程：
1. 按标准执行点检
2. 记录点检结果
3. 发现异常
4. 判断是否需要报修
5. 紧急处理或转维修

备件管理流程：
1. 设定安全库存
2. 库存预警
3. 申请采购
4. 入库登记
5. 维修领用
6. 使用记录
7. 库存盘点

关键指标说明：

OEE（设备综合效率）：
- OEE = 可用率 × 性能率 × 质量率
- 可用率 = (计划时间 - 停机时间) / 计划时间
- 性能率 = (实际产量 × 理论节拍) / 运行时间
- 质量率 = 合格数量 / 总产量

MTBF（平均故障间隔时间）：
- MTBF = 总运行时间 / 故障次数

MTTR（平均修复时间）：
- MTTR = 总维修时间 / 故障次数

利用率：
- 利用率 = 运行时间 / 总时间 × 100%

可用率：
- 可用率 = (总时间 - 停机时间 - 维护时间) / 总时间 × 100%

TPM（全面生产维护）八大支柱：
1. 自主保全（点检）
2. 计划保全（维护计划）
3. 个别改善
4. 初期管理
5. 品质保全
6. 教育训练
7. 事务改善
8. 安全环境

数据特点：
1. 所有表都包含软删除字段
2. 所有表都包含完整的审计字段
3. 关键字段都建立了索引
4. 支持完整的设备全生命周期管理
5. 丰富的图片、附件支持

使用建议：
1. 建立完善的维护计划体系，预防大于维修
2. 点检要日常化、标准化，及早发现问题
3. 故障要详细记录，建立故障知识库
4. 备件要合理储备，关键备件不能断
5. 定期分析设备KPI，持续改进
6. 重视能耗管理，降低运营成本
7. 建立设备档案，完整记录设备历史
8. 推行TPM管理，提升设备综合效率
9. 利用IoT技术，实现设备智能监控
10. 建立CMMS（计算机化维护管理系统）
*/
