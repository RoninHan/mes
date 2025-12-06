-- Master data extension: workshops, warehouses, locations, BOM and process routes

CREATE TABLE IF NOT EXISTS workshops (
    id           BIGSERIAL PRIMARY KEY,
    workshop_code VARCHAR(50)  NOT NULL UNIQUE,
    workshop_name VARCHAR(100) NOT NULL,
    workshop_type SMALLINT     NOT NULL DEFAULT 1,
    manager_id    BIGINT,
    status        SMALLINT     NOT NULL DEFAULT 1,
    remark        TEXT,
    created_time  TIMESTAMPTZ  NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_time  TIMESTAMPTZ  NOT NULL DEFAULT CURRENT_TIMESTAMP,
    is_deleted    SMALLINT     NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS warehouses (
    id             BIGSERIAL PRIMARY KEY,
    warehouse_code VARCHAR(50)  NOT NULL UNIQUE,
    warehouse_name VARCHAR(100) NOT NULL,
    warehouse_type SMALLINT     NOT NULL DEFAULT 1,
    location       VARCHAR(200),
    status         SMALLINT     NOT NULL DEFAULT 1,
    remark         TEXT,
    created_time   TIMESTAMPTZ  NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_time   TIMESTAMPTZ  NOT NULL DEFAULT CURRENT_TIMESTAMP,
    is_deleted     SMALLINT     NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS locations (
    id             BIGSERIAL PRIMARY KEY,
    warehouse_id   BIGINT       NOT NULL REFERENCES warehouses(id),
    location_code  VARCHAR(50)  NOT NULL,
    location_name  VARCHAR(100) NOT NULL,
    location_type  SMALLINT     NOT NULL DEFAULT 1,
    status         SMALLINT     NOT NULL DEFAULT 1,
    remark         TEXT,
    created_time   TIMESTAMPTZ  NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_time   TIMESTAMPTZ  NOT NULL DEFAULT CURRENT_TIMESTAMP,
    is_deleted     SMALLINT     NOT NULL DEFAULT 0,
    CONSTRAINT uk_locations_unique UNIQUE (warehouse_id, location_code)
);

CREATE TABLE IF NOT EXISTS boms (
    id           BIGSERIAL PRIMARY KEY,
    material_id  BIGINT      NOT NULL REFERENCES materials(id),
    bom_code     VARCHAR(50) NOT NULL,
    version      VARCHAR(20) NOT NULL,
    bom_type     SMALLINT    NOT NULL DEFAULT 1,
    is_default   SMALLINT    NOT NULL DEFAULT 0,
    status       SMALLINT    NOT NULL DEFAULT 1,
    items        JSONB       NOT NULL,
    remark       TEXT,
    created_time TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_time TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    is_deleted   SMALLINT    NOT NULL DEFAULT 0,
    CONSTRAINT uk_boms_material_version UNIQUE (material_id, version)
);

CREATE TABLE IF NOT EXISTS process_routes (
    id           BIGSERIAL PRIMARY KEY,
    material_id  BIGINT      NOT NULL REFERENCES materials(id),
    route_code   VARCHAR(50) NOT NULL,
    route_name   VARCHAR(100) NOT NULL,
    version      VARCHAR(20) NOT NULL,
    is_default   SMALLINT    NOT NULL DEFAULT 0,
    status       SMALLINT    NOT NULL DEFAULT 1,
    operations   JSONB       NOT NULL,
    remark       TEXT,
    created_time TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_time TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    is_deleted   SMALLINT    NOT NULL DEFAULT 0,
    CONSTRAINT uk_process_routes_material_version UNIQUE (material_id, version)
);



