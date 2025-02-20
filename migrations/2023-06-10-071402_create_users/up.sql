CREATE TABLE IF NOT EXISTS users
(
 id       serial NOT NULL,
 name       varchar(255) NOT NULL,
 email      varchar(255) NOT NULL,
 password   varchar(255) NOT NULL,
 blocked  boolean NOT NULL DEFAULT FALSE,
 api_rights boolean NOT NULL DEFAULT FALSE,
 admin      boolean NOT NULL DEFAULT FALSE,
 created_at timestamp NULL DEFAULT CURRENT_TIMESTAMP,
 updated_at timestamp NULL DEFAULT CURRENT_TIMESTAMP,
 picture    text,
 CONSTRAINT PK_users PRIMARY KEY ( id )
);