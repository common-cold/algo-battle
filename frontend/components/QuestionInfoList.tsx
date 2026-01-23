import { Contest, Question } from "@/types/db"
import { ContestInfo } from "./ContestInfo"
import { QuestionInfo } from "./QuestionInfo"

export type QuestionInfoListProps = {
    questionList: Question[],
    showAttemptButton: boolean
}

export default function QuestionInfoList({questionList, showAttemptButton}: QuestionInfoListProps) {
    return <div className="flex flex-col w-full h-full min-h-0 gap-5">
        <div className="flex-1 min-h-0 flex flex-col gap-5 overflow-y-auto">
            {
                questionList.length == 0
                ?
                <div> 
                    No Contests saved Yet!
                </div>
                :
                questionList.map((q, index) => {
                    return <div key={index}>
                        <QuestionInfo 
                            id={q.id}
                            title={q.title}
                            timeLimit={q.time_limit}
                            points={q.points}
                            showAttemptButton={showAttemptButton}
                        />
                    </div>
                })
            }
        </div>
    </div>
}