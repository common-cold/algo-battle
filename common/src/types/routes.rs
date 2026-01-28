use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{Contest, ContestStatus, QuestionType, Role};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUserArgs {
    pub name: String,
    pub email: String,
    pub password: String,
    pub role: Role
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignInArgs {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateQuestionArgs {
    pub question_type: QuestionType,
    pub title: String,
    pub description: String,
    pub options: Vec<String>,
    pub correct_option: i16,
    pub time_limit: i64,
    pub points: i16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetQuestionArgs {
    pub page: Option<usize>,
    pub limit: Option<usize>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetQuestionsByIdArgs {
    pub ids: Vec<Uuid>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateContestArgs {
    pub title: String,
    pub description: String,
    pub start_date: i64,
    pub end_date: i64,
    pub question_ids: Vec<Uuid>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetContestArgs {
    pub page: Option<usize>,
    pub limit: Option<usize>,
    pub status: ContestStatus
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestionWithoutAnswer {
    pub id: Uuid,
    pub question_type: QuestionType,
    pub title: String,
    pub description: String,
    pub options: Vec<String>,
    pub time_limit: i64,
    pub points: i16,
    pub owner_id: Uuid,
    pub created_at: i64
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FullContest {
    pub contest: Contest,
    pub questions: Vec<QuestionWithoutAnswer>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetContestJoinedAt {
    #[serde(rename ="userId")]
    pub user_id: Uuid,
    #[serde(rename="contestId")]
    pub contest_id: Uuid
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmitQuestionArgs {
    pub contest_id: Uuid,
    pub question_id: Uuid,
    pub selected_option: i16
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateContestArgs {
    pub title: Option<String>,
    pub description: Option<String>,
    pub start_date: Option<i64>,
    pub end_date: Option<i64>,
    pub status: Option<ContestStatus>
}