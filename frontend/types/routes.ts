import { Contest, ContestStatus, Question, QuestionType, Role, Submission } from "./db"

export const API_BASE_URL = 'https://algo-battle.prajjwalk.com:8080';
export const WS_BASE_URL = 'wss://algo-battle.prajjwalk.com/ws';

export interface SignupArgs {
    name: string,
    email: string,
    password: string,
    role: Role
}

export interface SignInArgs {
    email: string,
    password: string,
}

export interface CreateQuestionArgs {
    question_type: QuestionType,
    title: string,
    description: string,
    options: Array<string>,
    correct_option: number,
    time_limit: number,
    points: number
}

export interface GetQuestionArgs {
    page?: number,
    limit?: number
}

export interface GetQuestionsByIdArgs {
    ids: string[]
}

export interface CreateContestArgs {
    title: string,
    description: string,
    start_date: number,
    end_date: number
    question_ids?: Array<String>
}

export interface GetContestsArgs {
    page?: number,
    limit?: number,
    status: ContestStatus
}

export interface FullContest {
    contest: Contest,
    questions: Question[]
}

export interface SubmitQuestionArgs {
    contest_id: string,
    question_id: string,
    selected_option: number
}

export interface SubmitDsaQuestionArgs {
    contest_id: string,
    question_id: string,
    attempt_id: string
}

export interface EvaluateDsaQuestionArgs {
    contest_id: string,
    question_id: string,
    attempt_id: string
    code: string,
    language_id: number
}

export interface CompactTestcase {
    input: string,
    output: string,
}

export interface SubmissionStatusResponse {
    total_testcases: number,
    pending: number,
    failed: number,
    passed: number,
    failed_testcase?: CompactTestcase,
    compile_result?: string
}

export interface FetchAttemptIdArgs {
    contest_id: string,
    problem_id: string
}    

export interface FetchAttemptResponse {
    attempt_id?: string,
    found: boolean 
}

export interface GetSubmissionResponse {
    found: boolean,
    submission?: Submission
}