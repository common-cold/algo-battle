CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE DSA_QUESTIONS (
    id UUID PRIMARY KEY REFERENCES questions(id) ON DELETE CASCADE,
    question_type QUESTION_TYPE NOT NULL,
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    time_limit BIGINT NOT NULL,
    points SMALLINT NOT NULL,
    testcase_input TEXT NOT NULL,
    testcase_output TEXT NOT NULL
)