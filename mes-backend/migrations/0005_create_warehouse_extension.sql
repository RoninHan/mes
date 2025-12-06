-- Warehouse extension: outbound, transfer, stock count, adjustment, reservations, transactions, alerts, batch traceability

CREATE TABLE IF NOT EXISTS outbound_orders (
    id                BIGSERIAL PRIMARY KEY,
    outbound_no       VARCHAR(50)  NOT NULL UNIQUE,
    outbound_type     SMALLINT     NOT NULL, -- 1:销售出库,2:生产领料,3:调拨出库,4:退货出库,5:其他
    warehouse_id      BIGINT       NOT NULL REFERENCES warehouses(id),
    customer_id       BIGINT,
    plan_outbound_date DATE,
    actual_outbound_date DATE,
    total_quantity    NUMERIC(18,6) NOT NULL DEFAULT 0,
    order_status      SMALLINT      NOT NULL DEFAULT 1, -- 1:待出库,2:部分出库,3:已完成,4:已取消
    remark            TEXT,
    created_time      TIMESTAMPTZ  NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_time      TIMESTAMPTZ  NOT NULL DEFAULT CURRENT_TIMESTAMP,
    is_deleted        SMALLINT     NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS outbound_order_lines (
    id                BIGSERIAL PRIMARY KEY,
    outbound_id       BIGINT       NOT NULL REFERENCES outbound_orders(id) ON DELETE CASCADE,
    material_id       BIGINT       NOT NULL REFERENCES materials(id),
    warehouse_id      BIGINT       NOT NULL REFERENCES warehouses(id),
    location_id       BIGINT       REFERENCES locations(id),
    batch_no          VARCHAR(100),
    plan_quantity     NUMERIC(18,6) NOT NULL DEFAULT 0,
    actual_quantity   NUMERIC(18,6) NOT NULL DEFAULT 0,
    unit              VARCHAR(20)   NOT NULL,
    line_status       SMALLINT      NOT NULL DEFAULT 1, -- 1:待出库,2:部分出库,3:已完成,4:已取消
    remark            TEXT,
    created_time      TIMESTAMPTZ  NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_time      TIMESTAMPTZ  NOT NULL DEFAULT CURRENT_TIMESTAMP,
    is_deleted        SMALLINT     NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS transfer_orders (
    id                BIGSERIAL PRIMARY KEY,
    transfer_no       VARCHAR(50)  NOT NULL UNIQUE,
    from_warehouse_id BIGINT       NOT NULL REFERENCES warehouses(id),
    to_warehouse_id   BIGINT       NOT NULL REFERENCES warehouses(id),
    plan_transfer_date DATE,
    actual_transfer_date DATE,
    total_quantity    NUMERIC(18,6) NOT NULL DEFAULT 0,
    order_status      SMALLINT      NOT NULL DEFAULT 1, -- 1:待调拨,2:部分调拨,3:已完成,4:已取消
    remark            TEXT,
    created_time      TIMESTAMPTZ  NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_time      TIMESTAMPTZ  NOT NULL DEFAULT CURRENT_TIMESTAMP,
    is_deleted        SMALLINT     NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS transfer_order_lines (
    id                BIGSERIAL PRIMARY KEY,
    transfer_id       BIGINT       NOT NULL REFERENCES transfer_orders(id) ON DELETE CASCADE,
    material_id       BIGINT       NOT NULL REFERENCES materials(id),
    from_warehouse_id BIGINT       NOT NULL REFERENCES warehouses(id),
    from_location_id  BIGINT       REFERENCES locations(id),
    to_warehouse_id   BIGINT       NOT NULL REFERENCES warehouses(id),
    to_location_id    BIGINT       REFERENCES locations(id),
    batch_no          VARCHAR(100),
    plan_quantity     NUMERIC(18,6) NOT NULL DEFAULT 0,
    actual_quantity   NUMERIC(18,6) NOT NULL DEFAULT 0,
    unit              VARCHAR(20)   NOT NULL,
    line_status       SMALLINT      NOT NULL DEFAULT 1,
    remark            TEXT,
    created_time      TIMESTAMPTZ  NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_time      TIMESTAMPTZ  NOT NULL DEFAULT CURRENT_TIMESTAMP,
    is_deleted        SMALLINT     NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS stock_count_orders (
    id                BIGSERIAL PRIMARY KEY,
    count_no          VARCHAR(50)  NOT NULL UNIQUE,
    warehouse_id      BIGINT       NOT NULL REFERENCES warehouses(id),
    count_type        SMALLINT     NOT NULL DEFAULT 1, -- 1:全盘,2:抽盘
    plan_count_date   DATE,
    actual_count_date DATE,
    order_status      SMALLINT     NOT NULL DEFAULT 1, -- 1:待盘点,2:盘点中,3:已完成,4:已取消
    remark            TEXT,
    created_time      TIMESTAMPTZ  NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_time      TIMESTAMPTZ  NOT NULL DEFAULT CURRENT_TIMESTAMP,
    is_deleted        SMALLINT     NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS stock_count_lines (
    id                BIGSERIAL PRIMARY KEY,
    count_id          BIGINT       NOT NULL REFERENCES stock_count_orders(id) ON DELETE CASCADE,
    material_id       BIGINT       NOT NULL REFERENCES materials(id),
    warehouse_id      BIGINT       NOT NULL REFERENCES warehouses(id),
    location_id       BIGINT       REFERENCES locations(id),
    batch_no          VARCHAR(100),
    book_quantity     NUMERIC(18,6) NOT NULL DEFAULT 0,
    counted_quantity  NUMERIC(18,6) NOT NULL DEFAULT 0,
    diff_quantity     NUMERIC(18,6) NOT NULL DEFAULT 0,
    unit              VARCHAR(20)   NOT NULL,
    line_status       SMALLINT      NOT NULL DEFAULT 1,
    remark            TEXT,
    created_time      TIMESTAMPTZ  NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_time      TIMESTAMPTZ  NOT NULL DEFAULT CURRENT_TIMESTAMP,
    is_deleted        SMALLINT     NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS stock_adjustments (
    id                BIGSERIAL PRIMARY KEY,
    adjustment_no     VARCHAR(50)  NOT NULL UNIQUE,
    warehouse_id      BIGINT       NOT NULL REFERENCES warehouses(id),
    location_id       BIGINT       REFERENCES locations(id),
    material_id       BIGINT       NOT NULL REFERENCES materials(id),
    batch_no          VARCHAR(100),
    quantity_delta    NUMERIC(18,6) NOT NULL,
    unit              VARCHAR(20)   NOT NULL,
    reason            VARCHAR(200),
    adjustment_type   SMALLINT      NOT NULL DEFAULT 1, -- 1:盘盈,2:盘亏,3:报废,4:其他
    business_time     TIMESTAMPTZ   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    status            SMALLINT      NOT NULL DEFAULT 1, -- 1:待确认,2:已生效,3:已取消
    remark            TEXT,
    created_time      TIMESTAMPTZ  NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_time      TIMESTAMPTZ  NOT NULL DEFAULT CURRENT_TIMESTAMP,
    is_deleted        SMALLINT     NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS stock_reservations (
    id                BIGSERIAL PRIMARY KEY,
    reservation_no    VARCHAR(50)  NOT NULL UNIQUE,
    material_id       BIGINT       NOT NULL REFERENCES materials(id),
    warehouse_id      BIGINT       NOT NULL REFERENCES warehouses(id),
    location_id       BIGINT       REFERENCES locations(id),
    batch_no          VARCHAR(100),
    reserved_quantity NUMERIC(18,6) NOT NULL,
    unit              VARCHAR(20)   NOT NULL,
    source_type       SMALLINT      NOT NULL, -- 1:生产,2:销售,3:维修,4:其他
    source_id         BIGINT,
    status            SMALLINT      NOT NULL DEFAULT 1, -- 1:生效,2:已释放,3:已关闭
    expire_time       TIMESTAMPTZ,
    remark            TEXT,
    created_time      TIMESTAMPTZ  NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_time      TIMESTAMPTZ  NOT NULL DEFAULT CURRENT_TIMESTAMP,
    is_deleted        SMALLINT     NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS stock_transactions (
    id                BIGSERIAL PRIMARY KEY,
    trans_type        SMALLINT     NOT NULL, -- 1:入库,2:出库,3:调拨出,4:调拨入,5:盘盈,6:盘亏,7:调整
    ref_type          SMALLINT     NOT NULL, -- 1:入库单,2:出库单,3:调拨单,4:盘点单,5:调整单,6:其他
    ref_id            BIGINT,
    material_id       BIGINT       NOT NULL REFERENCES materials(id),
    warehouse_id      BIGINT       NOT NULL REFERENCES warehouses(id),
    location_id       BIGINT       REFERENCES locations(id),
    batch_no          VARCHAR(100),
    quantity_delta    NUMERIC(18,6) NOT NULL,
    unit              VARCHAR(20)   NOT NULL,
    before_quantity   NUMERIC(18,6),
    after_quantity    NUMERIC(18,6),
    business_time     TIMESTAMPTZ   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    remark            TEXT,
    created_time      TIMESTAMPTZ  NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_time      TIMESTAMPTZ  NOT NULL DEFAULT CURRENT_TIMESTAMP,
    is_deleted        SMALLINT     NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS stock_alert_rules (
    id                BIGSERIAL PRIMARY KEY,
    material_id       BIGINT       NOT NULL REFERENCES materials(id),
    warehouse_id      BIGINT       NOT NULL REFERENCES warehouses(id),
    min_quantity      NUMERIC(18,6),
    max_quantity      NUMERIC(18,6),
    enabled           SMALLINT     NOT NULL DEFAULT 1,
    remark            TEXT,
    created_time      TIMESTAMPTZ  NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_time      TIMESTAMPTZ  NOT NULL DEFAULT CURRENT_TIMESTAMP,
    is_deleted        SMALLINT     NOT NULL DEFAULT 0,
    CONSTRAINT uk_stock_alert UNIQUE(material_id, warehouse_id)
);

CREATE TABLE IF NOT EXISTS batch_traceability (
    id                BIGSERIAL PRIMARY KEY,
    material_id       BIGINT       NOT NULL REFERENCES materials(id),
    batch_no          VARCHAR(100) NOT NULL,
    source_type       SMALLINT     NOT NULL, -- 1:采购,2:生产,3:退货,4:其他
    source_ref        VARCHAR(100),
    current_warehouse_id BIGINT    REFERENCES warehouses(id),
    current_location_id  BIGINT    REFERENCES locations(id),
    quantity          NUMERIC(18,6),
    unit              VARCHAR(20),
    status            SMALLINT     NOT NULL DEFAULT 1,
    remark            TEXT,
    created_time      TIMESTAMPTZ  NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_time      TIMESTAMPTZ  NOT NULL DEFAULT CURRENT_TIMESTAMP,
    is_deleted        SMALLINT     NOT NULL DEFAULT 0
);


