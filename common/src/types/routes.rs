use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{ContestStatus, QuestionType, Role};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUserArgs {
    pub name: String,
    pub email: String,
    pub password: String,
    pub role: Role
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
    pub owner_id: Uuid
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetQuestionArgs {
    pub page: Option<usize>,
    pub limit: Option<usize>,
    pub id: Uuid
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
    pub owner_id: Uuid,
    pub question_ids: Vec<Uuid>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetContestArgs {
    pub page: Option<usize>,
    pub limit: Option<usize>,
    pub id: Uuid,
    pub status: ContestStatus
}