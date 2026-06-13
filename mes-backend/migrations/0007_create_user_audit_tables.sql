-- User management tables

-- Departments table
CREATE TABLE IF NOT EXISTS departments (
    id            BIGSERIAL PRIMARY KEY,
    dept_code     VARCHAR(50) NOT NULL UNIQUE,
    dept_name     VARCHAR(100) NOT NULL,
    parent_id     BIGINT DEFAULT 0,
    dept_level    INTEGER DEFAULT 1,
    dept_path     VARCHAR(500),
    manager_id    BIGINT,
    phone         VARCHAR(20),
    email         VARCHAR(100),
    sort_order    INTEGER DEFAULT 0,
    status        SMALLINT DEFAULT 1,
    remark        VARCHAR(500),
    created_by    BIGINT,
    created_time  TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_by    BIGINT,
    updated_time  TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    is_deleted    SMALLINT DEFAULT 0
);

CREATE INDEX IF NOT EXISTS idx_departments_parent_id ON departments (parent_id);
CREATE INDEX IF NOT EXISTS idx_departments_status ON departments (status);

-- Users table
CREATE TABLE IF NOT EXISTS users (
    id                  BIGSERIAL PRIMARY KEY,
    username            VARCHAR(50) NOT NULL UNIQUE,
    password            VARCHAR(200) NOT NULL,
    real_name           VARCHAR(50) NOT NULL,
    employee_no         VARCHAR(50) UNIQUE,
    dept_id             BIGINT,
    email               VARCHAR(100),
    phone               VARCHAR(20),
    gender              SMALLINT,
    avatar              VARCHAR(500),
    job_title           VARCHAR(50),
    status              SMALLINT DEFAULT 1,
    is_locked           SMALLINT DEFAULT 0,
    lock_reason         VARCHAR(200),
    pwd_update_time     TIMESTAMPTZ,
    last_login_time     TIMESTAMPTZ,
    last_login_ip       VARCHAR(50),
    login_fail_count    INTEGER DEFAULT 0,
    remark              VARCHAR(500),
    created_by          BIGINT,
    created_time        TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_by          BIGINT,
    updated_time        TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    is_deleted          SMALLINT DEFAULT 0
);

CREATE INDEX IF NOT EXISTS idx_users_dept_id ON users (dept_id);
CREATE INDEX IF NOT EXISTS idx_users_phone ON users (phone);
CREATE INDEX IF NOT EXISTS idx_users_email ON users (email);
CREATE INDEX IF NOT EXISTS idx_users_status ON users (status);

-- Roles table
CREATE TABLE IF NOT EXISTS roles (
    id            BIGSERIAL PRIMARY KEY,
    role_code     VARCHAR(50) NOT NULL UNIQUE,
    role_name     VARCHAR(100) NOT NULL,
    role_type     SMALLINT DEFAULT 0,
    data_scope    SMALLINT DEFAULT 1,
    sort_order    INTEGER DEFAULT 0,
    status        SMALLINT DEFAULT 1,
    remark        VARCHAR(500),
    created_by    BIGINT,
    created_time  TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_by    BIGINT,
    updated_time  TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    is_deleted    SMALLINT DEFAULT 0
);

CREATE INDEX IF NOT EXISTS idx_roles_status ON roles (status);

-- Permissions table
CREATE TABLE IF NOT EXISTS permissions (
    id                BIGSERIAL PRIMARY KEY,
    permission_code   VARCHAR(100) NOT NULL UNIQUE,
    permission_name   VARCHAR(100) NOT NULL,
    parent_id         BIGINT DEFAULT 0,
    permission_type   SMALLINT NOT NULL,
    route_path        VARCHAR(200),
    component_path    VARCHAR(200),
    icon              VARCHAR(100),
    api_url           VARCHAR(200),
    api_method        VARCHAR(10),
    sort_order        INTEGER DEFAULT 0,
    is_visible        SMALLINT DEFAULT 1,
    status            SMALLINT DEFAULT 1,
    remark            VARCHAR(500),
    created_by        BIGINT,
    created_time      TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_by        BIGINT,
    updated_time      TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    is_deleted        SMALLINT DEFAULT 0
);

CREATE INDEX IF NOT EXISTS idx_permissions_parent_id ON permissions (parent_id);
CREATE INDEX IF NOT EXISTS idx_permissions_permission_type ON permissions (permission_type);
CREATE INDEX IF NOT EXISTS idx_permissions_status ON permissions (status);

-- User roles table
CREATE TABLE IF NOT EXISTS user_roles (
    id            BIGSERIAL PRIMARY KEY,
    user_id       BIGINT NOT NULL,
    role_id       BIGINT NOT NULL,
    created_by    BIGINT,
    created_time  TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(user_id, role_id)
);

