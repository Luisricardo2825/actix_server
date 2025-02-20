CREATE TABLE IF NOT EXISTS tables
(
 id        serial NOT NULL,
 name        varchar(255) NOT NULL,
 description text NOT NULL,
 view_sql    text NULL,
 capacity    int NULL,
 is_view     boolean NOT NULL DEFAULT FALSE,
 is_active   boolean NOT NULL DEFAULT TRUE,
 is_deleted  boolean NOT NULL DEFAULT FALSE,
 created_at  timestamp NULL DEFAULT NOW(),
 updated_at  timestamp NULL DEFAULT NOW(),
 CONSTRAINT PK_tables PRIMARY KEY ( id )
);