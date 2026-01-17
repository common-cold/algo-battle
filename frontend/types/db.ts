export type Role = "Examiner" | "Candidate";

export type QuestionType = "Mcq" | "Dsa" | "LiveAssignment";

export type ContestStatus = "Scheduled" | "Active" | "Closed";


export interface User {
    id: string,
    name: string,
    email: string,
    password: string,
    role: Role,
    created_at: number
}

export interface Question {
    id: string,
    question_type: QuestionType,
    title: string,
    description: string,
    options: Array<string>,
    correct_option?: number,
    time_limit: number,
    points: number,
    owner_id: string,
    created_at: number
}

export interface Contest {
    id: string,
    title: string,
    description: string,
    start_date: number,
    end_date: number,
    status: ContestStatus,
    owner_id: string,
    created_at: number
}