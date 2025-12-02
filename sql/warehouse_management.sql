-- =============================================
-- MES系统 - 仓储管理模块数据库表结构
-- 创建日期: 2025-10-28
-- 数据库: MySQL 5.7+
-- =============================================

-- 设置字符集
SET NAMES utf8mb4;
SET FOREIGN_KEY_CHECKS = 0;

-- =============================================
-- 1. 库存主表 (inventory)
-- =============================================
DROP TABLE IF EXISTS `inventory`;
CREATE TABLE `inventory` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '库存ID',
  `material_id` bigint(20) NOT NULL COMMENT '物料ID',
  `warehouse_id` bigint(20) NOT NULL COMMENT '仓库ID',
  `location_id` bigint(20) DEFAULT NULL COMMENT '库位ID',
  `batch_no` varchar(50) DEFAULT NULL COMMENT '批次号',
  `serial_no` varchar(50) DEFAULT NULL COMMENT '序列号',
  `quantity` decimal(12,4) NOT NULL DEFAULT 0.0000 COMMENT '库存数量',
  `available_quantity` decimal(12,4) NOT NULL DEFAULT 0.0000 COMMENT '可用数量',
  `locked_quantity` decimal(12,4) DEFAULT 0.0000 COMMENT '锁定数量',
  `allocated_quantity` decimal(12,4) DEFAULT 0.0000 COMMENT '已分配数量',
  `unit` varchar(20) NOT NULL COMMENT '单位',
  `unit_cost` decimal(12,4) DEFAULT 0.0000 COMMENT '单位成本',
  `total_cost` decimal(15,2) DEFAULT 0.00 COMMENT '总成本',
  `production_date` date DEFAULT NULL COMMENT '生产日期',
  `receipt_date` date DEFAULT NULL COMMENT '入库日期',
  `expiry_date` date DEFAULT NULL COMMENT '到期日期',
  `supplier_id` bigint(20) DEFAULT NULL COMMENT '供应商ID',
  `quality_status` tinyint(1) DEFAULT 1 COMMENT '质量状态：1-合格，2-待检，3-不合格，4-冻结',
  `stock_status` tinyint(1) DEFAULT 1 COMMENT '库存状态：1-正常，2-预警，3-短缺',
  `last_in_time` datetime DEFAULT NULL COMMENT '最后入库时间',
  `last_out_time` datetime DEFAULT NULL COMMENT '最后出库时间',
  `remark` varchar(500) DEFAULT NULL COMMENT '备注',
  `created_by` bigint(20) DEFAULT NULL COMMENT '创建人ID',
  `created_time` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `updated_by` bigint(20) DEFAULT NULL COMMENT '更新人ID',
  `updated_time` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  `is_deleted` tinyint(1) DEFAULT 0 COMMENT '是否删除：0-否，1-是',
  PRIMARY KEY (`id`),
  UNIQUE KEY `uk_inventory` (`material_id`, `warehouse_id`, `location_id`, `batch_no`, `serial_no`),
  KEY `idx_material_id` (`material_id`),
  KEY `idx_warehouse_id` (`warehouse_id`),
  KEY `idx_location_id` (`location_id`),
  KEY `idx_batch_no` (`batch_no`),
  KEY `idx_quality_status` (`quality_status`),
  KEY `idx_stock_status` (`stock_status`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='库存主表';

-- =============================================
-- 2. 入库单主表 (inbound_orders)
-- =============================================
DROP TABLE IF EXISTS `inbound_orders`;
CREATE TABLE `inbound_orders` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '入库单ID',
  `inbound_no` varchar(50) NOT NULL COMMENT '入库单号',
  `inbound_type` tinyint(1) NOT NULL COMMENT '入库类型：1-采购入库，2-生产入库，3-退货入库，4-调拨入库，5-其他入库',
  `source_order_no` varchar(50) DEFAULT NULL COMMENT '来源单号（采购单、生产单等）',
  `warehouse_id` bigint(20) NOT NULL COMMENT '入库仓库ID',
  `supplier_id` bigint(20) DEFAULT NULL COMMENT '供应商ID（采购入库时必填）',
  `delivery_no` varchar(50) DEFAULT NULL COMMENT '送货单号',
  `plan_inbound_date` date DEFAULT NULL COMMENT '计划入库日期',
  `actual_inbound_date` date DEFAULT NULL COMMENT '实际入库日期',
  `total_quantity` decimal(12,4) DEFAULT 0.0000 COMMENT '总数量',
  `total_amount` decimal(15,2) DEFAULT 0.00 COMMENT '总金额',
  `handler_id` bigint(20) DEFAULT NULL COMMENT '经办人ID',
  `receiver_id` bigint(20) DEFAULT NULL COMMENT '收货人ID',
  `inspector_id` bigint(20) DEFAULT NULL COMMENT '质检员ID',
  `inspect_result` tinyint(1) DEFAULT NULL COMMENT '质检结果：1-合格，2-不合格，3-让步接收',
  `order_status` tinyint(1) DEFAULT 1 COMMENT '单据状态：1-待入库，2-部分入库，3-已完成，4-已取消',
  `is_urgent` tinyint(1) DEFAULT 0 COMMENT '是否加急：0-否，1-是',
  `dept_id` bigint(20) DEFAULT NULL COMMENT '部门ID',
  `remark` varchar(500) DEFAULT NULL COMMENT '备注',
  `attachment_url` varchar(500) DEFAULT NULL COMMENT '附件URL',
  `created_by` bigint(20) DEFAULT NULL COMMENT '创建人ID',
  `created_time` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `updated_by` bigint(20) DEFAULT NULL COMMENT '更新人ID',
  `updated_time` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  `is_deleted` tinyint(1) DEFAULT 0 COMMENT '是否删除：0-否，1-是',
  PRIMARY KEY (`id`),
  UNIQUE KEY `uk_inbound_no` (`inbound_no`),
  KEY `idx_inbound_type` (`inbound_type`),
  KEY `idx_warehouse_id` (`warehouse_id`),
  KEY `idx_supplier_id` (`supplier_id`),
  KEY `idx_order_status` (`order_status`),
  KEY `idx_plan_date` (`plan_inbound_date`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='入库单主表';

-- =============================================
-- 3. 入库单明细表 (inbound_order_details)
-- =============================================
DROP TABLE IF EXISTS `inbound_order_details`;
CREATE TABLE `inbound_order_details` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '明细ID',
  `inbound_order_id` bigint(20) NOT NULL COMMENT '入库单ID',
  `material_id` bigint(20) NOT NULL COMMENT '物料ID',
  `location_id` bigint(20) DEFAULT NULL COMMENT '库位ID',
  `batch_no` varchar(50) DEFAULT NULL COMMENT '批次号',
  `serial_no` varchar(50) DEFAULT NULL COMMENT '序列号',
  `plan_quantity` decimal(12,4) NOT NULL COMMENT '计划数量',
  `actual_quantity` decimal(12,4) DEFAULT 0.0000 COMMENT '实际数量',
  `qualified_quantity` decimal(12,4) DEFAULT 0.0000 COMMENT '合格数量',
  `unqualified_quantity` decimal(12,4) DEFAULT 0.0000 COMMENT '不合格数量',
  `unit` varchar(20) NOT NULL COMMENT '单位',
  `unit_price` decimal(12,4) DEFAULT 0.0000 COMMENT '单价',
  `amount` decimal(15,2) DEFAULT 0.00 COMMENT '金额',
  `production_date` date DEFAULT NULL COMMENT '生产日期',
  `expiry_date` date DEFAULT NULL COMMENT '到期日期',
  `quality_status` tinyint(1) DEFAULT 1 COMMENT '质量状态：1-合格，2-待检，3-不合格',
  `line_status` tinyint(1) DEFAULT 1 COMMENT '明细状态：1-待入库，2-已入库',
  `remark` varchar(500) DEFAULT NULL COMMENT '备注',
  `created_by` bigint(20) DEFAULT NULL COMMENT '创建人ID',
  `created_time` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `updated_by` bigint(20) DEFAULT NULL COMMENT '更新人ID',
  `updated_time` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  `is_deleted` tinyint(1) DEFAULT 0 COMMENT '是否删除：0-否，1-是',
  PRIMARY KEY (`id`),
  KEY `idx_inbound_order_id` (`inbound_order_id`),
  KEY `idx_material_id` (`material_id`),
  KEY `idx_batch_no` (`batch_no`),
  KEY `idx_line_status` (`line_status`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='入库单明细表';

-- =============================================
-- 4. 出库单主表 (outbound_orders)
-- =============================================
DROP TABLE IF EXISTS `outbound_orders`;
CREATE TABLE `outbound_orders` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '出库单ID',
  `outbound_no` varchar(50) NOT NULL COMMENT '出库单号',
  `outbound_type` tinyint(1) NOT NULL COMMENT '出库类型：1-销售出库，2-生产领料，3-调拨出库，4-报废出库，5-其他出库',
  `source_order_no` varchar(50) DEFAULT NULL COMMENT '来源单号（销售单、生产单等）',
  `warehouse_id` bigint(20) NOT NULL COMMENT '出库仓库ID',
  `customer_id` bigint(20) DEFAULT NULL COMMENT '客户ID（销售出库时必填）',
  `receiver_name` varchar(100) DEFAULT NULL COMMENT '收货人姓名',
  `receiver_phone` varchar(20) DEFAULT NULL COMMENT '收货人电话',
  `receiver_address` varchar(300) DEFAULT NULL COMMENT '收货地址',
  `plan_outbound_date` date DEFAULT NULL COMMENT '计划出库日期',
  `actual_outbound_date` date DEFAULT NULL COMMENT '实际出库日期',
  `total_quantity` decimal(12,4) DEFAULT 0.0000 COMMENT '总数量',
  `total_amount` decimal(15,2) DEFAULT 0.00 COMMENT '总金额',
  `handler_id` bigint(20) DEFAULT NULL COMMENT '经办人ID',
  `picker_id` bigint(20) DEFAULT NULL COMMENT '拣货人ID',
  `reviewer_id` bigint(20) DEFAULT NULL COMMENT '复核人ID',
  `shipper_id` bigint(20) DEFAULT NULL COMMENT '发货人ID',
  `order_status` tinyint(1) DEFAULT 1 COMMENT '单据状态：1-待拣货，2-拣货中，3-待复核，4-待出库，5-部分出库，6-已完成，7-已取消',
  `is_urgent` tinyint(1) DEFAULT 0 COMMENT '是否加急：0-否，1-是',
  `dept_id` bigint(20) DEFAULT NULL COMMENT '部门ID',
  `workshop_id` bigint(20) DEFAULT NULL COMMENT '车间ID（生产领料）',
  `cost_center` varchar(50) DEFAULT NULL COMMENT '成本中心',
  `logistics_company` varchar(100) DEFAULT NULL COMMENT '物流公司',
  `tracking_no` varchar(100) DEFAULT NULL COMMENT '物流单号',
  `remark` varchar(500) DEFAULT NULL COMMENT '备注',
  `attachment_url` varchar(500) DEFAULT NULL COMMENT '附件URL',
  `created_by` bigint(20) DEFAULT NULL COMMENT '创建人ID',
  `created_time` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `updated_by` bigint(20) DEFAULT NULL COMMENT '更新人ID',
  `updated_time` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  `is_deleted` tinyint(1) DEFAULT 0 COMMENT '是否删除：0-否，1-是',
  PRIMARY KEY (`id`),
  UNIQUE KEY `uk_outbound_no` (`outbound_no`),
  KEY `idx_outbound_type` (`outbound_type`),
  KEY `idx_warehouse_id` (`warehouse_id`),
  KEY `idx_customer_id` (`customer_id`),
  KEY `idx_order_status` (`order_status`),
  KEY `idx_plan_date` (`plan_outbound_date`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='出库单主表';

-- =============================================
-- 5. 出库单明细表 (outbound_order_details)
-- =============================================
DROP TABLE IF EXISTS `outbound_order_details`;
CREATE TABLE `outbound_order_details` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '明细ID',
  `outbound_order_id` bigint(20) NOT NULL COMMENT '出库单ID',
  `material_id` bigint(20) NOT NULL COMMENT '物料ID',
  `location_id` bigint(20) DEFAULT NULL COMMENT '库位ID',
  `batch_no` varchar(50) DEFAULT NULL COMMENT '批次号',
  `serial_no` varchar(50) DEFAULT NULL COMMENT '序列号',
  `plan_quantity` decimal(12,4) NOT NULL COMMENT '计划数量',
  `picked_quantity` decimal(12,4) DEFAULT 0.0000 COMMENT '拣货数量',
  `actual_quantity` decimal(12,4) DEFAULT 0.0000 COMMENT '实际出库数量',
  `unit` varchar(20) NOT NULL COMMENT '单位',
  `unit_price` decimal(12,4) DEFAULT 0.0000 COMMENT '单价',
  `amount` decimal(15,2) DEFAULT 0.00 COMMENT '金额',
  `production_date` date DEFAULT NULL COMMENT '生产日期',
  `expiry_date` date DEFAULT NULL COMMENT '到期日期',
  `line_status` tinyint(1) DEFAULT 1 COMMENT '明细状态：1-待拣货，2-已拣货，3-已出库',
  `remark` varchar(500) DEFAULT NULL COMMENT '备注',
  `created_by` bigint(20) DEFAULT NULL COMMENT '创建人ID',
  `created_time` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `updated_by` bigint(20) DEFAULT NULL COMMENT '更新人ID',
  `updated_time` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  `is_deleted` tinyint(1) DEFAULT 0 COMMENT '是否删除：0-否，1-是',
  PRIMARY KEY (`id`),
  KEY `idx_outbound_order_id` (`outbound_order_id`),
  KEY `idx_material_id` (`material_id`),
  KEY `idx_batch_no` (`batch_no`),
  KEY `idx_line_status` (`line_status`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='出库单明细表';

-- =============================================
-- 6. 库存调拨单主表 (transfer_orders)
-- =============================================
DROP TABLE IF EXISTS `transfer_orders`;
CREATE TABLE `transfer_orders` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '调拨单ID',
  `transfer_no` varchar(50) NOT NULL COMMENT '调拨单号',
  `transfer_type` tinyint(1) NOT NULL COMMENT '调拨类型：1-仓库间调拨，2-库位间调拨，3-批次调整',
  `from_warehouse_id` bigint(20) NOT NULL COMMENT '调出仓库ID',
  `to_warehouse_id` bigint(20) NOT NULL COMMENT '调入仓库ID',
  `plan_transfer_date` date DEFAULT NULL COMMENT '计划调拨日期',
  `actual_transfer_date` date DEFAULT NULL COMMENT '实际调拨日期',
  `total_quantity` decimal(12,4) DEFAULT 0.0000 COMMENT '总数量',
  `handler_id` bigint(20) DEFAULT NULL COMMENT '经办人ID',
  `out_handler_id` bigint(20) DEFAULT NULL COMMENT '调出经办人ID',
  `in_handler_id` bigint(20) DEFAULT NULL COMMENT '调入经办人ID',
  `order_status` tinyint(1) DEFAULT 1 COMMENT '单据状态：1-待审核，2-待调出，3-待调入，4-已完成，5-已取消',
  `reason` varchar(500) DEFAULT NULL COMMENT '调拨原因',
  `remark` varchar(500) DEFAULT NULL COMMENT '备注',
  `created_by` bigint(20) DEFAULT NULL COMMENT '创建人ID',
  `created_time` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `updated_by` bigint(20) DEFAULT NULL COMMENT '更新人ID',
  `updated_time` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  `is_deleted` tinyint(1) DEFAULT 0 COMMENT '是否删除：0-否，1-是',
  PRIMARY KEY (`id`),
  UNIQUE KEY `uk_transfer_no` (`transfer_no`),
  KEY `idx_from_warehouse` (`from_warehouse_id`),
  KEY `idx_to_warehouse` (`to_warehouse_id`),
  KEY `idx_order_status` (`order_status`),
  KEY `idx_plan_date` (`plan_transfer_date`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='库存调拨单主表';

-- =============================================
-- 7. 库存调拨单明细表 (transfer_order_details)
-- =============================================
DROP TABLE IF EXISTS `transfer_order_details`;
CREATE TABLE `transfer_order_details` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '明细ID',
  `transfer_order_id` bigint(20) NOT NULL COMMENT '调拨单ID',
  `material_id` bigint(20) NOT NULL COMMENT '物料ID',
  `from_location_id` bigint(20) DEFAULT NULL COMMENT '调出库位ID',
  `to_location_id` bigint(20) DEFAULT NULL COMMENT '调入库位ID',
  `batch_no` varchar(50) DEFAULT NULL COMMENT '批次号',
  `serial_no` varchar(50) DEFAULT NULL COMMENT '序列号',
  `transfer_quantity` decimal(12,4) NOT NULL COMMENT '调拨数量',
  `actual_quantity` decimal(12,4) DEFAULT 0.0000 COMMENT '实际数量',
  `unit` varchar(20) NOT NULL COMMENT '单位',
  `line_status` tinyint(1) DEFAULT 1 COMMENT '明细状态：1-待调拨，2-已调出，3-已调入',
  `remark` varchar(500) DEFAULT NULL COMMENT '备注',
  `created_by` bigint(20) DEFAULT NULL COMMENT '创建人ID',
  `created_time` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `updated_by` bigint(20) DEFAULT NULL COMMENT '更新人ID',
  `updated_time` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  `is_deleted` tinyint(1) DEFAULT 0 COMMENT '是否删除：0-否，1-是',
  PRIMARY KEY (`id`),
  KEY `idx_transfer_order_id` (`transfer_order_id`),
  KEY `idx_material_id` (`material_id`),
  KEY `idx_line_status` (`line_status`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='库存调拨单明细表';

-- =============================================
-- 8. 库存盘点单主表 (inventory_check_orders)
-- =============================================
DROP TABLE IF EXISTS `inventory_check_orders`;
CREATE TABLE `inventory_check_orders` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '盘点单ID',
  `check_no` varchar(50) NOT NULL COMMENT '盘点单号',
  `check_type` tinyint(1) NOT NULL COMMENT '盘点类型：1-全盘，2-抽盘，3-循环盘点，4-动态盘点',
  `warehouse_id` bigint(20) NOT NULL COMMENT '盘点仓库ID',
  `check_date` date NOT NULL COMMENT '盘点日期',
  `check_start_time` datetime DEFAULT NULL COMMENT '盘点开始时间',
  `check_end_time` datetime DEFAULT NULL COMMENT '盘点结束时间',
  `total_items` int(11) DEFAULT 0 COMMENT '盘点物料种类数',
  `checked_items` int(11) DEFAULT 0 COMMENT '已盘点物料种类数',
  `difference_items` int(11) DEFAULT 0 COMMENT '有差异物料种类数',
  `supervisor_id` bigint(20) DEFAULT NULL COMMENT '盘点主管ID',
  `check_team` varchar(500) DEFAULT NULL COMMENT '盘点小组成员（JSON格式）',
  `order_status` tinyint(1) DEFAULT 1 COMMENT '单据状态：1-待盘点，2-盘点中，3-待复盘，4-待审核，5-已完成，6-已取消',
  `is_locked` tinyint(1) DEFAULT 0 COMMENT '是否锁定库存：0-否，1-是',
  `remark` varchar(500) DEFAULT NULL COMMENT '备注',
  `created_by` bigint(20) DEFAULT NULL COMMENT '创建人ID',
  `created_time` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `updated_by` bigint(20) DEFAULT NULL COMMENT '更新人ID',
  `updated_time` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  `is_deleted` tinyint(1) DEFAULT 0 COMMENT '是否删除：0-否，1-是',
  PRIMARY KEY (`id`),
  UNIQUE KEY `uk_check_no` (`check_no`),
  KEY `idx_check_type` (`check_type`),
  KEY `idx_warehouse_id` (`warehouse_id`),
  KEY `idx_order_status` (`order_status`),
  KEY `idx_check_date` (`check_date`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='库存盘点单主表';

-- =============================================
-- 9. 库存盘点单明细表 (inventory_check_details)
-- =============================================
DROP TABLE IF EXISTS `inventory_check_details`;
CREATE TABLE `inventory_check_details` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '明细ID',
  `check_order_id` bigint(20) NOT NULL COMMENT '盘点单ID',
  `material_id` bigint(20) NOT NULL COMMENT '物料ID',
  `location_id` bigint(20) DEFAULT NULL COMMENT '库位ID',
  `batch_no` varchar(50) DEFAULT NULL COMMENT '批次号',
  `serial_no` varchar(50) DEFAULT NULL COMMENT '序列号',
  `book_quantity` decimal(12,4) DEFAULT 0.0000 COMMENT '账面数量',
  `actual_quantity` decimal(12,4) DEFAULT NULL COMMENT '实际数量',
  `difference_quantity` decimal(12,4) DEFAULT 0.0000 COMMENT '差异数量',
  `unit` varchar(20) NOT NULL COMMENT '单位',
  `unit_cost` decimal(12,4) DEFAULT 0.0000 COMMENT '单位成本',
  `difference_amount` decimal(15,2) DEFAULT 0.00 COMMENT '差异金额',
  `check_status` tinyint(1) DEFAULT 1 COMMENT '盘点状态：1-待盘点，2-已盘点，3-有差异，4-已调整',
  `checker_id` bigint(20) DEFAULT NULL COMMENT '盘点人ID',
  `check_time` datetime DEFAULT NULL COMMENT '盘点时间',
  `recheck_quantity` decimal(12,4) DEFAULT NULL COMMENT '复盘数量',
  `rechecker_id` bigint(20) DEFAULT NULL COMMENT '复盘人ID',
  `recheck_time` datetime DEFAULT NULL COMMENT '复盘时间',
  `difference_reason` varchar(500) DEFAULT NULL COMMENT '差异原因',
  `adjustment_status` tinyint(1) DEFAULT 0 COMMENT '调整状态：0-未调整，1-已调整',
  `remark` varchar(500) DEFAULT NULL COMMENT '备注',
  `created_by` bigint(20) DEFAULT NULL COMMENT '创建人ID',
  `created_time` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `updated_by` bigint(20) DEFAULT NULL COMMENT '更新人ID',
  `updated_time` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  `is_deleted` tinyint(1) DEFAULT 0 COMMENT '是否删除：0-否，1-是',
  PRIMARY KEY (`id`),
  KEY `idx_check_order_id` (`check_order_id`),
  KEY `idx_material_id` (`material_id`),
  KEY `idx_check_status` (`check_status`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='库存盘点单明细表';

-- =============================================
-- 10. 库存调整单主表 (inventory_adjustment_orders)
-- =============================================
DROP TABLE IF EXISTS `inventory_adjustment_orders`;
CREATE TABLE `inventory_adjustment_orders` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '调整单ID',
  `adjustment_no` varchar(50) NOT NULL COMMENT '调整单号',
  `adjustment_type` tinyint(1) NOT NULL COMMENT '调整类型：1-盘盈，2-盘亏，3-报损，4-报溢，5-其他调整',
  `warehouse_id` bigint(20) NOT NULL COMMENT '仓库ID',
  `check_order_id` bigint(20) DEFAULT NULL COMMENT '关联盘点单ID',
  `adjustment_date` date NOT NULL COMMENT '调整日期',
  `total_quantity` decimal(12,4) DEFAULT 0.0000 COMMENT '调整总数量',
  `total_amount` decimal(15,2) DEFAULT 0.00 COMMENT '调整总金额',
  `handler_id` bigint(20) DEFAULT NULL COMMENT '经办人ID',
  `approver_id` bigint(20) DEFAULT NULL COMMENT '审批人ID',
  `approval_time` datetime DEFAULT NULL COMMENT '审批时间',
  `order_status` tinyint(1) DEFAULT 1 COMMENT '单据状态：1-待审核，2-已审核，3-已执行，4-已取消',
  `reason` varchar(500) DEFAULT NULL COMMENT '调整原因',
  `remark` varchar(500) DEFAULT NULL COMMENT '备注',
  `created_by` bigint(20) DEFAULT NULL COMMENT '创建人ID',
  `created_time` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `updated_by` bigint(20) DEFAULT NULL COMMENT '更新人ID',
  `updated_time` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  `is_deleted` tinyint(1) DEFAULT 0 COMMENT '是否删除：0-否，1-是',
  PRIMARY KEY (`id`),
  UNIQUE KEY `uk_adjustment_no` (`adjustment_no`),
  KEY `idx_adjustment_type` (`adjustment_type`),
  KEY `idx_warehouse_id` (`warehouse_id`),
  KEY `idx_check_order_id` (`check_order_id`),
  KEY `idx_order_status` (`order_status`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='库存调整单主表';

-- =============================================
-- 11. 库存调整单明细表 (inventory_adjustment_details)
-- =============================================
DROP TABLE IF EXISTS `inventory_adjustment_details`;
CREATE TABLE `inventory_adjustment_details` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '明细ID',
  `adjustment_order_id` bigint(20) NOT NULL COMMENT '调整单ID',
  `material_id` bigint(20) NOT NULL COMMENT '物料ID',
  `location_id` bigint(20) DEFAULT NULL COMMENT '库位ID',
  `batch_no` varchar(50) DEFAULT NULL COMMENT '批次号',
  `serial_no` varchar(50) DEFAULT NULL COMMENT '序列号',
  `before_quantity` decimal(12,4) DEFAULT 0.0000 COMMENT '调整前数量',
  `adjustment_quantity` decimal(12,4) NOT NULL COMMENT '调整数量（正数为增加，负数为减少）',
  `after_quantity` decimal(12,4) DEFAULT 0.0000 COMMENT '调整后数量',
  `unit` varchar(20) NOT NULL COMMENT '单位',
  `unit_cost` decimal(12,4) DEFAULT 0.0000 COMMENT '单位成本',
  `adjustment_amount` decimal(15,2) DEFAULT 0.00 COMMENT '调整金额',
  `remark` varchar(500) DEFAULT NULL COMMENT '备注',
  `created_by` bigint(20) DEFAULT NULL COMMENT '创建人ID',
  `created_time` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `updated_by` bigint(20) DEFAULT NULL COMMENT '更新人ID',
  `updated_time` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  `is_deleted` tinyint(1) DEFAULT 0 COMMENT '是否删除：0-否，1-是',
  PRIMARY KEY (`id`),
  KEY `idx_adjustment_order_id` (`adjustment_order_id`),
  KEY `idx_material_id` (`material_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='库存调整单明细表';

-- =============================================
-- 12. 库存预留表 (inventory_reservations)
-- =============================================
DROP TABLE IF EXISTS `inventory_reservations`;
CREATE TABLE `inventory_reservations` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '预留ID',
  `reservation_no` varchar(50) NOT NULL COMMENT '预留单号',
  `reservation_type` tinyint(1) NOT NULL COMMENT '预留类型：1-销售订单预留，2-生产订单预留，3-其他预留',
  `source_order_no` varchar(50) NOT NULL COMMENT '来源单号',
  `material_id` bigint(20) NOT NULL COMMENT '物料ID',
  `warehouse_id` bigint(20) NOT NULL COMMENT '仓库ID',
  `location_id` bigint(20) DEFAULT NULL COMMENT '库位ID',
  `batch_no` varchar(50) DEFAULT NULL COMMENT '批次号',
  `reserved_quantity` decimal(12,4) NOT NULL COMMENT '预留数量',
  `used_quantity` decimal(12,4) DEFAULT 0.0000 COMMENT '已使用数量',
  `remaining_quantity` decimal(12,4) DEFAULT 0.0000 COMMENT '剩余数量',
  `unit` varchar(20) NOT NULL COMMENT '单位',
  `reservation_date` date NOT NULL COMMENT '预留日期',
  `expiry_date` date DEFAULT NULL COMMENT '预留到期日期',
  `status` tinyint(1) DEFAULT 1 COMMENT '状态：1-已预留，2-部分使用，3-已使用，4-已取消，5-已过期',
  `handler_id` bigint(20) DEFAULT NULL COMMENT '经办人ID',
  `remark` varchar(500) DEFAULT NULL COMMENT '备注',
  `created_by` bigint(20) DEFAULT NULL COMMENT '创建人ID',
  `created_time` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `updated_by` bigint(20) DEFAULT NULL COMMENT '更新人ID',
  `updated_time` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  `is_deleted` tinyint(1) DEFAULT 0 COMMENT '是否删除：0-否，1-是',
  PRIMARY KEY (`id`),
  UNIQUE KEY `uk_reservation_no` (`reservation_no`),
  KEY `idx_material_id` (`material_id`),
  KEY `idx_warehouse_id` (`warehouse_id`),
  KEY `idx_source_order` (`source_order_no`),
  KEY `idx_status` (`status`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='库存预留表';

-- =============================================
-- 13. 库存交易流水表 (inventory_transactions)
-- =============================================
DROP TABLE IF EXISTS `inventory_transactions`;
CREATE TABLE `inventory_transactions` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '流水ID',
  `transaction_no` varchar(50) NOT NULL COMMENT '流水号',
  `transaction_type` tinyint(1) NOT NULL COMMENT '交易类型：1-入库，2-出库，3-调拨出，4-调拨入，5-盘盈，6-盘亏，7-锁定，8-解锁',
  `business_type` varchar(50) DEFAULT NULL COMMENT '业务类型：采购入库/生产入库/销售出库/生产领料等',
  `order_no` varchar(50) DEFAULT NULL COMMENT '单据号',
  `material_id` bigint(20) NOT NULL COMMENT '物料ID',
  `warehouse_id` bigint(20) NOT NULL COMMENT '仓库ID',
  `location_id` bigint(20) DEFAULT NULL COMMENT '库位ID',
  `batch_no` varchar(50) DEFAULT NULL COMMENT '批次号',
  `serial_no` varchar(50) DEFAULT NULL COMMENT '序列号',
  `transaction_quantity` decimal(12,4) NOT NULL COMMENT '交易数量（正数为增加，负数为减少）',
  `before_quantity` decimal(12,4) DEFAULT 0.0000 COMMENT '交易前数量',
  `after_quantity` decimal(12,4) DEFAULT 0.0000 COMMENT '交易后数量',
  `unit` varchar(20) NOT NULL COMMENT '单位',
  `unit_cost` decimal(12,4) DEFAULT 0.0000 COMMENT '单位成本',
  `transaction_amount` decimal(15,2) DEFAULT 0.00 COMMENT '交易金额',
  `transaction_date` datetime NOT NULL COMMENT '交易时间',
  `handler_id` bigint(20) DEFAULT NULL COMMENT '经办人ID',
  `remark` varchar(500) DEFAULT NULL COMMENT '备注',
  `created_by` bigint(20) DEFAULT NULL COMMENT '创建人ID',
  `created_time` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  PRIMARY KEY (`id`),
  UNIQUE KEY `uk_transaction_no` (`transaction_no`),
  KEY `idx_material_id` (`material_id`),
  KEY `idx_warehouse_id` (`warehouse_id`),
  KEY `idx_transaction_type` (`transaction_type`),
  KEY `idx_order_no` (`order_no`),
  KEY `idx_transaction_date` (`transaction_date`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='库存交易流水表';

-- =============================================
-- 14. 库存预警规则表 (inventory_alert_rules)
-- =============================================
DROP TABLE IF EXISTS `inventory_alert_rules`;
CREATE TABLE `inventory_alert_rules` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '规则ID',
  `rule_code` varchar(50) NOT NULL COMMENT '规则编码',
  `rule_name` varchar(100) NOT NULL COMMENT '规则名称',
  `material_id` bigint(20) DEFAULT NULL COMMENT '物料ID（为空表示通用规则）',
  `warehouse_id` bigint(20) DEFAULT NULL COMMENT '仓库ID（为空表示所有仓库）',
  `alert_type` tinyint(1) NOT NULL COMMENT '预警类型：1-库存不足，2-库存超量，3-临期预警，4-长期不动',
  `min_quantity` decimal(12,4) DEFAULT NULL COMMENT '最小库存量',
  `max_quantity` decimal(12,4) DEFAULT NULL COMMENT '最大库存量',
  `alert_days` int(11) DEFAULT NULL COMMENT '预警天数（用于临期预警、不动预警）',
  `alert_level` tinyint(1) DEFAULT 1 COMMENT '预警级别：1-一般，2-重要，3-紧急',
  `is_send_email` tinyint(1) DEFAULT 0 COMMENT '是否发送邮件：0-否，1-是',
  `is_send_sms` tinyint(1) DEFAULT 0 COMMENT '是否发送短信：0-否，1-是',
  `recipient_ids` varchar(500) DEFAULT NULL COMMENT '接收人ID列表（JSON格式）',
  `status` tinyint(1) DEFAULT 1 COMMENT '状态：0-停用，1-启用',
  `remark` varchar(500) DEFAULT NULL COMMENT '备注',
  `created_by` bigint(20) DEFAULT NULL COMMENT '创建人ID',
  `created_time` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `updated_by` bigint(20) DEFAULT NULL COMMENT '更新人ID',
  `updated_time` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  `is_deleted` tinyint(1) DEFAULT 0 COMMENT '是否删除：0-否，1-是',
  PRIMARY KEY (`id`),
  UNIQUE KEY `uk_rule_code` (`rule_code`),
  KEY `idx_material_id` (`material_id`),
  KEY `idx_warehouse_id` (`warehouse_id`),
  KEY `idx_status` (`status`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='库存预警规则表';

-- =============================================
-- 15. 库存预警记录表 (inventory_alert_records)
-- =============================================
DROP TABLE IF EXISTS `inventory_alert_records`;
CREATE TABLE `inventory_alert_records` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '记录ID',
  `rule_id` bigint(20) NOT NULL COMMENT '规则ID',
  `material_id` bigint(20) NOT NULL COMMENT '物料ID',
  `warehouse_id` bigint(20) NOT NULL COMMENT '仓库ID',
  `alert_type` tinyint(1) NOT NULL COMMENT '预警类型：1-库存不足，2-库存超量，3-临期预警，4-长期不动',
  `alert_level` tinyint(1) DEFAULT 1 COMMENT '预警级别：1-一般，2-重要，3-紧急',
  `current_quantity` decimal(12,4) DEFAULT 0.0000 COMMENT '当前数量',
  `threshold_quantity` decimal(12,4) DEFAULT NULL COMMENT '阈值数量',
  `alert_content` varchar(500) DEFAULT NULL COMMENT '预警内容',
  `alert_time` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '预警时间',
  `is_handled` tinyint(1) DEFAULT 0 COMMENT '是否已处理：0-未处理，1-已处理',
  `handler_id` bigint(20) DEFAULT NULL COMMENT '处理人ID',
  `handle_time` datetime DEFAULT NULL COMMENT '处理时间',
  `handle_result` varchar(500) DEFAULT NULL COMMENT '处理结果',
  `remark` varchar(500) DEFAULT NULL COMMENT '备注',
  `created_time` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  PRIMARY KEY (`id`),
  KEY `idx_rule_id` (`rule_id`),
  KEY `idx_material_id` (`material_id`),
  KEY `idx_warehouse_id` (`warehouse_id`),
  KEY `idx_alert_time` (`alert_time`),
  KEY `idx_is_handled` (`is_handled`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='库存预警记录表';

-- =============================================
-- 16. 批次追溯表 (batch_traceability)
-- =============================================
DROP TABLE IF EXISTS `batch_traceability`;
CREATE TABLE `batch_traceability` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '追溯ID',
  `batch_no` varchar(50) NOT NULL COMMENT '批次号',
  `material_id` bigint(20) NOT NULL COMMENT '物料ID',
  `trace_type` tinyint(1) NOT NULL COMMENT '追溯类型：1-正向追溯，2-反向追溯',
  `parent_batch_no` varchar(50) DEFAULT NULL COMMENT '父批次号',
  `production_order_no` varchar(50) DEFAULT NULL COMMENT '生产订单号',
  `purchase_order_no` varchar(50) DEFAULT NULL COMMENT '采购订单号',
  `supplier_id` bigint(20) DEFAULT NULL COMMENT '供应商ID',
  `supplier_batch_no` varchar(50) DEFAULT NULL COMMENT '供应商批次号',
  `production_date` date DEFAULT NULL COMMENT '生产日期',
  `expiry_date` date DEFAULT NULL COMMENT '到期日期',
  `workshop_id` bigint(20) DEFAULT NULL COMMENT '生产车间ID',
  `production_line` varchar(100) DEFAULT NULL COMMENT '生产线',
  `operator_id` bigint(20) DEFAULT NULL COMMENT '操作员ID',
  `inspector_id` bigint(20) DEFAULT NULL COMMENT '检验员ID',
  `inspect_result` tinyint(1) DEFAULT NULL COMMENT '检验结果：1-合格，2-不合格',
  `quality_certificate_no` varchar(50) DEFAULT NULL COMMENT '质检报告单号',
  `total_quantity` decimal(12,4) DEFAULT 0.0000 COMMENT '批次总数量',
  `unit` varchar(20) DEFAULT NULL COMMENT '单位',
  `status` tinyint(1) DEFAULT 1 COMMENT '状态：1-在库，2-已用完，3-已过期',
  `remark` varchar(500) DEFAULT NULL COMMENT '备注',
  `created_by` bigint(20) DEFAULT NULL COMMENT '创建人ID',
  `created_time` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `updated_by` bigint(20) DEFAULT NULL COMMENT '更新人ID',
  `updated_time` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  `is_deleted` tinyint(1) DEFAULT 0 COMMENT '是否删除：0-否，1-是',
  PRIMARY KEY (`id`),
  UNIQUE KEY `uk_batch_material` (`batch_no`, `material_id`),
  KEY `idx_batch_no` (`batch_no`),
  KEY `idx_material_id` (`material_id`),
  KEY `idx_parent_batch` (`parent_batch_no`),
  KEY `idx_production_order` (`production_order_no`),
  KEY `idx_status` (`status`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='批次追溯表';

SET FOREIGN_KEY_CHECKS = 1;

-- =============================================
-- 初始化数据
-- =============================================

-- 插入库存预警规则示例
INSERT INTO `inventory_alert_rules` (`id`, `rule_code`, `rule_name`, `alert_type`, `min_quantity`, `alert_level`, `status`) VALUES
(1, 'ALERT-001', '通用库存不足预警', 1, 100.0000, 2, 1),
(2, 'ALERT-002', '通用库存超量预警', 2, 10000.0000, 1, 1),
(3, 'ALERT-003', '临期预警（7天）', 3, NULL, 2, 1),
(4, 'ALERT-004', '长期不动预警（90天）', 4, NULL, 1, 1);

-- =============================================
-- 说明文档
-- =============================================
/*
MES系统仓储管理模块表结构说明：

核心表说明：

1. inventory（库存主表）
   - 系统核心表，实时库存数据
   - 支持批次号、序列号管理
   - 包含可用数量、锁定数量、已分配数量
   - 支持质量状态管理（合格/待检/不合格/冻结）
   - 支持库存状态预警（正常/预警/短缺）

2. inbound_orders & inbound_order_details（入库单主表及明细表）
   - 支持多种入库类型：采购入库、生产入库、退货入库、调拨入库等
   - 完整的入库流程管理
   - 支持质检流程
   - 单据状态流转：待入库 -> 部分入库 -> 已完成

3. outbound_orders & outbound_order_details（出库单主表及明细表）
   - 支持多种出库类型：销售出库、生产领料、调拨出库、报废出库等
   - 完整的拣货、复核、发货流程
   - 支持物流信息管理
   - 单据状态流转：待拣货 -> 拣货中 -> 待复核 -> 待出库 -> 已完成

4. transfer_orders & transfer_order_details（调拨单主表及明细表）
   - 支持仓库间调拨、库位间调拨
   - 支持批次调整
   - 双重确认机制（调出确认、调入确认）

5. inventory_check_orders & inventory_check_details（盘点单主表及明细表）
   - 支持全盘、抽盘、循环盘点、动态盘点
   - 支持复盘流程
   - 差异自动计算
   - 盘点期间可锁定库存

6. inventory_adjustment_orders & inventory_adjustment_details（调整单主表及明细表）
   - 库存数量调整（盘盈、盘亏、报损、报溢等）
   - 需要审批流程
   - 自动生成库存交易流水

7. inventory_reservations（库存预留表）
   - 支持销售订单预留、生产订单预留
   - 预留数量管理
   - 支持预留过期管理

8. inventory_transactions（库存交易流水表）
   - 记录所有库存变动
   - 完整的库存变动历史
   - 支持追溯和审计

9. inventory_alert_rules（库存预警规则表）
   - 灵活的预警规则配置
   - 支持多种预警类型：库存不足、库存超量、临期预警、长期不动
   - 支持邮件、短信通知

10. inventory_alert_records（库存预警记录表）
    - 预警记录管理
    - 预警处理跟踪

11. batch_traceability（批次追溯表）
    - 完整的批次追溯信息
    - 支持正向追溯和反向追溯
    - 关联生产、采购、质检信息

业务流程：

入库流程：
1. 创建入库单（待入库）
2. 质检（可选）
3. 入库确认（已完成）
4. 生成库存交易流水
5. 更新库存主表

出库流程：
1. 创建出库单（待拣货）
2. 拣货（拣货中）
3. 复核（待复核）
4. 出库确认（已完成）
5. 生成库存交易流水
6. 更新库存主表

调拨流程：
1. 创建调拨单（待审核）
2. 调出确认（待调入）
3. 调入确认（已完成）
4. 生成库存交易流水
5. 更新库存主表

盘点流程：
1. 创建盘点单（待盘点）
2. 锁定库存（可选）
3. 执行盘点（盘点中）
4. 复盘（待复盘，可选）
5. 审核（待审核）
6. 生成调整单
7. 完成（已完成）

数据特点：
1. 所有表都包含软删除字段
2. 所有表都包含完整的审计字段
3. 关键字段都建立了索引
4. 支持完整的库存追溯
5. 支持实时库存预警

使用建议：
1. 库存主表是核心表，所有入出库操作都要更新此表
2. 库存交易流水表记录所有库存变动，用于审计和追溯
3. 定期执行库存盘点，确保账实相符
4. 合理设置库存预警规则，及时处理异常情况
5. 批次管理物料必须填写批次号
6. 建议定期归档历史数据，保持系统性能
*/
