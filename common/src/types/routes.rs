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
pub struct CreateMcqQuestionArgs {
    pub question_type: QuestionType,
    pub title: String,
    pub description: String,
    pub options: Vec<String>,
    pub correct_option: i16,
    pub time_limit: i64,
    pub points: i32,
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
    pub options: Option<Vec<String>>,
    pub time_limit: i64,
    pub points: i32,
    pub testcase_input: Option<String>,
    pub testcase_output: Option<String>
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
pub struct SubmitDsaQuestionArgs {
    pub contest_id: Uuid,
    pub question_id: Uuid,
    pub attempt_id: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvaluateDsaQuestionArgs {
    pub contest_id: Uuid,
    pub question_id: Uuid,
    pub attempt_id: String,
    pub code: String,
    pub language_id: i16
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateContestArgs {
    pub title: Option<String>,
    pub description: Option<String>,
    pub start_date: Option<i64>,
    pub end_date: Option<i64>,
    pub status: Option<ContestStatus>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDsaQuestionArgs {
    pub title: String,
    pub description: String,
    pub time_limit: i64,
    pub points: i32,
    pub testcase_input: String,
    pub testcase_output: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateBulkBoilerPlateCodeArgs {
    pub problem_id: Uuid,
    pub language_ids: Vec<i16>,
    pub partial_codes: Vec<String>,
    pub full_codes: Vec<String>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateBulkTestCasesArgs {
    pub problem_id: Uuid,
    pub inputs: Vec<String>,
    pub outputs: Vec<String>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDsaSubmissionArgs {
    pub attempt_id: String,
    pub submission_id: String,
    pub problem_id: Uuid,
    pub contest_id: Uuid,
    pub testcase_id: Uuid,
    pub user_id: Uuid,
    pub status: String, 
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSubmissionArgs {
    pub contest_id: Uuid,
    pub question_id: Uuid,
    pub user_id: Uuid,
    pub points_earned: i32
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Judge0SubmissionDto {
    pub source_code: String,
    pub language_id: i16,
    pub stdin: String,
    pub expected_output:String,
    pub callback_url: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Judge0BatchSubmissionDto {
    pub submissions: Vec<Judge0SubmissionDto>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Judge0SubmissionResponseDto {
    pub token: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Judge0SubmissionCallBackArgs {
  pub stdout: Option<String>,
  pub time: Option<String>,
  pub memory: Option<i64>,
  pub stderr: Option<String>,
  pub token: String,
  pub compile_output: Option<String>,
  pub message: Option<String>,
  pub status: Judge0StatusDto
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Judge0StatusDto {
    pub id: i16,
    pub description: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompactTestcase {
    pub input: String,
    pub output: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmissionStatusResponse {
    pub total_testcases: i16,
    pub pending: i16,
    pub failed: i16,
    pub passed: i16,
    pub failed_testcase: Option<CompactTestcase>,
    pub compile_result: Option<String>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FetchAttemptIdArgs {
    pub contest_id: Uuid,
    pub problem_id: Uuid
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSubmissionParams {
    #[serde(rename="contestId")]
    pub contest_id: Uuid,

    #[serde(rename="questionId")]
    pub question_id: Uuid
}