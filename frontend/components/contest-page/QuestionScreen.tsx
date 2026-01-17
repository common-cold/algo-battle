import { contestEndDateAtom, contestJoinedAtAtom, contestSecondsAtom } from "@/store/atom";
import { Question } from "@/types/db"
import { convertEpochToIsoFormat } from "@/utils/common";
import { useAtom } from "jotai";
import { useState } from "react";

type QuestionScreenProps = {
    question: Question | null
}

type OptionProps = {
    index: number,
    onClick: () => void
}

export default function QuestionScreen({question}: QuestionScreenProps) {
    const [contestJoinedAt, ] = useAtom(contestJoinedAtAtom);
    const [contestEndDate, ] = useAtom(contestEndDateAtom);
    const [questionTime, ] = useState(question!.time_limit);
    const [contestSeconds, setContestSeconds] = useAtom(contestSecondsAtom);
    const [selectedOption, setSelectedOption] = useState<number | null>(null);

    if (!question) {
        return <div></div>
    }

    function OptionComponent({index, onClick}: OptionProps) {
        return <div onClick={() => {
            console.log("Index: " + index);
            onClick()
        }}
            className={`inputBox px-3 py-3 w-100 ${selectedOption == index ? "bg-[#93C5FD]! text-[#0f1117]! font-bold" : ""} cursor-pointer`}>
            {question!.options[index]}
        </div>
    }    


    return <div className="flex flex-1 min-h-0 flex-col gap-10 items-center">
        <div className="flex justify-between gap-80">
            <div className="flex justify-centre gap-5">
                <div className="textBgStyle2 px-3 py-2 rounded-[10px]">
                    {`Time Left: ${convertEpochToIsoFormat(contestEndDate! - contestSeconds!)}`}
                </div>
                <div className="textBgStyle4 px-3 py-2 rounded-[10px]">
                    Rank: 105th
                </div>
            </div>
            
        </div>

        <div className="flex min-h-0 w-full max-h-1/2 border-2 border-[#1F2937] rounded-[10px] text-left px-3 py-3 font-medium overflow-y-auto">
            {question.description}
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

        <div className="textBgStyle5 px-3 py-2 rounded-[10px]">
            Submit
        </div>
    </div>
}