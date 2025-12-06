-- User module extension: login logs and operation audit logs

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



