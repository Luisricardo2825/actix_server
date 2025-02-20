CREATE TABLE IF NOT EXISTS fields (
    id serial NOT NULL,
    name varchar(255) NOT NULL,
    description text NULL,
    field_type varchar(255) NOT NULL,
    table_id int NOT NULL,
    is_required boolean NOT NULL DEFAULT FALSE,
    is_primary_key boolean NOT NULL DEFAULT FALSE,
    is_auto_increment boolean NOT NULL DEFAULT FALSE,
    is_generated boolean NOT NULL DEFAULT FALSE,
    default_value varchar NULL DEFAULT '',
    custom_expression text NULL,
    is_unique boolean NOT NULL DEFAULT FALSE,
    created_at timestamp NULL DEFAULT NOW(),
    updated_at timestamp NULL DEFAULT NOW(),
    CONSTRAINT PK_fields PRIMARY KEY (id),
    CONSTRAINT FK_fields_1 FOREIGN KEY (table_id) REFERENCES tables (id) ON DELETE CASCADE
);