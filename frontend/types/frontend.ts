export type ContestInfoData = {
    contestId: string,
    contestName: string,
    startTime?: string,
    isUpcomingContest: boolean
}

export type ContestDetailType = {
    title: string,
    description: string,
    startDate: string,
    endDate: string
}

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