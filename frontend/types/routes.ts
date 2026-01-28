import { Contest, ContestStatus, Question, QuestionType, Role } from "./db"

export const API_BASE_URL = 'http://localhost:8080';
export const WS_BASE_URL = 'http://localhost:8081';

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