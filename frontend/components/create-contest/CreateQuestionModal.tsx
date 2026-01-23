import { QuestionData, QuestionType } from "@/types/frontend";
import { useEffect, useRef, useState } from "react";
import { showErrorToast } from "../QuestionInfo";
import { createQuestion } from "@/utils/api";
import { useAtomValue } from "jotai";
import { showSuccessToast } from "../ContestInfo";
import { isStringBlank } from "@/utils/common";

type NewQuestionModalProps = {
    onClose: () => void
}

export default function CreateQuestionModal({onClose}: NewQuestionModalProps) {
    const modalRef = useRef<HTMLDivElement>(null);
    const [questionData, setQuestionData] = useState<QuestionData>({
        type: "Mcq",
        timeLimit: "10",
        points: "5"
    });
    const [options, setOptions] = useState<string[]>([]);
    const [correctOption, setCorrectOption] = useState<number | null>(null);


    function setOptionInObj(index: number, newVal: string) {
        const newOptions = new Array(...options);
        newOptions[index] = newVal;
        setOptions(newOptions);
        setQuestionData(prev => ({
            ...prev,
            options: newOptions
        } as QuestionData))
    }

    async function saveQuestion() {
        if (!questionData || !questionData.correctIndex || isStringBlank(questionData.description) || isStringBlank(questionData.points)
            || isStringBlank(questionData.timeLimit) || isStringBlank(questionData.title) || !questionData.type
            || !questionData.options || questionData.options.length != 4 
            || !questionData.options.every(o => o != null && o != undefined && o!= "" && o.trim().length > 0)
        ) {
            console.log(JSON.stringify(questionData));
            showErrorToast("Please fill all fields");
            return;
        }

        let timeLimitInMins = Number(questionData.timeLimit);
        let timeLimitInSeconds = timeLimitInMins * 60;

        let points = Number(questionData.points);

        const response = await createQuestion({
            question_type: questionData.type,
            title: questionData.title!,
            description: questionData.description!,
            options: questionData.options,
            correct_option: questionData.correctIndex,
            time_limit: timeLimitInSeconds,
            points: points
        });

        if (!response) {
            showErrorToast("Error in saving question");
        } else if (response.status != 200) {
            const data = response.data as any;
            showErrorToast(data.error);
        } else {
            showSuccessToast("Question saved successfully");
        }
    }
    
    useEffect(() => {
        function handleClick(e: MouseEvent) {
            if (modalRef.current && !modalRef.current.contains(e.target as Node)) {
                onClose();
            }
        }

        function handleEscape(e: KeyboardEvent) {
            if (e.key == "Escape") {
                onClose();
            }
        }

        window.addEventListener("mousedown", handleClick);
        window.addEventListener("keydown", handleEscape);

        return () => {
            document.removeEventListener("mousedown", handleClick);
            document.removeEventListener("keydown", handleEscape);
        }
    }, [])

    function TickComponent({index, onClick, correctOption}: {index: number, onClick: () => void, correctOption: number | null}) {
        return <div className={`h-7 w-7 border ${index == correctOption && 'bg-[#22C55E]' } border-[#9CA3AF] rounded-[100%]`}
            onClick={() => {
                setQuestionData(prev => ({
                    ...prev,
                    correctIndex: index
                } as QuestionData))
                onClick()
            }}>
        </div>
    }


    return <div className="fixed inset-0 z-50 flex items-center justify-center">
        <div className="absolute inset-0 bg-black/50">
            <div ref={modalRef}
                className="relative z-10 mx-20 my-20 px-5 py-5 flex flex-col gap-5 bg-[#0f1117] h-160 max-h-160 overflow-y-auto">
                    <div className="flex justify-between">
                        <div className="text-left font-bold textColor text-[30px]">
                            Create New Question
                        </div>
                        <div onClick={() => saveQuestion()}
                        className="button2 px-3 font-bold flex items-center">
                            Create Question
                        </div>
                    </div>

                    <div className="flex justify-between">
                        <div className="flex justify-between items-baseline gap-3">
                            <div className="text-left font-bold textColor text-[20px]">
                                Question Type
                            </div>
                            <select 
                                className="bg-[#161B26] px-2 py-2 items-center justify-center rounded-[7px]"        
                                onChange={(e) => setQuestionData(prev => ({
                                ...prev,
                                type: e.target.value as QuestionType
                            } as QuestionData))}
                                name="selectedType" defaultValue="Mcq">
                                <option value="Dsa">DSA</option>
                                <option value="LiveAssignment">Live Assignment</option>
                                <option value="Mcq">MCQ</option>
                            </select>
                        </div>

                        <div className="flex max-w-75 justify-between items-baseline gap-3">
                            <div className="text-left font-bold textColor text-[20px]">
                                Time Limit
                            </div>
                            <select 
                                className="bg-[#161B26] px-2 py-2 items-center justify-center rounded-[7px]"        
                                onChange={(e) => setQuestionData(prev => ({
                                ...prev,
                                timeLimit: e.target.value as QuestionType
                            } as QuestionData))}
                                name="selectedType" defaultValue="10">
                                <option value="10">10 mins</option>
                                <option value="20">20 mins</option>
                                <option value="30">30 mins</option>
                                <option value="40">40 mins</option>
                                <option value="50">50 mins</option>
                                <option value="60">60 mins</option>
                                <option value="70">70 mins</option>
                                <option value="80">80 mins</option>
                                <option value="90">90 mins</option>
                            </select>
                        </div>

                        <div className="flex max-w-75 justify-between items-baseline gap-3">
                            <div className="text-left font-bold textColor text-[20px]">
                                Points
                            </div>
                            <select 
                                className="bg-[#161B26] px-2 py-2 items-center justify-center rounded-[7px]"        
                                onChange={(e) => setQuestionData(prev => ({
                                ...prev,
                                points: e.target.value as QuestionType
                            } as QuestionData))}
                                name="selectedType" defaultValue="5">
                                <option value="5">5</option>
                                <option value="10">10</option>
                                <option value="15">10</option>
                                <option value="20">20</option>
                                <option value="25">25</option>
                                <option value="30">30</option>
                                <option value="35">35</option>
                                <option value="40">40</option>
                                <option value="45">45</option>
                                <option value="50">50</option>
                            </select>
                        </div>
                    </div>

                    <div className="flex flex-col gap-2">
                        <div className="text-left font-bold textColor text-[20px]">
                            Title
                        </div>
                        <input onChange={(e) => setQuestionData(prev => ({
                            ...prev,
                            title: e.target.value
                        } as QuestionData))}
                        className="inputBox px-3 py-3"/>
                    </div>

                    <div className="flex flex-col gap-2">
                        <div className="text-left font-bold textColor text-[20px]">
                            Description
                        </div>
                        <textarea onChange={(e) => setQuestionData(prev => ({
                            ...prev,
                            description: e.target.value
                        } as QuestionData))}
                            className="w-full h-40 p-3 rounded-md bg-[#161B26] text-[#E5E7EB] resize-y"
                        />
                    </div>

                    <div className="flex flex-col gap-2">
                        <div className="text-left font-bold textColor text-[20px]">
                            Options
                        </div>
                        <div className="flex flex-col gap-2">
                            <div className="flex justify-between">
                                <div className="flex justify-between items-center gap-3">
                                    <input onChange={(e) => {setOptionInObj(0, e.target.value)}}
                                        className="inputBox px-3 py-3 w-100"
                                        placeholder="Option A"
                                    />
                                    <TickComponent index={0} onClick={() => setCorrectOption(0)} correctOption={correctOption}/>
                                </div>
                                <div className="flex justify-between items-center gap-3">
                                    <TickComponent index={1} onClick={() => setCorrectOption(1)} correctOption={correctOption}/>
                                    <input onChange={(e) => {setOptionInObj(1, e.target.value)}}
                                        className="inputBox px-3 py-3 w-100"
                                        placeholder="Option B"
                                    />
                                </div>
                            </div>
                            <div className="flex justify-between">
                                <div className="flex justify-between items-center gap-3">
                                    <input onChange={(e) => {setOptionInObj(2, e.target.value)}}
                                        className="inputBox px-3 py-3 w-100"
                                        placeholder="Option C"
                                    />
                                    <TickComponent index={2} onClick={() => setCorrectOption(2)} correctOption={correctOption}/>
                                </div>
                                <div className="flex justify-between items-center gap-3">
                                    <TickComponent index={3} onClick={() => setCorrectOption(3)} correctOption={correctOption}/>
                                    <input onChange={(e) => {setOptionInObj(3, e.target.value)}}
                                        className="inputBox px-3 py-3 w-100"
                                        placeholder="Option D"
                                    />
                                </div>
                            </div>
                        </div>
                    </div>
            </div>
        </div>
    </div>
}