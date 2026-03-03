CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE LEADERBOARD (
    contest_id   UUID NOT NULL,
    user_id      UUID NOT NULL,
    score        INTEGER NOT NULL,
    rank         INTEGER NOT NULL,
    created_at   BIGINT NOT NULL DEFAULT EXTRACT(EPOCH FROM NOW()),
    PRIMARY KEY  (contest_id, user_id)
)