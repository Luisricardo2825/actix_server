-- Your SQL goes here
CREATE TABLE IF NOT EXISTS tables_permissions (
    id serial NOT NULL,
    table_id int NOT NULL,
    permission varchar(9) NOT NULL,
    allow boolean NOT NULL,
    CONSTRAINT PK_5 PRIMARY KEY (id),
    CONSTRAINT FK_1 FOREIGN KEY (table_id) REFERENCES tables (id) ON DELETE CASCADE
);