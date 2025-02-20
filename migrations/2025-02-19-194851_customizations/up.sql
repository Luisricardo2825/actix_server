-- Your SQL goes here
CREATE TABLE IF NOT EXISTS customizations
(
 id       serial NOT NULL,
 path     text NOT NULL,
 table_id   int NOT NULL,
 run_on     varchar(50) NOT NULL,
 created_at timestamp NOT NULL,
 updated_at timestamp NOT NULL,
 created_by int NOT NULL,
 updated_by int NOT NULL,
 CONSTRAINT PK_6 PRIMARY KEY ( id ),
 CONSTRAINT FK_1 FOREIGN KEY ( created_by ) REFERENCES users ( id ),
 CONSTRAINT FK_2 FOREIGN KEY ( updated_by ) REFERENCES users ( id ),
 CONSTRAINT FK_3 FOREIGN KEY ( table_id ) REFERENCES tables ( id )
);
