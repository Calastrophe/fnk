-- Add up migration script here

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE
    "tests" (
        id UUID NOT NULL PRIMARY KEY DEFAULT (uuid_generate_v4()),
        teacher UUID NOT NULL,
        name VARCHAR(100) NOT NULL,
        closed BOOL NOT NULL,

        FOREIGN KEY (teacher) REFERENCES teachers(id)

    );

CREATE TABLE
    "results" (
        id UUID NOT NULL PRIMARY KEY DEFAULT (uuid_generate_v4()),
        test UUID NOT NULL,
        name VARCHAR(100) NOT NULL,
        score INT NOT NULL,
        finished BOOL NOT NULL,
        flagged BOOL NOT NULL,

        FOREIGN KEY (test) REFERENCES tests(id)
    );
