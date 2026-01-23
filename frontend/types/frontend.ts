import { Role } from "./db"

export type QuestionType = "Mcq" | "Dsa" | "LiveAssignment"

export interface QuestionData {
    type: QuestionType,
    timeLimit: string,
    points: string,
    title?: string,
    description?: string,
    options?: string[]
    correctIndex?: number
}

export type JwtPayload = {
    id: string,
    role: Role,
    username: string,
    exp: number
}