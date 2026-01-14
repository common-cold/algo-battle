import { ContestInfoData } from "@/types/frontend"
import { Contest } from "@/types/db"
import { ContestInfo } from "./ContestInfo"

export type ContestInfoListProps = {
    title: string,
    contestList: Contest[]
}

export default function ContestInfoList({title, contestList}: ContestInfoListProps) {
    return <div className="flex flex-col h-full min-h-0 gap-5">
        <div className="text-left font-bold textColor text-[30px]">
            {title}
        </div>
        <div className="flex-1 min-h-0 flex flex-col gap-5 overflow-y-auto">
            {
                contestList.length == 0
                ?
                <div> 
                    No Contests saved Yet!
                </div>
                :
                contestList.map((c, index) => {
                    return <div key={index}>
                        <ContestInfo 
                            contestId={c.id} 
                            contestName={c.title} 
                            startTime={c.start_date} 
                            status={c.status}
                        />
                    </div>
                })
            }
        </div>
    </div>
}