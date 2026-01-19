CREATE TABLE CONTEST_QUESTION (
    contest_id UUID NOT NULL REFERENCES contests(id) ON DELETE CASCADE,
    question_id UUID NOT NULL REFERENCES questions(id) ON DELETE CASCADE,
    position INT NOT NULL,
    PRIMARY KEY(contest_id, question_id),
    UNIQUE(contest_id, position)
)