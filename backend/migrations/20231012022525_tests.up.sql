-- Add up migration script here

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE
    "tests" (
        id UUID NOT NULL PRIMARY KEY DEFAULT (uuid_generate_v4()),
        teacher UUID NOT NULL,
        name VARCHAR(100),

        FOREIGN KEY (teacher) REFERENCES teachers(id)
    );

CREATE TABLE
    "testresults" (
        id UUID NOT NULL PRIMARY KEY DEFAULT (uuid_generate_v4()),
        test UUID NOT NULL,
        name VARCHAR(100),
        score INT,

        FOREIGN KEY (test) REFERENCES tests(id)
    );
