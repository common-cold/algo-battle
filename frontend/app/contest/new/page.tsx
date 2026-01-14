import ContestDetail from "@/components/create-contest/ContestDetail";
import QuestionSection from "@/components/create-contest/QuestionSection";
import { SaveContestButton } from "@/components/create-contest/SaveContestButton";

export default function CreateContestPage() {
    return <div className="flex-1 h-screen py-5 px-10 overflow-hidden">
        <div className="flex flex-col h-full gap-10">
            <div className="flex justify-between">
                <div className="text-left font-bold textColor text-[30px]">
                    Create New Contest
                </div>
                <SaveContestButton/>
            </div>
            
            <ContestDetail/>
            <QuestionSection/>
        </div>
    </div>
}

