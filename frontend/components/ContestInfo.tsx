import { userAtom } from "@/store/atom";
import { ContestStatus } from "@/types/db";
import { joinContest } from "@/utils/api";
import { convertSecondsToHumanReadable } from "@/utils/common";
import { useAtomValue } from "jotai";
import { useRouter } from "next/navigation";
import toast from "react-hot-toast";

export type ContestInfoProps = {
    contestId: string,
    contestName: string,
    startTime?: number,
    status: ContestStatus
}

export function ContestInfo({contestId, contestName, startTime, status} : ContestInfoProps) {
    const user = useAtomValue(userAtom);

    const router = useRouter();

    async function onJoin() {
        const response = await joinContest(contestId);
        
        if (!response) {
            showErrorToast("Error in joining contest");
        } else if (response.status != 200) {
            const data = response.data as any;
            showErrorToast(data.error);
        } else {
            router.push(`/contest/${contestId}`)
        }
    }

    return <div className="flex justify-between contestRow text-[18px] px-5 py-3 hover:!bg-[#1F2433]">
        <div className="textColor">
            {contestName}
        </div>
        <div className="flex justify-between gap-15">
            {
                status == "Scheduled" &&
                <div className="textColor">
                    {`Starts at ${convertSecondsToHumanReadable(startTime! - (Date.now()/1000))}`}
                </div>    
            }
            <div>
                {
                    status == "Scheduled"
                    ?
                    user!.role == "Examiner"
                    &&
                    <div className="button1 px-3 font-medium">
                        Edit 
                    </div> 
                    :
                    (
                        status == "Closed"
                        ?
                        <div className="button2 px-3 font-medium">
                            View Leaderboard
                        </div>
                        :
                        (
                            user!.role == "Examiner"
                            ?
                            <div className="button2 px-3 font-medium">
                                In Progress
                            </div>
                            :
                            <div onClick={() => onJoin()}
                                className="button2 px-3 font-medium">
                                Join
                            </div>
                        )
                    )                    
                }
            </div>
        </div>
    </div>
}

export function showSuccessToast(message: string) {
    toast.success(
        <div>
            {message}
        </div>,
        { 
            duration: 5000, 
            style: {
            borderRadius: "5px",
            background: "white",
            color: "black",
            fontWeight: "bold"
        }}
    );  
}

export function showErrorToast(message: string) {
    toast.error(
        <div>
            {message}
        </div>,
        { 
            duration: 5000, 
            style: {
            borderRadius: "5px",
            background: "white",
            color: "black",
            fontWeight: "bold"
        }}
    );  
}