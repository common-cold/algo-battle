import { ContestStatus } from "@/types/db";
import { convertMillisToMinutes } from "@/utils/common";
import toast from "react-hot-toast";

export type QuestionInfoProps = {
    id: string,
    title: string,
    timeLimit: number,
    points: number,
    showAttemptButton: boolean
}

export function QuestionInfo({id, title, timeLimit, points, showAttemptButton} : QuestionInfoProps) {
    return <div className="flex justify-between contestRow text-[18px] px-5 py-3 hover:!bg-[#1F2433]">
        <div className="textColor">
            {title}
        </div>
        <div className="flex justify-between gap-15">
           <div className="textColor">
                {`${convertMillisToMinutes(timeLimit)} Mins`}
            </div>
            <div>
                {`${points} Points`}
            </div>   
            {
                showAttemptButton
                &&
                <div className="button2 px-3 font-medium">
                    Attempt
                </div>
            } 
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