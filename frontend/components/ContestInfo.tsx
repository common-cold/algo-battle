import { ContestStatus } from "@/types/db";
import toast from "react-hot-toast";

export type ContestInfoProps = {
    contestId: string,
    contestName: string,
    startTime?: number,
    status: ContestStatus
}

export function ContestInfo({contestId, contestName, startTime, status} : ContestInfoProps) {
    return <div className="flex justify-between contestRow text-[18px] px-5 py-3 hover:!bg-[#1F2433]">
        <div className="textColor">
            {contestName}
        </div>
        <div className="flex justify-between gap-15">
            {
                status == "Scheduled" &&
                <div className="textColor">
                    {`Starts at ${startTime}`}
                </div>    
            }
            <div>
                {
                    status == "Scheduled"
                    ?
                    <div className="button1 px-3 font-medium">
                        Edit 
                    </div>
                    :
                    <div className="button2 px-3 font-medium">
                        View Leaderboard
                    </div>
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