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
    pub owner_id: Uuid
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateContestArgs {
    pub title: String,
    pub description: String,
    pub start_date: i64,
    pub end_date: i64,
    pub status: ContestStatus,
    pub owner_id: Uuid,
    pub created_at: i64
}