use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::FullContest;



#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum WebSocketMessage {
    StartContest(FullContest),
    JoinContest(JoinContestArgs),

    Debug
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JoinContestArgs {
    pub contest_id: Uuid
}


#[derive(Debug, Serialize, Deserialize)]
pub struct WebSocketResponse {
    pub data: ResponseData
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload")]
pub enum ResponseData {
    Log(LogArgs),
    NextQuestion(NextQuestionArgs),
    EndContest(EndContestArgs)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogArgs {
    pub message: String,

    #[serde(rename="isError")]
    pub is_error: bool
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NextQuestionArgs {
    pub question_id: Uuid,
    pub contest_id: Uuid,
    pub new_rank: i32
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EndContestArgs {
    pub contest_id: Uuid
}