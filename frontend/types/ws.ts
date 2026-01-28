export interface JoinContestArgs {
    contest_id: string
}

export type WebSocketMessage = 
    | {JoinContest: JoinContestArgs}


export type ResponseType = "Log" | "NextQuestion" | "EndContest";

export interface WebSocketResponse {
    data: ResponseData
}

export type ResponsePayload = LogArgs | NextQuestionArgs | EndContestArgs;

export interface LogArgs {
    message: string,
    isError: boolean
}

export interface NextQuestionArgs {
    question_id: string,
    contest_id: string,
    new_rank: number
}

export interface EndContestArgs {
    contest_id: string
}

export interface ResponseData {
    type: ResponseType;
    payload: ResponsePayload
}