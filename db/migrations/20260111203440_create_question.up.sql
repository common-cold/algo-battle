CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TYPE QUESTION_TYPE AS ENUM('Mcq', 'Dsa', 'LiveAssignment');

CREATE TABLE QUESTIONS (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    question_type QUESTION_TYPE NOT NULL,
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    options TEXT[] NOT NULL,
    correct_option SMALLINT NOT NULL,
    owner_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    created_at BIGINT NOT NULL DEFAULT EXTRACT(EPOCH FROM NOW())
)