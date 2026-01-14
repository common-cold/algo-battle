import { ContestStatus, QuestionType, Role } from "./db"

export interface CreateUserArgs {
    name: string,
    email: string,
    password: string,
    role: Role
}

export interface CreateQuestionArgs {
    question_type: QuestionType,
    title: string,
    description: string,
    options: Array<string>,
    correct_option: number,
    time_limit: number,
    points: number,
    owner_id: string
}

export interface GetQuestionArgs {
    page?: number,
    limit?: number,
    id: string
}

export interface GetQuestionsByIdArgs {
    ids: string[]
}

export interface CreateContestArgs {
    title: string,
    description: string,
    start_date: number,
    end_date: number,
    owner_id: string,
    question_ids: Array<String>
}

export interface GetContestsArgs {
    page?: number,
    limit?: number,
    id: string,
    status: ContestStatus
}