CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE SUBMISSIONS (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    contest_id UUID NOT NULL REFERENCES contests(id) ON DELETE CASCADE,
    question_id UUID NOT NULL REFERENCES questions(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    points_earned INTEGER NOT NULL,
    created_at BIGINT NOT NULL DEFAULT EXTRACT(EPOCH FROM NOW()),
    UNIQUE(contest_id, question_id, user_id)
);