use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, sqlx::Type)]
#[sqlx(type_name = "role")]
#[sqlx(rename_all = "PascalCase")]
pub enum Role {
    Examiner,
    Candidate
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, sqlx::Type)]
#[sqlx(type_name = "question_type")]
#[sqlx(rename_all = "PascalCase")]
pub enum QuestionType {
    Mcq,
    Dsa,
    LiveAssignment
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, sqlx::Type)]
#[sqlx(type_name = "contest_status")]
#[sqlx(rename_all = "PascalCase")]
pub enum ContestStatus {
    Scheduled,
    Active,
    Closed
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
    pub role: Role,
    pub created_at: i64
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Question {
    pub id: Uuid,
    pub question_type: QuestionType,
    pub owner_id: Option<Uuid>,
    pub created_at: i64
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct McqQuestion {
    pub id: Uuid,
    pub question_type: QuestionType,
    pub title: String,
    pub description: String,
    pub options: Vec<String>,
    pub correct_option: i16,
    pub time_limit: i64,
    pub points: i16
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DsaQuestion {
    pub id: Uuid,
    pub question_type: QuestionType,
    pub title: String,
    pub description: String,
    pub time_limit: i64,
    pub points: i16,
    pub testcase_input: String,
    pub testcase_output: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Contest {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub start_date: i64,
    pub end_date: i64,
    pub status: ContestStatus,
    pub owner_id: Uuid,
    pub created_at: i64
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Leaderboard {
    pub contest_id: Uuid,
    pub user_id: Uuid,
    pub score: i16,
    pub rank: i16,
    pub created_at: i64
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BoilerplateCode {
    pub id: Uuid,
    pub problem_id: Uuid,
    pub language_id: i16,
    pub partial_code: String,
    pub full_code: String,
    pub created_at: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TestCase {
    pub id: Uuid,
    pub problem_id: Uuid,
    pub input: String,
    pub output: String,
    pub created_at: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Submission {
    pub id: Uuid,
    pub problem_id: Uuid,
    pub contest_id: Uuid,
    pub user_id: Uuid,
    pub status: String,
    pub created_at: i64,
}