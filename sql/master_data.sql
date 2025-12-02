-- =============================================
-- MES系统 - 主数据管理模块数据库表结构
-- 创建日期: 2025-10-28
-- 数据库: MySQL 5.7+
-- =============================================

-- 设置字符集
SET NAMES utf8mb4;
SET FOREIGN_KEY_CHECKS = 0;

-- =============================================
-- 1. 物料分类表 (material_categories)
-- =============================================
DROP TABLE IF EXISTS `material_categories`;
CREATE TABLE `material_categories` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '分类ID',
  `category_code` varchar(50) NOT NULL COMMENT '分类编码',
  `category_name` varchar(100) NOT NULL COMMENT '分类名称',
  `parent_id` bigint(20) DEFAULT 0 COMMENT '父分类ID，0表示顶级分类',
  `category_level` int(11) DEFAULT 1 COMMENT '分类层级',
  `category_path` varchar(500) DEFAULT NULL COMMENT '分类路径，如：/1/2/3',
  `sort_order` int(11) DEFAULT 0 COMMENT '排序号',
  `status` tinyint(1) DEFAULT 1 COMMENT '状态：0-禁用，1-启用',
  `remark` varchar(500) DEFAULT NULL COMMENT '备注',
  `created_by` bigint(20) DEFAULT NULL COMMENT '创建人ID',
  `created_time` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `updated_by` bigint(20) DEFAULT NULL COMMENT '更新人ID',
  `updated_time` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  `is_deleted` tinyint(1) DEFAULT 0 COMMENT '是否删除：0-否，1-是',
  PRIMARY KEY (`id`),
  UNIQUE KEY `uk_category_code` (`category_code`),
  KEY `idx_parent_id` (`parent_id`),
  KEY `idx_status` (`status`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='物料分类表';

-- =============================================
-- 2. 物料主数据表 (materials)
-- =============================================
DROP TABLE IF EXISTS `materials`;
CREATE TABLE `materials` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '物料ID',
  `material_code` varchar(50) NOT NULL COMMENT '物料编码',
  `material_name` varchar(200) NOT NULL COMMENT '物料名称',
  `material_spec` varchar(200) DEFAULT NULL COMMENT '物料规格',
  `material_model` varchar(100) DEFAULT NULL COMMENT '物料型号',
  `category_id` bigint(20) NOT NULL COMMENT '物料分类ID',
  `material_type` tinyint(1) NOT NULL COMMENT '物料类型：1-原材料，2-半成品，3-成品，4-辅料，5-包装材料',
  `unit` varchar(20) NOT NULL COMMENT '基本单位：个/件/kg/m等',
  `aux_unit` varchar(20) DEFAULT NULL COMMENT '辅助单位',
  `conversion_rate` decimal(10,4) DEFAULT 1.0000 COMMENT '单位转换率',
  `barcode` varchar(100) DEFAULT NULL COMMENT '条形码',
  `qr_code` varchar(200) DEFAULT NULL COMMENT '二维码',
  `abc_category` char(1) DEFAULT 'C' COMMENT 'ABC分类：A-重要，B-一般，C-普通',
  `batch_managed` tinyint(1) DEFAULT 0 COMMENT '是否批次管理：0-否，1-是',
  `serial_managed` tinyint(1) DEFAULT 0 COMMENT '是否序列号管理：0-否，1-是',
  `shelf_life_days` int(11) DEFAULT NULL COMMENT '保质期（天）',
  `min_stock` decimal(12,4) DEFAULT 0.0000 COMMENT '最小库存',
  `max_stock` decimal(12,4) DEFAULT 0.0000 COMMENT '最大库存',
  `safety_stock` decimal(12,4) DEFAULT 0.0000 COMMENT '安全库存',
  `standard_cost` decimal(12,4) DEFAULT 0.0000 COMMENT '标准成本',
  `purchase_price` decimal(12,4) DEFAULT 0.0000 COMMENT '采购单价',
  `sales_price` decimal(12,4) DEFAULT 0.0000 COMMENT '销售单价',
  `lead_time` int(11) DEFAULT 0 COMMENT '采购提前期（天）',
  `drawing_no` varchar(100) DEFAULT NULL COMMENT '图号',
  `version` varchar(50) DEFAULT NULL COMMENT '版本号',
  `weight` decimal(10,4) DEFAULT NULL COMMENT '重量（kg）',
  `volume` decimal(10,4) DEFAULT NULL COMMENT '体积（m³）',
  `color` varchar(50) DEFAULT NULL COMMENT '颜色',
  `quality_level` varchar(50) DEFAULT NULL COMMENT '质量等级',
  `origin_place` varchar(100) DEFAULT NULL COMMENT '产地',
  `manufacturer` varchar(200) DEFAULT NULL COMMENT '生产厂家',
  `supplier_id` bigint(20) DEFAULT NULL COMMENT '默认供应商ID',
  `status` tinyint(1) DEFAULT 1 COMMENT '状态：0-停用，1-启用，2-待审核',
  `image_url` varchar(500) DEFAULT NULL COMMENT '物料图片URL',
  `remark` varchar(500) DEFAULT NULL COMMENT '备注',
  `created_by` bigint(20) DEFAULT NULL COMMENT '创建人ID',
  `created_time` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `updated_by` bigint(20) DEFAULT NULL COMMENT '更新人ID',
  `updated_time` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  `is_deleted` tinyint(1) DEFAULT 0 COMMENT '是否删除：0-否，1-是',
  PRIMARY KEY (`id`),
  UNIQUE KEY `uk_material_code` (`material_code`),
  KEY `idx_category_id` (`category_id`),
  KEY `idx_material_type` (`material_type`),
  KEY `idx_barcode` (`barcode`),
  KEY `idx_status` (`status`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='物料主数据表';

-- =============================================
-- 3. 物料BOM表 (material_bom)
-- =============================================
DROP TABLE IF EXISTS `material_bom`;
CREATE TABLE `material_bom` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT 'BOM ID',
  `bom_code` varchar(50) NOT NULL COMMENT 'BOM编码',
  `parent_material_id` bigint(20) NOT NULL COMMENT '父物料ID（产品）',
  `child_material_id` bigint(20) NOT NULL COMMENT '子物料ID（组件）',
  `quantity` decimal(12,4) NOT NULL COMMENT '用量',
  `unit` varchar(20) NOT NULL COMMENT '单位',
  `scrap_rate` decimal(5,2) DEFAULT 0.00 COMMENT '损耗率（%）',
  `position_no` varchar(50) DEFAULT NULL COMMENT '位号',
  `sort_order` int(11) DEFAULT 0 COMMENT '排序号',
  `version` varchar(50) DEFAULT '1.0' COMMENT 'BOM版本',
  `is_key_material` tinyint(1) DEFAULT 0 COMMENT '是否关键物料：0-否，1-是',
  `effective_date` date DEFAULT NULL COMMENT '生效日期',
  `expiry_date` date DEFAULT NULL COMMENT '失效日期',
  `status` tinyint(1) DEFAULT 1 COMMENT '状态：0-停用，1-启用',
  `remark` varchar(500) DEFAULT NULL COMMENT '备注',
  `created_by` bigint(20) DEFAULT NULL COMMENT '创建人ID',
  `created_time` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `updated_by` bigint(20) DEFAULT NULL COMMENT '更新人ID',
  `updated_time` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  `is_deleted` tinyint(1) DEFAULT 0 COMMENT '是否删除：0-否，1-是',
  PRIMARY KEY (`id`),
  UNIQUE KEY `uk_bom_code` (`bom_code`),
  KEY `idx_parent_material` (`parent_material_id`),
  KEY `idx_child_material` (`child_material_id`),
  KEY `idx_status` (`status`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='物料BOM表';

-- =============================================
-- 4. 供应商主数据表 (suppliers)
-- =============================================
DROP TABLE IF EXISTS `suppliers`;
CREATE TABLE `suppliers` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '供应商ID',
  `supplier_code` varchar(50) NOT NULL COMMENT '供应商编码',
  `supplier_name` varchar(200) NOT NULL COMMENT '供应商名称',
  `short_name` varchar(100) DEFAULT NULL COMMENT '简称',
  `supplier_type` tinyint(1) NOT NULL COMMENT '供应商类型：1-原材料，2-设备，3-服务，4-其他',
  `supplier_level` char(1) DEFAULT 'C' COMMENT '供应商等级：A-优秀，B-合格，C-一般',
  `credit_code` varchar(50) DEFAULT NULL COMMENT '统一社会信用代码',
  `legal_person` varchar(50) DEFAULT NULL COMMENT '法人代表',
  `contact_person` varchar(50) DEFAULT NULL COMMENT '联系人',
  `contact_phone` varchar(20) DEFAULT NULL COMMENT '联系电话',
  `contact_mobile` varchar(20) DEFAULT NULL COMMENT '联系手机',
  `email` varchar(100) DEFAULT NULL COMMENT '电子邮箱',
  `fax` varchar(50) DEFAULT NULL COMMENT '传真',
  `province` varchar(50) DEFAULT NULL COMMENT '省份',
  `city` varchar(50) DEFAULT NULL COMMENT '城市',
  `district` varchar(50) DEFAULT NULL COMMENT '区县',
  `address` varchar(300) DEFAULT NULL COMMENT '详细地址',
  `postal_code` varchar(20) DEFAULT NULL COMMENT '邮政编码',
  `bank_name` varchar(200) DEFAULT NULL COMMENT '开户银行',
  `bank_account` varchar(50) DEFAULT NULL COMMENT '银行账号',
  `tax_rate` decimal(5,2) DEFAULT 0.00 COMMENT '税率（%）',
  `payment_terms` varchar(200) DEFAULT NULL COMMENT '付款条件',
  `delivery_cycle` int(11) DEFAULT 0 COMMENT '交货周期（天）',
  `cooperation_start_date` date DEFAULT NULL COMMENT '合作开始日期',
  `status` tinyint(1) DEFAULT 1 COMMENT '状态：0-禁用，1-启用，2-黑名单',
  `remark` varchar(500) DEFAULT NULL COMMENT '备注',
  `created_by` bigint(20) DEFAULT NULL COMMENT '创建人ID',
  `created_time` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `updated_by` bigint(20) DEFAULT NULL COMMENT '更新人ID',
  `updated_time` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  `is_deleted` tinyint(1) DEFAULT 0 COMMENT '是否删除：0-否，1-是',
  PRIMARY KEY (`id`),
  UNIQUE KEY `uk_supplier_code` (`supplier_code`),
  KEY `idx_supplier_type` (`supplier_type`),
  KEY `idx_supplier_level` (`supplier_level`),
  KEY `idx_status` (`status`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='供应商主数据表';

-- =============================================
-- 5. 客户主数据表 (customers)
-- =============================================
DROP TABLE IF EXISTS `customers`;
CREATE TABLE `customers` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '客户ID',
  `customer_code` varchar(50) NOT NULL COMMENT '客户编码',
  `customer_name` varchar(200) NOT NULL COMMENT '客户名称',
  `short_name` varchar(100) DEFAULT NULL COMMENT '简称',
  `customer_type` tinyint(1) NOT NULL COMMENT '客户类型：1-直接客户，2-经销商，3-代理商，4-终端客户',
  `customer_level` char(1) DEFAULT 'C' COMMENT '客户等级：A-重要，B-一般，C-普通',
  `industry` varchar(100) DEFAULT NULL COMMENT '所属行业',
  `credit_code` varchar(50) DEFAULT NULL COMMENT '统一社会信用代码',
  `legal_person` varchar(50) DEFAULT NULL COMMENT '法人代表',
  `contact_person` varchar(50) DEFAULT NULL COMMENT '联系人',
  `contact_phone` varchar(20) DEFAULT NULL COMMENT '联系电话',
  `contact_mobile` varchar(20) DEFAULT NULL COMMENT '联系手机',
  `email` varchar(100) DEFAULT NULL COMMENT '电子邮箱',
  `fax` varchar(50) DEFAULT NULL COMMENT '传真',
  `province` varchar(50) DEFAULT NULL COMMENT '省份',
  `city` varchar(50) DEFAULT NULL COMMENT '城市',
  `district` varchar(50) DEFAULT NULL COMMENT '区县',
  `address` varchar(300) DEFAULT NULL COMMENT '详细地址',
  `postal_code` varchar(20) DEFAULT NULL COMMENT '邮政编码',
  `bank_name` varchar(200) DEFAULT NULL COMMENT '开户银行',
  `bank_account` varchar(50) DEFAULT NULL COMMENT '银行账号',
  `tax_rate` decimal(5,2) DEFAULT 0.00 COMMENT '税率（%）',
  `credit_limit` decimal(15,2) DEFAULT 0.00 COMMENT '信用额度',
  `payment_terms` varchar(200) DEFAULT NULL COMMENT '付款条件',
  `delivery_terms` varchar(200) DEFAULT NULL COMMENT '交货条件',
  `sales_person_id` bigint(20) DEFAULT NULL COMMENT '销售负责人ID',
  `cooperation_start_date` date DEFAULT NULL COMMENT '合作开始日期',
  `status` tinyint(1) DEFAULT 1 COMMENT '状态：0-禁用，1-启用，2-黑名单',
  `remark` varchar(500) DEFAULT NULL COMMENT '备注',
  `created_by` bigint(20) DEFAULT NULL COMMENT '创建人ID',
  `created_time` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `updated_by` bigint(20) DEFAULT NULL COMMENT '更新人ID',
  `updated_time` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  `is_deleted` tinyint(1) DEFAULT 0 COMMENT '是否删除：0-否，1-是',
  PRIMARY KEY (`id`),
  UNIQUE KEY `uk_customer_code` (`customer_code`),
  KEY `idx_customer_type` (`customer_type`),
  KEY `idx_customer_level` (`customer_level`),
  KEY `idx_status` (`status`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='客户主数据表';

-- =============================================
-- 6. 设备分类表 (equipment_categories)
-- =============================================
DROP TABLE IF EXISTS `equipment_categories`;
CREATE TABLE `equipment_categories` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '分类ID',
  `category_code` varchar(50) NOT NULL COMMENT '分类编码',
  `category_name` varchar(100) NOT NULL COMMENT '分类名称',
  `parent_id` bigint(20) DEFAULT 0 COMMENT '父分类ID，0表示顶级分类',
  `category_level` int(11) DEFAULT 1 COMMENT '分类层级',
  `sort_order` int(11) DEFAULT 0 COMMENT '排序号',
  `status` tinyint(1) DEFAULT 1 COMMENT '状态：0-禁用，1-启用',
  `remark` varchar(500) DEFAULT NULL COMMENT '备注',
  `created_by` bigint(20) DEFAULT NULL COMMENT '创建人ID',
  `created_time` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `updated_by` bigint(20) DEFAULT NULL COMMENT '更新人ID',
  `updated_time` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  `is_deleted` tinyint(1) DEFAULT 0 COMMENT '是否删除：0-否，1-是',
  PRIMARY KEY (`id`),
  UNIQUE KEY `uk_category_code` (`category_code`),
  KEY `idx_parent_id` (`parent_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='设备分类表';

-- =============================================
-- 7. 设备主数据表 (equipments)
-- =============================================
DROP TABLE IF EXISTS `equipments`;
CREATE TABLE `equipments` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '设备ID',
  `equipment_code` varchar(50) NOT NULL COMMENT '设备编码',
  `equipment_name` varchar(200) NOT NULL COMMENT '设备名称',
  `equipment_model` varchar(100) DEFAULT NULL COMMENT '设备型号',
  `category_id` bigint(20) NOT NULL COMMENT '设备分类ID',
  `equipment_type` tinyint(1) NOT NULL COMMENT '设备类型：1-生产设备，2-检测设备，3-辅助设备，4-其他',
  `manufacturer` varchar(200) DEFAULT NULL COMMENT '制造商',
  `supplier_id` bigint(20) DEFAULT NULL COMMENT '供应商ID',
  `purchase_date` date DEFAULT NULL COMMENT '购置日期',
  `install_date` date DEFAULT NULL COMMENT '安装日期',
  `commissioning_date` date DEFAULT NULL COMMENT '投产日期',
  `warranty_period` int(11) DEFAULT 0 COMMENT '保修期（月）',
  `warranty_expiry_date` date DEFAULT NULL COMMENT '保修到期日',
  `original_value` decimal(15,2) DEFAULT 0.00 COMMENT '原值',
  `net_value` decimal(15,2) DEFAULT 0.00 COMMENT '净值',
  `depreciation_years` int(11) DEFAULT 0 COMMENT '折旧年限',
  `power_rating` decimal(10,2) DEFAULT NULL COMMENT '额定功率（kW）',
  `voltage` varchar(50) DEFAULT NULL COMMENT '电压',
  `workshop_id` bigint(20) DEFAULT NULL COMMENT '所属车间ID',
  `location` varchar(200) DEFAULT NULL COMMENT '存放位置',
  `responsible_person_id` bigint(20) DEFAULT NULL COMMENT '责任人ID',
  `maintenance_cycle` int(11) DEFAULT 0 COMMENT '保养周期（天）',
  `last_maintenance_date` date DEFAULT NULL COMMENT '上次保养日期',
  `next_maintenance_date` date DEFAULT NULL COMMENT '下次保养日期',
  `equipment_status` tinyint(1) DEFAULT 1 COMMENT '设备状态：1-正常，2-维修中，3-停用，4-报废',
  `qr_code` varchar(200) DEFAULT NULL COMMENT '设备二维码',
  `technical_params` text COMMENT '技术参数（JSON格式）',
  `image_url` varchar(500) DEFAULT NULL COMMENT '设备图片URL',
  `manual_url` varchar(500) DEFAULT NULL COMMENT '使用手册URL',
  `remark` varchar(500) DEFAULT NULL COMMENT '备注',
  `created_by` bigint(20) DEFAULT NULL COMMENT '创建人ID',
  `created_time` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `updated_by` bigint(20) DEFAULT NULL COMMENT '更新人ID',
  `updated_time` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  `is_deleted` tinyint(1) DEFAULT 0 COMMENT '是否删除：0-否，1-是',
  PRIMARY KEY (`id`),
  UNIQUE KEY `uk_equipment_code` (`equipment_code`),
  KEY `idx_category_id` (`category_id`),
  KEY `idx_equipment_type` (`equipment_type`),
  KEY `idx_equipment_status` (`equipment_status`),
  KEY `idx_workshop_id` (`workshop_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='设备主数据表';

-- =============================================
-- 8. 工序主数据表 (processes)
-- =============================================
DROP TABLE IF EXISTS `processes`;
CREATE TABLE `processes` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '工序ID',
  `process_code` varchar(50) NOT NULL COMMENT '工序编码',
  `process_name` varchar(100) NOT NULL COMMENT '工序名称',
  `process_type` tinyint(1) NOT NULL COMMENT '工序类型：1-加工，2-装配，3-检验，4-包装，5-其他',
  `workshop_id` bigint(20) DEFAULT NULL COMMENT '所属车间ID',
  `equipment_id` bigint(20) DEFAULT NULL COMMENT '默认设备ID',
  `standard_time` decimal(10,2) DEFAULT 0.00 COMMENT '标准工时（分钟）',
  `setup_time` decimal(10,2) DEFAULT 0.00 COMMENT '准备时间（分钟）',
  `wait_time` decimal(10,2) DEFAULT 0.00 COMMENT '等待时间（分钟）',
  `queue_time` decimal(10,2) DEFAULT 0.00 COMMENT '排队时间（分钟）',
  `move_time` decimal(10,2) DEFAULT 0.00 COMMENT '传送时间（分钟）',
  `labor_count` int(11) DEFAULT 1 COMMENT '标准人数',
  `labor_cost` decimal(10,2) DEFAULT 0.00 COMMENT '人工成本',
  `equipment_cost` decimal(10,2) DEFAULT 0.00 COMMENT '设备成本',
  `quality_control_point` tinyint(1) DEFAULT 0 COMMENT '是否质量控制点：0-否，1-是',
  `inspection_standard` varchar(500) DEFAULT NULL COMMENT '检验标准',
  `operation_instruction` text COMMENT '作业指导书',
  `safety_precautions` text COMMENT '安全注意事项',
  `sort_order` int(11) DEFAULT 0 COMMENT '排序号',
  `status` tinyint(1) DEFAULT 1 COMMENT '状态：0-停用，1-启用',
  `remark` varchar(500) DEFAULT NULL COMMENT '备注',
  `created_by` bigint(20) DEFAULT NULL COMMENT '创建人ID',
  `created_time` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `updated_by` bigint(20) DEFAULT NULL COMMENT '更新人ID',
  `updated_time` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  `is_deleted` tinyint(1) DEFAULT 0 COMMENT '是否删除：0-否，1-是',
  PRIMARY KEY (`id`),
  UNIQUE KEY `uk_process_code` (`process_code`),
  KEY `idx_process_type` (`process_type`),
  KEY `idx_workshop_id` (`workshop_id`),
  KEY `idx_status` (`status`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='工序主数据表';

-- =============================================
-- 9. 工艺路线表 (routing)
-- =============================================
DROP TABLE IF EXISTS `routing`;
CREATE TABLE `routing` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '路线ID',
  `routing_code` varchar(50) NOT NULL COMMENT '路线编码',
  `routing_name` varchar(100) NOT NULL COMMENT '路线名称',
  `material_id` bigint(20) NOT NULL COMMENT '物料ID',
  `version` varchar(50) DEFAULT '1.0' COMMENT '版本号',
  `effective_date` date DEFAULT NULL COMMENT '生效日期',
  `expiry_date` date DEFAULT NULL COMMENT '失效日期',
  `is_default` tinyint(1) DEFAULT 1 COMMENT '是否默认路线：0-否，1-是',
  `total_standard_time` decimal(10,2) DEFAULT 0.00 COMMENT '总标准工时（分钟）',
  `status` tinyint(1) DEFAULT 1 COMMENT '状态：0-停用，1-启用',
  `remark` varchar(500) DEFAULT NULL COMMENT '备注',
  `created_by` bigint(20) DEFAULT NULL COMMENT '创建人ID',
  `created_time` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `updated_by` bigint(20) DEFAULT NULL COMMENT '更新人ID',
  `updated_time` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  `is_deleted` tinyint(1) DEFAULT 0 COMMENT '是否删除：0-否，1-是',
  PRIMARY KEY (`id`),
  UNIQUE KEY `uk_routing_code` (`routing_code`),
  KEY `idx_material_id` (`material_id`),
  KEY `idx_status` (`status`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='工艺路线表';

-- =============================================
-- 10. 工艺路线工序明细表 (routing_process)
-- =============================================
DROP TABLE IF EXISTS `routing_process`;
CREATE TABLE `routing_process` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '明细ID',
  `routing_id` bigint(20) NOT NULL COMMENT '工艺路线ID',
  `process_id` bigint(20) NOT NULL COMMENT '工序ID',
  `sequence_no` int(11) NOT NULL COMMENT '工序顺序号',
  `equipment_id` bigint(20) DEFAULT NULL COMMENT '设备ID',
  `workshop_id` bigint(20) DEFAULT NULL COMMENT '车间ID',
  `standard_time` decimal(10,2) DEFAULT 0.00 COMMENT '标准工时（分钟）',
  `setup_time` decimal(10,2) DEFAULT 0.00 COMMENT '准备时间（分钟）',
  `labor_count` int(11) DEFAULT 1 COMMENT '标准人数',
  `is_parallel` tinyint(1) DEFAULT 0 COMMENT '是否并行工序：0-否，1-是',
  `parallel_group` int(11) DEFAULT NULL COMMENT '并行组号',
  `is_outsourced` tinyint(1) DEFAULT 0 COMMENT '是否外协：0-否，1-是',
  `supplier_id` bigint(20) DEFAULT NULL COMMENT '外协供应商ID',
  `quality_check_required` tinyint(1) DEFAULT 0 COMMENT '是否需要质检：0-否，1-是',
  `remark` varchar(500) DEFAULT NULL COMMENT '备注',
  `created_by` bigint(20) DEFAULT NULL COMMENT '创建人ID',
  `created_time` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `updated_by` bigint(20) DEFAULT NULL COMMENT '更新人ID',
  `updated_time` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  `is_deleted` tinyint(1) DEFAULT 0 COMMENT '是否删除：0-否，1-是',
  PRIMARY KEY (`id`),
  KEY `idx_routing_id` (`routing_id`),
  KEY `idx_process_id` (`process_id`),
  KEY `idx_sequence_no` (`sequence_no`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='工艺路线工序明细表';

-- =============================================
-- 11. 车间主数据表 (workshops)
-- =============================================
DROP TABLE IF EXISTS `workshops`;
CREATE TABLE `workshops` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '车间ID',
  `workshop_code` varchar(50) NOT NULL COMMENT '车间编码',
  `workshop_name` varchar(100) NOT NULL COMMENT '车间名称',
  `workshop_type` tinyint(1) NOT NULL COMMENT '车间类型：1-生产车间，2-装配车间，3-检验车间，4-仓储车间',
  `parent_id` bigint(20) DEFAULT 0 COMMENT '父车间ID，0表示顶级车间',
  `manager_id` bigint(20) DEFAULT NULL COMMENT '车间主任ID',
  `area` decimal(10,2) DEFAULT NULL COMMENT '面积（平方米）',
  `location` varchar(200) DEFAULT NULL COMMENT '位置',
  `phone` varchar(20) DEFAULT NULL COMMENT '联系电话',
  `capacity` decimal(12,2) DEFAULT NULL COMMENT '产能',
  `capacity_unit` varchar(20) DEFAULT NULL COMMENT '产能单位',
  `sort_order` int(11) DEFAULT 0 COMMENT '排序号',
  `status` tinyint(1) DEFAULT 1 COMMENT '状态：0-停用，1-启用',
  `remark` varchar(500) DEFAULT NULL COMMENT '备注',
  `created_by` bigint(20) DEFAULT NULL COMMENT '创建人ID',
  `created_time` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `updated_by` bigint(20) DEFAULT NULL COMMENT '更新人ID',
  `updated_time` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  `is_deleted` tinyint(1) DEFAULT 0 COMMENT '是否删除：0-否，1-是',
  PRIMARY KEY (`id`),
  UNIQUE KEY `uk_workshop_code` (`workshop_code`),
  KEY `idx_parent_id` (`parent_id`),
  KEY `idx_status` (`status`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='车间主数据表';

-- =============================================
-- 12. 仓库主数据表 (warehouses)
-- =============================================
DROP TABLE IF EXISTS `warehouses`;
CREATE TABLE `warehouses` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '仓库ID',
  `warehouse_code` varchar(50) NOT NULL COMMENT '仓库编码',
  `warehouse_name` varchar(100) NOT NULL COMMENT '仓库名称',
  `warehouse_type` tinyint(1) NOT NULL COMMENT '仓库类型：1-原材料仓，2-半成品仓，3-成品仓，4-辅料仓，5-工具仓，6-废品仓',
  `manager_id` bigint(20) DEFAULT NULL COMMENT '仓库管理员ID',
  `area` decimal(10,2) DEFAULT NULL COMMENT '面积（平方米）',
  `location` varchar(200) DEFAULT NULL COMMENT '位置',
  `phone` varchar(20) DEFAULT NULL COMMENT '联系电话',
  `is_default` tinyint(1) DEFAULT 0 COMMENT '是否默认仓库：0-否，1-是',
  `sort_order` int(11) DEFAULT 0 COMMENT '排序号',
  `status` tinyint(1) DEFAULT 1 COMMENT '状态：0-停用，1-启用',
  `remark` varchar(500) DEFAULT NULL COMMENT '备注',
  `created_by` bigint(20) DEFAULT NULL COMMENT '创建人ID',
  `created_time` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `updated_by` bigint(20) DEFAULT NULL COMMENT '更新人ID',
  `updated_time` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  `is_deleted` tinyint(1) DEFAULT 0 COMMENT '是否删除：0-否，1-是',
  PRIMARY KEY (`id`),
  UNIQUE KEY `uk_warehouse_code` (`warehouse_code`),
  KEY `idx_warehouse_type` (`warehouse_type`),
  KEY `idx_status` (`status`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='仓库主数据表';

-- =============================================
-- 13. 库位主数据表 (warehouse_locations)
-- =============================================
DROP TABLE IF EXISTS `warehouse_locations`;
CREATE TABLE `warehouse_locations` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '库位ID',
  `location_code` varchar(50) NOT NULL COMMENT '库位编码',
  `location_name` varchar(100) NOT NULL COMMENT '库位名称',
  `warehouse_id` bigint(20) NOT NULL COMMENT '所属仓库ID',
  `parent_id` bigint(20) DEFAULT 0 COMMENT '父库位ID，0表示顶级库位',
  `location_type` tinyint(1) NOT NULL COMMENT '库位类型：1-区域，2-货架，3-层，4-位',
  `row_no` varchar(20) DEFAULT NULL COMMENT '行号',
  `column_no` varchar(20) DEFAULT NULL COMMENT '列号',
  `level_no` varchar(20) DEFAULT NULL COMMENT '层号',
  `capacity` decimal(10,2) DEFAULT NULL COMMENT '容量',
  `capacity_unit` varchar(20) DEFAULT NULL COMMENT '容量单位',
  `current_volume` decimal(10,2) DEFAULT 0.00 COMMENT '当前占用量',
  `barcode` varchar(100) DEFAULT NULL COMMENT '库位条码',
  `qr_code` varchar(200) DEFAULT NULL COMMENT '库位二维码',
  `is_locked` tinyint(1) DEFAULT 0 COMMENT '是否锁定：0-否，1-是',
  `sort_order` int(11) DEFAULT 0 COMMENT '排序号',
  `status` tinyint(1) DEFAULT 1 COMMENT '状态：0-停用，1-启用',
  `remark` varchar(500) DEFAULT NULL COMMENT '备注',
  `created_by` bigint(20) DEFAULT NULL COMMENT '创建人ID',
  `created_time` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `updated_by` bigint(20) DEFAULT NULL COMMENT '更新人ID',
  `updated_time` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  `is_deleted` tinyint(1) DEFAULT 0 COMMENT '是否删除：0-否，1-是',
  PRIMARY KEY (`id`),
  UNIQUE KEY `uk_location_code` (`location_code`),
  KEY `idx_warehouse_id` (`warehouse_id`),
  KEY `idx_parent_id` (`parent_id`),
  KEY `idx_status` (`status`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='库位主数据表';

-- =============================================
-- 14. 质检标准表 (quality_standards)
-- =============================================
DROP TABLE IF EXISTS `quality_standards`;
CREATE TABLE `quality_standards` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '标准ID',
  `standard_code` varchar(50) NOT NULL COMMENT '标准编码',
  `standard_name` varchar(200) NOT NULL COMMENT '标准名称',
  `material_id` bigint(20) DEFAULT NULL COMMENT '物料ID',
  `process_id` bigint(20) DEFAULT NULL COMMENT '工序ID',
  `standard_type` tinyint(1) NOT NULL COMMENT '标准类型：1-来料检验，2-过程检验，3-成品检验，4-出货检验',
  `inspection_method` varchar(200) DEFAULT NULL COMMENT '检验方法',
  `sampling_plan` varchar(200) DEFAULT NULL COMMENT '抽样方案',
  `aql` decimal(5,2) DEFAULT NULL COMMENT 'AQL（允收质量水平）',
  `inspection_items` text COMMENT '检验项目（JSON格式）',
  `version` varchar(50) DEFAULT '1.0' COMMENT '版本号',
  `effective_date` date DEFAULT NULL COMMENT '生效日期',
  `expiry_date` date DEFAULT NULL COMMENT '失效日期',
  `status` tinyint(1) DEFAULT 1 COMMENT '状态：0-停用，1-启用',
  `attachment_url` varchar(500) DEFAULT NULL COMMENT '附件URL',
  `remark` varchar(500) DEFAULT NULL COMMENT '备注',
  `created_by` bigint(20) DEFAULT NULL COMMENT '创建人ID',
  `created_time` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `updated_by` bigint(20) DEFAULT NULL COMMENT '更新人ID',
  `updated_time` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  `is_deleted` tinyint(1) DEFAULT 0 COMMENT '是否删除：0-否，1-是',
  PRIMARY KEY (`id`),
  UNIQUE KEY `uk_standard_code` (`standard_code`),
  KEY `idx_material_id` (`material_id`),
  KEY `idx_process_id` (`process_id`),
  KEY `idx_standard_type` (`standard_type`),
  KEY `idx_status` (`status`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='质检标准表';

-- =============================================
-- 15. 不良原因代码表 (defect_codes)
-- =============================================
DROP TABLE IF EXISTS `defect_codes`;
CREATE TABLE `defect_codes` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '代码ID',
  `defect_code` varchar(50) NOT NULL COMMENT '不良代码',
  `defect_name` varchar(100) NOT NULL COMMENT '不良名称',
  `defect_category` varchar(50) DEFAULT NULL COMMENT '不良分类',
  `parent_id` bigint(20) DEFAULT 0 COMMENT '父代码ID，0表示顶级',
  `defect_level` tinyint(1) DEFAULT 3 COMMENT '缺陷等级：1-致命，2-严重，3-一般，4-轻微',
  `description` varchar(500) DEFAULT NULL COMMENT '描述',
  `solution` varchar(500) DEFAULT NULL COMMENT '解决方案',
  `sort_order` int(11) DEFAULT 0 COMMENT '排序号',
  `status` tinyint(1) DEFAULT 1 COMMENT '状态：0-停用，1-启用',
  `remark` varchar(500) DEFAULT NULL COMMENT '备注',
  `created_by` bigint(20) DEFAULT NULL COMMENT '创建人ID',
  `created_time` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `updated_by` bigint(20) DEFAULT NULL COMMENT '更新人ID',
  `updated_time` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  `is_deleted` tinyint(1) DEFAULT 0 COMMENT '是否删除：0-否，1-是',
  PRIMARY KEY (`id`),
  UNIQUE KEY `uk_defect_code` (`defect_code`),
  KEY `idx_parent_id` (`parent_id`),
  KEY `idx_status` (`status`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='不良原因代码表';

SET FOREIGN_KEY_CHECKS = 1;

-- =============================================
-- 初始化数据
-- =============================================

-- 插入物料分类
INSERT INTO `material_categories` (`id`, `category_code`, `category_name`, `parent_id`, `category_level`, `category_path`, `status`) VALUES
(1, 'RAW', '原材料', 0, 1, '/1', 1),
(2, 'RAW-METAL', '金属材料', 1, 2, '/1/2', 1),
(3, 'RAW-PLASTIC', '塑料材料', 1, 2, '/1/3', 1),
(4, 'SEMI', '半成品', 0, 1, '/4', 1),
(5, 'PROD', '成品', 0, 1, '/5', 1),
(6, 'PACK', '包装材料', 0, 1, '/6', 1);

-- 插入物料示例
INSERT INTO `materials` (`id`, `material_code`, `material_name`, `material_spec`, `category_id`, `material_type`, `unit`, `status`) VALUES
(1, 'MAT-RAW-001', '钢板', 'Q235 2mm*1000*2000', 2, 1, 'kg', 1),
(2, 'MAT-RAW-002', 'ABS塑料粒', '注塑级 黑色', 3, 1, 'kg', 1),
(3, 'MAT-SEMI-001', '底座组件', '已装配', 4, 2, '件', 1),
(4, 'MAT-PROD-001', '电子产品A', '标准版', 5, 3, '台', 1);

-- 插入设备分类
INSERT INTO `equipment_categories` (`id`, `category_code`, `category_name`, `parent_id`, `category_level`, `status`) VALUES
(1, 'PROD-EQ', '生产设备', 0, 1, 1),
(2, 'PROD-CNC', '数控设备', 1, 2, 1),
(3, 'PROD-PRESS', '冲压设备', 1, 2, 1),
(4, 'TEST-EQ', '检测设备', 0, 1, 1);

-- 插入车间
INSERT INTO `workshops` (`id`, `workshop_code`, `workshop_name`, `workshop_type`, `status`) VALUES
(1, 'WS-001', '机加工车间', 1, 1),
(2, 'WS-002', '装配车间', 2, 1),
(3, 'WS-003', '质检车间', 3, 1);

-- 插入仓库
INSERT INTO `warehouses` (`id`, `warehouse_code`, `warehouse_name`, `warehouse_type`, `is_default`, `status`) VALUES
(1, 'WH-RAW', '原材料仓库', 1, 1, 1),
(2, 'WH-SEMI', '半成品仓库', 2, 0, 1),
(3, 'WH-PROD', '成品仓库', 3, 0, 1),
(4, 'WH-AUX', '辅料仓库', 4, 0, 1);

-- 插入工序
INSERT INTO `processes` (`id`, `process_code`, `process_name`, `process_type`, `workshop_id`, `standard_time`, `status`) VALUES
(1, 'PROC-001', '下料', 1, 1, 10.00, 1),
(2, 'PROC-002', '冲压', 1, 1, 5.00, 1),
(3, 'PROC-003', '焊接', 1, 1, 15.00, 1),
(4, 'PROC-004', '装配', 2, 2, 20.00, 1),
(5, 'PROC-005', '功能测试', 3, 3, 10.00, 1),
(6, 'PROC-006', '包装', 4, 2, 5.00, 1);

-- 插入不良代码
INSERT INTO `defect_codes` (`id`, `defect_code`, `defect_name`, `defect_category`, `defect_level`, `status`) VALUES
(1, 'DEF-001', '尺寸不合格', '尺寸', 2, 1),
(2, 'DEF-002', '外观划伤', '外观', 3, 1),
(3, 'DEF-003', '功能失效', '功能', 1, 1),
(4, 'DEF-004', '焊接不良', '工艺', 2, 1),
(5, 'DEF-005', '包装破损', '包装', 3, 1);

-- =============================================
-- 说明文档
-- =============================================
/*
MES系统主数据管理模块表结构说明：

核心表说明：

1. material_categories（物料分类表）
   - 支持多级分类树形结构
   - 用于物料的分类管理

2. materials（物料主数据表）
   - 系统核心表，存储所有物料信息
   - 支持原材料、半成品、成品等多种类型
   - 包含库存管理、成本管理相关字段
   - 支持批次管理、序列号管理
   - 支持条码、二维码

3. material_bom（物料BOM表）
   - 定义产品的物料清单
   - 支持多层BOM结构
   - 包含用量、损耗率等信息

4. suppliers（供应商主数据表）
   - 供应商基本信息管理
   - 包含联系方式、银行账户、合作条件等
   - 支持供应商等级评定

5. customers（客户主数据表）
   - 客户基本信息管理
   - 包含联系方式、信用额度、合作条件等
   - 支持客户分类和等级管理

6. equipment_categories（设备分类表）
   - 设备分类管理

7. equipments（设备主数据表）
   - 生产设备信息管理
   - 包含设备状态、维护信息、资产信息
   - 支持设备二维码管理

8. processes（工序主数据表）
   - 工序信息定义
   - 包含标准工时、人工成本等
   - 支持质量控制点定义

9. routing（工艺路线表）
   - 定义产品的工艺路线
   - 支持多版本管理

10. routing_process（工艺路线工序明细表）
    - 工艺路线的具体工序序列
    - 支持并行工序
    - 支持外协工序

11. workshops（车间主数据表）
    - 车间基本信息
    - 包含产能信息

12. warehouses（仓库主数据表）
    - 仓库基本信息
    - 支持多仓库管理

13. warehouse_locations（库位主数据表）
    - 库位精细化管理
    - 支持多层级库位结构（区域-货架-层-位）
    - 支持库位容量管理

14. quality_standards（质检标准表）
    - 质量检验标准定义
    - 支持多种检验类型
    - 支持版本管理

15. defect_codes（不良原因代码表）
    - 不良品原因分类
    - 支持缺陷等级定义

数据特点：
1. 所有表都包含软删除字段（is_deleted）
2. 所有表都包含完整的审计字段
3. 关键字段都建立了索引
4. 支持状态管理
5. 预置了示例初始化数据

使用建议：
1. 建议先完善基础数据（部门、车间、仓库等）
2. 再录入物料、设备等主数据
3. 最后建立BOM、工艺路线等关联关系
4. 定期备份主数据
5. 建立主数据变更审批流程
*/