CREATE INDEX IF NOT EXISTS idx_user_roles_user_id ON user_roles (user_id);
CREATE INDEX IF NOT EXISTS idx_user_roles_role_id ON user_roles (role_id);

-- Role permissions table
CREATE TABLE IF NOT EXISTS role_permissions (
    id              BIGSERIAL PRIMARY KEY,
    role_id         BIGINT NOT NULL,
    permission_id   BIGINT NOT NULL,
    created_by      BIGINT,
    created_time    TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(role_id, permission_id)
);

CREATE INDEX IF NOT EXISTS idx_role_permissions_role_id ON role_permissions (role_id);
CREATE INDEX IF NOT EXISTS idx_role_permissions_permission_id ON role_permissions (permission_id);

-- Login logs table
CREATE TABLE IF NOT EXISTS login_logs (
    id            BIGSERIAL PRIMARY KEY,
    user_id       BIGINT,
    username      VARCHAR(100),
    login_time    TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    login_ip      VARCHAR(64),
    user_agent    VARCHAR(255),
    result        SMALLINT     NOT NULL, -- 1:success, 2:fail
    fail_reason   VARCHAR(255),
    created_time  TIMESTAMPTZ  NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_login_logs_user_time ON login_logs (user_id, login_time DESC);

-- Operation logs table
CREATE TABLE IF NOT EXISTS operation_logs (
    id            BIGSERIAL PRIMARY KEY,
    user_id       BIGINT,
    username      VARCHAR(100),
    module        VARCHAR(100),
    action        VARCHAR(100),
    request_path  VARCHAR(200),
    method        VARCHAR(10),
    request_time  TIMESTAMPTZ  NOT NULL DEFAULT CURRENT_TIMESTAMP,
    success       SMALLINT      NOT NULL,
    client_ip     VARCHAR(64),
    payload       JSONB,
    error_message VARCHAR(255),
    created_time  TIMESTAMPTZ   NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_operation_logs_user_time ON operation_logs (user_id, request_time DESC);

-- Insert default data
INSERT INTO departments (id, dept_code, dept_name, parent_id, dept_level, dept_path, status, remark) VALUES
(1, 'ROOT', '总公司', 0, 1, '/1', 1, '顶级部门'),
(2, 'IT', '信息技术部', 1, 2, '/1/2', 1, '负责IT系统开发与维护'),
(3, 'PROD', '生产部', 1, 2, '/1/3', 1, '负责生产制造'),
(4, 'QC', '质量管理部', 1, 2, '/1/4', 1, '负责质量检验与控制'),
(5, 'WH', '仓储部', 1, 2, '/1/5', 1, '负责物料仓储管理')
ON CONFLICT (id) DO NOTHING;

-- Insert default users (password: admin123)
INSERT INTO users (id, username, password, real_name, employee_no, dept_id, email, phone, gender, status, is_locked, created_time, updated_time, is_deleted) VALUES
(1, 'admin', '$2a$10$7JB720yubVSZvUI0rEqK/.VqGOZTH.ulu33dHOiBE/TP57ErKW9Cu', '系统管理员', 'EMP001', 2, 'admin@mes.com', '13800138000', 1, 1, 0, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, 0),
(2, 'operator', '$2a$10$7JB720yubVSZvUI0rEqK/.VqGOZTH.ulu33dHOiBE/TP57ErKW9Cu', '普通操作员', 'EMP002', 3, 'operator@mes.com', '13800138001', 1, 1, 0, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, 0)
ON CONFLICT (id) DO NOTHING;

-- Insert default roles
INSERT INTO roles (id, role_code, role_name, role_type, data_scope, status, remark) VALUES
(1, 'SUPER_ADMIN', '超级管理员', 1, 1, 1, '拥有系统所有权限'),
(2, 'ADMIN', '系统管理员', 1, 1, 1, '系统管理员角色'),
(3, 'PROD_MANAGER', '生产经理', 0, 2, 1, '生产部门经理'),
(4, 'OPERATOR', '操作员', 0, 4, 1, '普通操作员'),
(5, 'QC_STAFF', '质检员', 0, 3, 1, '质量检验人员')
ON CONFLICT (id) DO NOTHING;

-- Insert default permissions
INSERT INTO permissions (id, permission_code, permission_name, parent_id, permission_type, route_path, icon, sort_order, status) VALUES
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
(13, 'QUALITY', '质量管理', 0, 1, '/quality', 'quality', 3, 1)
ON CONFLICT (id) DO NOTHING;

-- Insert user role associations
INSERT INTO user_roles (user_id, role_id) VALUES
(1, 1),
(2, 4)
ON CONFLICT (user_id, role_id) DO NOTHING;



