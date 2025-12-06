-- Production extension: material requirements, picking, return, completion receipts

CREATE TABLE IF NOT EXISTS material_requirements (
    id                  BIGSERIAL PRIMARY KEY,
    production_order_id BIGINT       NOT NULL REFERENCES production_orders(id),
    material_id         BIGINT       NOT NULL REFERENCES materials(id),
    required_quantity   NUMERIC(18,6) NOT NULL DEFAULT 0,
    reserved_quantity   NUMERIC(18,6) NOT NULL DEFAULT 0,
    issued_quantity     NUMERIC(18,6) NOT NULL DEFAULT 0,
    unit                VARCHAR(20)   NOT NULL,
    remark              TEXT,
    created_time        TIMESTAMPTZ   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_time        TIMESTAMPTZ   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    is_deleted          SMALLINT      NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS picking_orders (
    id                  BIGSERIAL PRIMARY KEY,
    picking_no          VARCHAR(50)   NOT NULL UNIQUE,
    production_order_id BIGINT        NOT NULL REFERENCES production_orders(id),
    warehouse_id        BIGINT        NOT NULL REFERENCES warehouses(id),
    work_order_id       BIGINT,
    picking_type        SMALLINT      NOT NULL DEFAULT 1, -- 1:工单领料,2:补料,3:其他
    plan_picking_date   DATE,
    actual_picking_date DATE,
    total_quantity      NUMERIC(18,6) NOT NULL DEFAULT 0,
    order_status        SMALLINT      NOT NULL DEFAULT 1, -- 1:待领料,2:部分,3:完成,4:取消
    remark              TEXT,
    created_time        TIMESTAMPTZ   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_time        TIMESTAMPTZ   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    is_deleted          SMALLINT      NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS picking_order_lines (
    id                  BIGSERIAL PRIMARY KEY,
    picking_id          BIGINT        NOT NULL REFERENCES picking_orders(id) ON DELETE CASCADE,
    material_id         BIGINT        NOT NULL REFERENCES materials(id),
    warehouse_id        BIGINT        NOT NULL REFERENCES warehouses(id),
    location_id         BIGINT        REFERENCES locations(id),
    batch_no            VARCHAR(100),
    plan_quantity       NUMERIC(18,6) NOT NULL DEFAULT 0,
    actual_quantity     NUMERIC(18,6) NOT NULL DEFAULT 0,
    unit                VARCHAR(20)   NOT NULL,
    line_status         SMALLINT      NOT NULL DEFAULT 1, -- 1:待领料,2:部分,3:完成,4:取消
    remark              TEXT,
    created_time        TIMESTAMPTZ   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_time        TIMESTAMPTZ   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    is_deleted          SMALLINT      NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS return_orders (
    id                  BIGSERIAL PRIMARY KEY,
    return_no           VARCHAR(50)   NOT NULL UNIQUE,
    production_order_id BIGINT        NOT NULL REFERENCES production_orders(id),
    warehouse_id        BIGINT        NOT NULL REFERENCES warehouses(id),
    work_order_id       BIGINT,
    return_type         SMALLINT      NOT NULL DEFAULT 1, -- 1:多余退料,2:不良退料,3:其他
    plan_return_date    DATE,
    actual_return_date  DATE,
    total_quantity      NUMERIC(18,6) NOT NULL DEFAULT 0,
    order_status        SMALLINT      NOT NULL DEFAULT 1, -- 1:待退料,2:部分,3:完成,4:取消
    remark              TEXT,
    created_time        TIMESTAMPTZ   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_time        TIMESTAMPTZ   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    is_deleted          SMALLINT      NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS return_order_lines (
    id                  BIGSERIAL PRIMARY KEY,
    return_id           BIGINT        NOT NULL REFERENCES return_orders(id) ON DELETE CASCADE,
    material_id         BIGINT        NOT NULL REFERENCES materials(id),
    warehouse_id        BIGINT        NOT NULL REFERENCES warehouses(id),
    location_id         BIGINT        REFERENCES locations(id),
    batch_no            VARCHAR(100),
    plan_quantity       NUMERIC(18,6) NOT NULL DEFAULT 0,
    actual_quantity     NUMERIC(18,6) NOT NULL DEFAULT 0,
    unit                VARCHAR(20)   NOT NULL,
    line_status         SMALLINT      NOT NULL DEFAULT 1,
    remark              TEXT,
    created_time        TIMESTAMPTZ   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_time        TIMESTAMPTZ   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    is_deleted          SMALLINT      NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS production_receipts (
    id                  BIGSERIAL PRIMARY KEY,
    receipt_no          VARCHAR(50)   NOT NULL UNIQUE,
    production_order_id BIGINT        NOT NULL REFERENCES production_orders(id),
    work_order_id       BIGINT,
    material_id         BIGINT        NOT NULL REFERENCES materials(id),
    warehouse_id        BIGINT        NOT NULL REFERENCES warehouses(id),
    location_id         BIGINT        REFERENCES locations(id),
    receipt_type        SMALLINT      NOT NULL DEFAULT 1, -- 1:完工入库,2:返修入库,3:其他
    receipt_date        DATE,
    quantity            NUMERIC(18,6) NOT NULL DEFAULT 0,
    qualified_quantity  NUMERIC(18,6) NOT NULL DEFAULT 0,
    unqualified_quantity NUMERIC(18,6) NOT NULL DEFAULT 0,
    unit                VARCHAR(20)   NOT NULL,
    remark              TEXT,
    created_time        TIMESTAMPTZ   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_time        TIMESTAMPTZ   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    is_deleted          SMALLINT      NOT NULL DEFAULT 0
);


