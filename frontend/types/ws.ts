export type ResponseType = "Log";

export interface JoinContestArgs {
    contest_id: string
}

export interface WebSocketResponse {
    data: ResponseData
}

export type ResponsePayload = LogArgs;

export interface LogArgs {
    message: string,
    isError: boolean
}

export interface ResponseData {
    type: ResponseType;
    payload: ResponsePayload
}