CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE JUDGE0_SUBMISSIONS (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    attempt_id TEXT NOT NULL,
    submission_id TEXT NOT NULL,
    problem_id UUID NOT NULL REFERENCES dsa_questions(id) ON DELETE CASCADE,
    contest_id UUID NOT NULL REFERENCES contests(id) ON DELETE CASCADE,
    testcase_id UUID NOT NULL REFERENCES test_cases(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    status TEXT NOT NULL,
    compile_result TEXT,
    created_at BIGINT NOT NULL DEFAULT EXTRACT(EPOCH FROM NOW())
)