-- =============================================
-- MES系统 - 用户管理模块数据库表结构
-- 创建日期: 2025-10-28
-- 数据库: MySQL 5.7+
-- =============================================

-- 设置字符集
SET NAMES utf8mb4;
SET FOREIGN_KEY_CHECKS = 0;

-- =============================================
-- 1. 部门表 (departments)
-- =============================================
DROP TABLE IF EXISTS `departments`;
CREATE TABLE `departments` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '部门ID',
  `dept_code` varchar(50) NOT NULL COMMENT '部门编码',
  `dept_name` varchar(100) NOT NULL COMMENT '部门名称',
  `parent_id` bigint(20) DEFAULT 0 COMMENT '父部门ID，0表示顶级部门',
  `dept_level` int(11) DEFAULT 1 COMMENT '部门层级',
  `dept_path` varchar(500) DEFAULT NULL COMMENT '部门路径，如：/1/2/3',
  `manager_id` bigint(20) DEFAULT NULL COMMENT '部门负责人ID',
  `phone` varchar(20) DEFAULT NULL COMMENT '部门电话',
  `email` varchar(100) DEFAULT NULL COMMENT '部门邮箱',
  `sort_order` int(11) DEFAULT 0 COMMENT '排序号',
  `status` tinyint(1) DEFAULT 1 COMMENT '状态：0-禁用，1-启用',
  `remark` varchar(500) DEFAULT NULL COMMENT '备注',
  `created_by` bigint(20) DEFAULT NULL COMMENT '创建人ID',
  `created_time` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `updated_by` bigint(20) DEFAULT NULL COMMENT '更新人ID',
  `updated_time` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  `is_deleted` tinyint(1) DEFAULT 0 COMMENT '是否删除：0-否，1-是',
  PRIMARY KEY (`id`),
  UNIQUE KEY `uk_dept_code` (`dept_code`),
  KEY `idx_parent_id` (`parent_id`),
  KEY `idx_status` (`status`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='部门表';

-- =============================================
-- 2. 用户表 (users)
-- =============================================
DROP TABLE IF EXISTS `users`;
CREATE TABLE `users` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '用户ID',
  `username` varchar(50) NOT NULL COMMENT '用户名',
  `password` varchar(200) NOT NULL COMMENT '密码（加密后）',
  `real_name` varchar(50) NOT NULL COMMENT '真实姓名',
  `employee_no` varchar(50) DEFAULT NULL COMMENT '工号',
  `dept_id` bigint(20) DEFAULT NULL COMMENT '所属部门ID',
  `email` varchar(100) DEFAULT NULL COMMENT '邮箱',
  `phone` varchar(20) DEFAULT NULL COMMENT '手机号',
  `gender` tinyint(1) DEFAULT NULL COMMENT '性别：0-女，1-男，2-未知',
  `avatar` varchar(500) DEFAULT NULL COMMENT '头像URL',
  `job_title` varchar(50) DEFAULT NULL COMMENT '职位',
  `status` tinyint(1) DEFAULT 1 COMMENT '状态：0-禁用，1-启用',
  `is_locked` tinyint(1) DEFAULT 0 COMMENT '是否锁定：0-否，1-是',
  `lock_reason` varchar(200) DEFAULT NULL COMMENT '锁定原因',
  `pwd_update_time` datetime DEFAULT NULL COMMENT '密码更新时间',
  `last_login_time` datetime DEFAULT NULL COMMENT '最后登录时间',
  `last_login_ip` varchar(50) DEFAULT NULL COMMENT '最后登录IP',
  `login_fail_count` int(11) DEFAULT 0 COMMENT '登录失败次数',
  `remark` varchar(500) DEFAULT NULL COMMENT '备注',
  `created_by` bigint(20) DEFAULT NULL COMMENT '创建人ID',
  `created_time` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `updated_by` bigint(20) DEFAULT NULL COMMENT '更新人ID',
  `updated_time` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  `is_deleted` tinyint(1) DEFAULT 0 COMMENT '是否删除：0-否，1-是',
  PRIMARY KEY (`id`),
  UNIQUE KEY `uk_username` (`username`),
  UNIQUE KEY `uk_employee_no` (`employee_no`),
  KEY `idx_dept_id` (`dept_id`),
  KEY `idx_phone` (`phone`),
  KEY `idx_email` (`email`),
  KEY `idx_status` (`status`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='用户表';

-- =============================================
-- 3. 角色表 (roles)
-- =============================================
DROP TABLE IF EXISTS `roles`;
CREATE TABLE `roles` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '角色ID',
  `role_code` varchar(50) NOT NULL COMMENT '角色编码',
  `role_name` varchar(100) NOT NULL COMMENT '角色名称',
  `role_type` tinyint(1) DEFAULT 0 COMMENT '角色类型：0-普通角色，1-系统角色',
  `data_scope` tinyint(1) DEFAULT 1 COMMENT '数据权限范围：1-全部，2-本部门及下级，3-本部门，4-仅本人，5-自定义',
  `sort_order` int(11) DEFAULT 0 COMMENT '排序号',
  `status` tinyint(1) DEFAULT 1 COMMENT '状态：0-禁用，1-启用',
  `remark` varchar(500) DEFAULT NULL COMMENT '备注',
  `created_by` bigint(20) DEFAULT NULL COMMENT '创建人ID',
  `created_time` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `updated_by` bigint(20) DEFAULT NULL COMMENT '更新人ID',
  `updated_time` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  `is_deleted` tinyint(1) DEFAULT 0 COMMENT '是否删除：0-否，1-是',
  PRIMARY KEY (`id`),
  UNIQUE KEY `uk_role_code` (`role_code`),
  KEY `idx_status` (`status`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='角色表';

-- =============================================
-- 4. 权限表 (permissions)
-- =============================================
DROP TABLE IF EXISTS `permissions`;
CREATE TABLE `permissions` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '权限ID',
  `permission_code` varchar(100) NOT NULL COMMENT '权限编码',
  `permission_name` varchar(100) NOT NULL COMMENT '权限名称',
  `parent_id` bigint(20) DEFAULT 0 COMMENT '父权限ID，0表示顶级权限',
  `permission_type` tinyint(1) NOT NULL COMMENT '权限类型：1-菜单，2-按钮，3-接口',
  `route_path` varchar(200) DEFAULT NULL COMMENT '路由路径',
  `component_path` varchar(200) DEFAULT NULL COMMENT '组件路径',
  `icon` varchar(100) DEFAULT NULL COMMENT '图标',
  `api_url` varchar(200) DEFAULT NULL COMMENT 'API接口地址',
  `api_method` varchar(10) DEFAULT NULL COMMENT 'API请求方法：GET/POST/PUT/DELETE',
  `sort_order` int(11) DEFAULT 0 COMMENT '排序号',
  `is_visible` tinyint(1) DEFAULT 1 COMMENT '是否显示：0-否，1-是',
  `status` tinyint(1) DEFAULT 1 COMMENT '状态：0-禁用，1-启用',
  `remark` varchar(500) DEFAULT NULL COMMENT '备注',
  `created_by` bigint(20) DEFAULT NULL COMMENT '创建人ID',
  `created_time` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `updated_by` bigint(20) DEFAULT NULL COMMENT '更新人ID',
  `updated_time` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  `is_deleted` tinyint(1) DEFAULT 0 COMMENT '是否删除：0-否，1-是',
  PRIMARY KEY (`id`),
  UNIQUE KEY `uk_permission_code` (`permission_code`),
  KEY `idx_parent_id` (`parent_id`),
  KEY `idx_permission_type` (`permission_type`),
  KEY `idx_status` (`status`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='权限表';

-- =============================================
-- 5. 用户角色关联表 (user_roles)
-- =============================================
DROP TABLE IF EXISTS `user_roles`;
CREATE TABLE `user_roles` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '关联ID',
  `user_id` bigint(20) NOT NULL COMMENT '用户ID',
  `role_id` bigint(20) NOT NULL COMMENT '角色ID',
  `created_by` bigint(20) DEFAULT NULL COMMENT '创建人ID',
  `created_time` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  PRIMARY KEY (`id`),
  UNIQUE KEY `uk_user_role` (`user_id`, `role_id`),
  KEY `idx_user_id` (`user_id`),
  KEY `idx_role_id` (`role_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='用户角色关联表';

-- =============================================
-- 6. 角色权限关联表 (role_permissions)
-- =============================================
DROP TABLE IF EXISTS `role_permissions`;
CREATE TABLE `role_permissions` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '关联ID',
  `role_id` bigint(20) NOT NULL COMMENT '角色ID',
  `permission_id` bigint(20) NOT NULL COMMENT '权限ID',
  `created_by` bigint(20) DEFAULT NULL COMMENT '创建人ID',
  `created_time` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  PRIMARY KEY (`id`),
  UNIQUE KEY `uk_role_permission` (`role_id`, `permission_id`),
  KEY `idx_role_id` (`role_id`),
  KEY `idx_permission_id` (`permission_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='角色权限关联表';

-- =============================================
-- 7. 用户登录日志表 (user_login_logs)
-- =============================================
DROP TABLE IF EXISTS `user_login_logs`;
CREATE TABLE `user_login_logs` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '日志ID',
  `user_id` bigint(20) NOT NULL COMMENT '用户ID',
  `username` varchar(50) NOT NULL COMMENT '用户名',
  `login_time` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '登录时间',
  `login_ip` varchar(50) DEFAULT NULL COMMENT '登录IP',
  `login_location` varchar(100) DEFAULT NULL COMMENT '登录地点',
  `browser` varchar(100) DEFAULT NULL COMMENT '浏览器',
  `os` varchar(100) DEFAULT NULL COMMENT '操作系统',
  `login_status` tinyint(1) NOT NULL COMMENT '登录状态：0-失败，1-成功',
  `login_message` varchar(200) DEFAULT NULL COMMENT '登录消息',
  `logout_time` datetime DEFAULT NULL COMMENT '退出时间',
  PRIMARY KEY (`id`),
  KEY `idx_user_id` (`user_id`),
  KEY `idx_login_time` (`login_time`),
  KEY `idx_login_status` (`login_status`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='用户登录日志表';

-- =============================================
-- 8. 用户操作日志表 (user_operation_logs)
-- =============================================
DROP TABLE IF EXISTS `user_operation_logs`;
CREATE TABLE `user_operation_logs` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT COMMENT '日志ID',
  `user_id` bigint(20) DEFAULT NULL COMMENT '用户ID',
  `username` varchar(50) DEFAULT NULL COMMENT '用户名',
  `module` varchar(50) NOT NULL COMMENT '操作模块',
  `operation_type` varchar(50) NOT NULL COMMENT '操作类型：新增/修改/删除/查询/导出等',
  `operation_desc` varchar(500) DEFAULT NULL COMMENT '操作描述',
  `request_method` varchar(10) DEFAULT NULL COMMENT '请求方法',
  `request_url` varchar(500) DEFAULT NULL COMMENT '请求URL',
  `request_params` text COMMENT '请求参数',
  `response_result` text COMMENT '响应结果',
  `operation_ip` varchar(50) DEFAULT NULL COMMENT '操作IP',
  `operation_location` varchar(100) DEFAULT NULL COMMENT '操作地点',
  `operation_status` tinyint(1) DEFAULT 1 COMMENT '操作状态：0-失败，1-成功',
  `error_msg` text COMMENT '错误信息',
  `execution_time` int(11) DEFAULT NULL COMMENT '执行时长（毫秒）',
  `operation_time` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '操作时间',
  PRIMARY KEY (`id`),
  KEY `idx_user_id` (`user_id`),
  KEY `idx_module` (`module`),
  KEY `idx_operation_time` (`operation_time`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='用户操作日志表';

-- =============================================
-- 初始化数据
-- =============================================

-- 插入默认部门
INSERT INTO `departments` (`id`, `dept_code`, `dept_name`, `parent_id`, `dept_level`, `dept_path`, `status`, `remark`) VALUES
(1, 'ROOT', '总公司', 0, 1, '/1', 1, '顶级部门'),
(2, 'IT', '信息技术部', 1, 2, '/1/2', 1, '负责IT系统开发与维护'),
(3, 'PROD', '生产部', 1, 2, '/1/3', 1, '负责生产制造'),
(4, 'QC', '质量管理部', 1, 2, '/1/4', 1, '负责质量检验与控制'),
(5, 'WH', '仓储部', 1, 2, '/1/5', 1, '负责物料仓储管理');

-- 插入默认用户（密码为：admin123，实际使用时需要加密）
INSERT INTO `users` (`id`, `username`, `password`, `real_name`, `employee_no`, `dept_id`, `email`, `phone`, `gender`, `status`) VALUES
(1, 'admin', '$2a$10$7JB720yubVSZvUI0rEqK/.VqGOZTH.ulu33dHOiBE/TP57ErKW9Cu', '系统管理员', 'EMP001', 2, 'admin@mes.com', '13800138000', 1, 1),
(2, 'operator', '$2a$10$7JB720yubVSZvUI0rEqK/.VqGOZTH.ulu33dHOiBE/TP57ErKW9Cu', '普通操作员', 'EMP002', 3, 'operator@mes.com', '13800138001', 1, 1);

-- 插入默认角色
INSERT INTO `roles` (`id`, `role_code`, `role_name`, `role_type`, `data_scope`, `status`, `remark`) VALUES
(1, 'SUPER_ADMIN', '超级管理员', 1, 1, 1, '拥有系统所有权限'),
(2, 'ADMIN', '系统管理员', 1, 1, 1, '系统管理员角色'),
(3, 'PROD_MANAGER', '生产经理', 0, 2, 1, '生产部门经理'),
(4, 'OPERATOR', '操作员', 0, 4, 1, '普通操作员'),
(5, 'QC_STAFF', '质检员', 0, 3, 1, '质量检验人员');

-- 插入默认权限（示例）
INSERT INTO `permissions` (`id`, `permission_code`, `permission_name`, `parent_id`, `permission_type`, `route_path`, `icon`, `sort_order`, `status`) VALUES
(1, 'SYSTEM', '系统管理', 0, 1, '/system', 'system', 1, 1),
(2, 'SYSTEM:USER', '用户管理', 1, 1, '/system/user', 'user', 1, 1),
(3, 'SYSTEM:USER:LIST', '用户列表', 2, 2, NULL, NULL, 1, 1),
(4, 'SYSTEM:USER:ADD', '新增用户', 2, 2, NULL, NULL, 2, 1),
(5, 'SYSTEM:USER:EDIT', '编辑用户', 2, 2, NULL, NULL, 3, 1),
(6, 'SYSTEM:USER:DELETE', '删除用户', 2, 2, NULL, NULL, 4, 1),
(7, 'SYSTEM:ROLE', '角色管理', 1, 1, '/system/role', 'role', 2, 1),
(8, 'SYSTEM:ROLE:LIST', '角色列表', 7, 2, NULL, NULL, 1, 1),
(9, 'SYSTEM:ROLE:ADD', '新增角色', 7, 2, NULL, NULL, 2, 1),
(10, 'SYSTEM:ROLE:EDIT', '编辑角色', 7, 2, NULL, NULL, 3, 1),
(11, 'SYSTEM:DEPT', '部门管理', 1, 1, '/system/dept', 'dept', 3, 1),
(12, 'PRODUCTION', '生产管理', 0, 1, '/production', 'production', 2, 1),
(13, 'QUALITY', '质量管理', 0, 1, '/quality', 'quality', 3, 1);

-- 插入用户角色关联
INSERT INTO `user_roles` (`user_id`, `role_id`) VALUES
(1, 1),
(2, 4);

-- 插入角色权限关联（超级管理员拥有所有权限）
INSERT INTO `role_permissions` (`role_id`, `permission_id`) 
SELECT 1, id FROM `permissions`;

-- 插入操作员基本权限
INSERT INTO `role_permissions` (`role_id`, `permission_id`) VALUES
(4, 12),
(4, 13);

SET FOREIGN_KEY_CHECKS = 1;

-- =============================================
-- 说明文档
-- =============================================
/*
表结构说明：

1. departments（部门表）
   - 支持多级部门树形结构
   - 包含部门负责人、联系方式等信息

2. users（用户表）
   - 存储用户基本信息
   - 包含账号状态、锁定状态、登录信息等
   - 关联部门信息

3. roles（角色表）
   - 定义系统角色
   - 支持数据权限范围控制
   - 区分系统角色和普通角色

4. permissions（权限表）
   - 支持菜单、按钮、接口三种权限类型
   - 树形结构管理权限
   - 可配置路由、组件、API等信息

5. user_roles（用户角色关联表）
   - 多对多关系：一个用户可以有多个角色

6. role_permissions（角色权限关联表）
   - 多对多关系：一个角色可以有多个权限

7. user_login_logs（用户登录日志表）
   - 记录用户登录、登出信息
   - 包含IP、地点、浏览器等信息

8. user_operation_logs（用户操作日志表）
   - 记录用户操作行为
   - 包含请求信息、响应结果、执行时长等

使用说明：
1. 密码字段使用BCrypt加密存储
2. 所有表都包含软删除字段（is_deleted）
3. 所有表都包含创建人、创建时间、更新人、更新时间字段
4. 索引已根据常用查询场景优化
5. 初始密码为：admin123（实际使用时请修改）

数据权限范围说明：
1 - 全部数据权限
2 - 本部门及下级部门数据权限
3 - 本部门数据权限
4 - 仅本人数据权限
5 - 自定义数据权限
*/
