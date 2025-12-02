-- PostgreSQL schema for equipment & MQTT related tables

CREATE TABLE IF NOT EXISTS equipment (
    id SERIAL PRIMARY KEY,
    equipment_code VARCHAR(50) NOT NULL UNIQUE,
    equipment_name VARCHAR(100) NOT NULL,
    equipment_type VARCHAR(50) NOT NULL,
    model VARCHAR(50),
    factory VARCHAR(100),
    production_date DATE,
    install_date DATE,
    status SMALLINT NOT NULL DEFAULT 0,
    ip_address VARCHAR(50),
    mqtt_topic VARCHAR(100) NOT NULL,
    location VARCHAR(100),
    responsible_person VARCHAR(50),
    remark TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS equipment_status_log (
    id SERIAL PRIMARY KEY,
    equipment_id INT NOT NULL,
    status SMALLINT NOT NULL,
    running_param JSONB,
    error_code VARCHAR(50),
    error_desc TEXT,
    log_time TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT fk_equipment_status_log_equipment
        FOREIGN KEY (equipment_id) REFERENCES equipment(id)
);

CREATE TABLE IF NOT EXISTS equipment_mqtt_config (
    id SERIAL PRIMARY KEY,
    equipment_id INT NOT NULL UNIQUE,
    broker_address VARCHAR(100) NOT NULL,
    username VARCHAR(50),
    password VARCHAR(100),
    client_id VARCHAR(50) NOT NULL,
    keep_alive INT DEFAULT 60,
    qos SMALLINT DEFAULT 1,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT fk_equipment_mqtt_config_equipment
        FOREIGN KEY (equipment_id) REFERENCES equipment(id)
);


