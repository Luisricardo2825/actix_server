-- Your SQL goes here
CREATE TABLE IF NOT EXISTS fields (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    field_type VARCHAR(255) NOT NULL,
    table_id INT NOT NULL,
    is_required BOOLEAN NOT NULL DEFAULT FALSE,
    is_primary_key BOOLEAN NOT NULL DEFAULT FALSE,
    is_auto_increment BOOLEAN NOT NULL DEFAULT FALSE,
    is_generated BOOLEAN NOT NULL DEFAULT FALSE,
    default_value VARCHAR DEFAULT '',
    custom_expression TEXT,
    is_unique BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW(),
    FOREIGN KEY (table_id) REFERENCES tables(id)
);