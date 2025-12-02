-- Production schedules table for MES APS / Gantt

CREATE TABLE IF NOT EXISTS production_schedules (
    id SERIAL PRIMARY KEY,
    work_order_id BIGINT NOT NULL,
    equipment_id BIGINT,
    workshop_id BIGINT,
    start_time TIMESTAMPTZ NOT NULL,
    end_time   TIMESTAMPTZ NOT NULL,
    status SMALLINT NOT NULL DEFAULT 1, -- 1: planned, 2: locked, 3: running, 4: done, 5: cancelled
    priority SMALLINT NOT NULL DEFAULT 3,
    remark TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);


