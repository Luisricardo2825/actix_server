-- Your SQL goes here
CREATE TABLE IF NOT EXISTS users_permissions (
    id serial NOT NULL,
    user_id int NOT NULL,
    permission varchar(9) NOT NULL,
    allow boolean NOT NULL,
    CONSTRAINT PK_7 PRIMARY KEY (id),
    CONSTRAINT FK_1 FOREIGN KEY (user_id) REFERENCES users (id)
);