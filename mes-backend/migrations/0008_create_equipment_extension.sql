-- Equipment extension: maintenance plans/tasks, fault reports, repair orders, inspections, KPI

CREATE TABLE IF NOT EXISTS equipment_maintenance_plans (
    id              BIGSERIAL PRIMARY KEY,
    plan_no         VARCHAR(50)  NOT NULL UNIQUE,
    equipment_id    BIGINT       NOT NULL REFERENCES equipment(id),
    plan_type       SMALLINT     NOT NULL, -- 1:保养,2:点检,3:校准,4:其他
    cycle_type      SMALLINT     NOT NULL, -- 1:按时间,2:按运行小时,3:按次数
    cycle_value     INTEGER      NOT NULL,
    next_due_time   TIMESTAMPTZ,
    status          SMALLINT     NOT NULL DEFAULT 1, -- 1:启用,0:停用
    remark          TEXT,
    created_time    TIMESTAMPTZ  NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_time    TIMESTAMPTZ  NOT NULL DEFAULT CURRENT_TIMESTAMP,
    is_deleted      SMALLINT     NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS equipment_maintenance_tasks (
    id              BIGSERIAL PRIMARY KEY,
    task_no         VARCHAR(50)  NOT NULL UNIQUE,
    plan_id         BIGINT       REFERENCES equipment_maintenance_plans(id),
    equipment_id    BIGINT       NOT NULL REFERENCES equipment(id),
    task_type       SMALLINT     NOT NULL, -- 1:保养,2:点检,3:校准,4:临时
    scheduled_time  TIMESTAMPTZ,
    start_time      TIMESTAMPTZ,
    end_time        TIMESTAMPTZ,
    result          SMALLINT, -- 1:完成,2:延期,3:取消
    status          SMALLINT     NOT NULL DEFAULT 1, -- 1:待执行,2:执行中,3:已完成,4:已关闭
    remark          TEXT,
    created_time    TIMESTAMPTZ  NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_time    TIMESTAMPTZ  NOT NULL DEFAULT CURRENT_TIMESTAMP,
    is_deleted      SMALLINT     NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS equipment_fault_reports (
    id              BIGSERIAL PRIMARY KEY,
    fault_no        VARCHAR(50)  NOT NULL UNIQUE,
    equipment_id    BIGINT       NOT NULL REFERENCES equipment(id),
    fault_level     SMALLINT     NOT NULL, -- 1:一般,2:严重,3:致命
    occur_time      TIMESTAMPTZ  NOT NULL,
    report_time     TIMESTAMPTZ  NOT NULL DEFAULT CURRENT_TIMESTAMP,
    reporter_id     BIGINT,
    description     TEXT,
    status          SMALLINT     NOT NULL DEFAULT 1, -- 1:待响应,2:处理中,3:已恢复,4:已关闭
    root_cause      TEXT,
    solution        TEXT,
    remark          TEXT,
    created_time    TIMESTAMPTZ  NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_time    TIMESTAMPTZ  NOT NULL DEFAULT CURRENT_TIMESTAMP,
    is_deleted      SMALLINT     NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS equipment_repair_orders (
    id              BIGSERIAL PRIMARY KEY,
    repair_no       VARCHAR(50)  NOT NULL UNIQUE,
    fault_id        BIGINT       REFERENCES equipment_fault_reports(id),
    equipment_id    BIGINT       NOT NULL REFERENCES equipment(id),
    repair_type     SMALLINT     NOT NULL DEFAULT 1, -- 1:故障维修,2:预防性维修,3:改造
    start_time      TIMESTAMPTZ,
    end_time        TIMESTAMPTZ,
    downtime_minutes INTEGER     DEFAULT 0,
    repair_person_id BIGINT,
    cost_labor      NUMERIC(18,6) DEFAULT 0,
    cost_spare_parts NUMERIC(18,6) DEFAULT 0,
    status          SMALLINT      NOT NULL DEFAULT 1, -- 1:待维修,2:维修中,3:已完成,4:已关闭
    remark          TEXT,
    created_time    TIMESTAMPTZ   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_time    TIMESTAMPTZ   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    is_deleted      SMALLINT      NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS equipment_inspections (
    id              BIGSERIAL PRIMARY KEY,
    inspection_no   VARCHAR(50)  NOT NULL UNIQUE,
    equipment_id    BIGINT       NOT NULL REFERENCES equipment(id),
    inspection_type SMALLINT     NOT NULL, -- 1:点检,2:巡检,3:专项
    inspection_time TIMESTAMPTZ  NOT NULL DEFAULT CURRENT_TIMESTAMP,
    inspector_id    BIGINT,
    result          SMALLINT     NOT NULL, -- 1:正常,2:异常
    items           JSONB,
    remark          TEXT,
    created_time    TIMESTAMPTZ  NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_time    TIMESTAMPTZ  NOT NULL DEFAULT CURRENT_TIMESTAMP,
    is_deleted      SMALLINT     NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS equipment_kpi (
    id              BIGSERIAL PRIMARY KEY,
    equipment_id    BIGINT       NOT NULL REFERENCES equipment(id),
    stat_date       DATE         NOT NULL,
    runtime_minutes INTEGER      DEFAULT 0,
    downtime_minutes INTEGER     DEFAULT 0,
    fault_count     INTEGER      DEFAULT 0,
    mtbf_minutes    INTEGER      DEFAULT 0,
    mttr_minutes    INTEGER      DEFAULT 0,
    oee             NUMERIC(10,4),
    remark          TEXT,
    created_time    TIMESTAMPTZ  NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_time    TIMESTAMPTZ  NOT NULL DEFAULT CURRENT_TIMESTAMP,
    is_deleted      SMALLINT     NOT NULL DEFAULT 0,
    CONSTRAINT uk_equipment_kpi UNIQUE (equipment_id, stat_date)
);



