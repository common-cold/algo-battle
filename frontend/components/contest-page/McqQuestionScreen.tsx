import { contestEndDateAtom, contestJoinedAtAtom, contestSecondsAtom, currentContestIdAtom, currentRankAtom } from "@/store/atom";
import { Question } from "@/types/db"
import { convertEpochToIsoFormat } from "@/utils/common";
import { useAtom, useAtomValue } from "jotai";
import { useState } from "react";
import { showErrorToast, showSuccessToast } from "../ContestInfo";
import { submitMcqQuestion } from "@/utils/api";

type QuestionScreenProps = {
    question: Question | null
}

type OptionProps = {
    index: number,
    onClick: () => void
}

export default function McqQuestionScreen({question}: QuestionScreenProps) {
    const currentContestId = useAtomValue(currentContestIdAtom);
    const [selectedOption, setSelectedOption] = useState<number | null>(null);

    async function handleMcqSubmit() {
        if (selectedOption == null) {
            showErrorToast("Select an option");
            return;
        }

        if (!question || !currentContestId) {
            return;
        }

        const response = await submitMcqQuestion({
            contest_id: currentContestId,
            question_id: question.id,
            selected_option: selectedOption
        });

        if (!response) {
            showErrorToast("Error in submitting answer");
        } else if (response.status != 200) {
            const data = response.data as any;
            showErrorToast(data.error);
        } else {
            showSuccessToast("Question submitted successfully");
        }
    }

    if (!question) {
        return <div></div>
    }

    function OptionComponent({index, onClick}: OptionProps) {
        return <div onClick={() => {
            console.log("Index: " + index);
            onClick()
        }}
            className={`inputBox px-3 py-3 w-100 ${selectedOption == index ? "bg-[#93C5FD]! text-[#0f1117]! font-bold" : ""} cursor-pointer`}>
            {question!.options![index]}
        </div>
    }

    return <div className="flex flex-1 min-h-0 flex-col gap-10 items-center w-full">
        <div className="flex flex-col gap-5 min-h-0 w-full max-h-1/2 border-2 border-[#1F2937] rounded-[10px] text-left px-3 py-3 font-medium overflow-y-auto whitespace-pre-line">
            <div className="font-bold text-2xl">
                {question.title}
            </div>
            <div>
                {question.description}
            </div>
        </div>

        <div className="flex flex-col items-center gap-2">
            <div className="flex justify-between gap-5">   
                <OptionComponent index={0} onClick={() => setSelectedOption(0)}/>
                <OptionComponent index={1} onClick={() => setSelectedOption(1)}/>
            </div>

            <div className="flex justify-between gap-5">   
                <OptionComponent index={2} onClick={() => setSelectedOption(2)}/>
                <OptionComponent index={3} onClick={() => setSelectedOption(3)}/>  
            </div>
        </div>   

        <div onClick={() => {handleMcqSubmit()}}
            className="textBgStyle5 px-3 py-2 rounded-[10px]">
            Submit
        </div>
    </div>
}