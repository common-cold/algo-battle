"use client"

import { useEffect, useState } from "react"
import ImportQuestionModal from "./ImportQuestionModal";
import CreateQuestionModal from "./CreateQuestionModal";
import { useAtom, useAtomValue } from "jotai";
import { selectedQuestionsAtom } from "@/store/atom";
import { getQuestionsById } from "@/utils/api";
import { showErrorToast } from "../ContestInfo";
import { Question } from "@/types/db";
import QuestionInfoList from "../QuestionInfoList";



export default function QuestionSection() {
    const selectedQuestions = useAtomValue(selectedQuestionsAtom);
    const [showImportQuestionModal, setshowImportQuestionModal] = useState(false);
    const [showCreateQuestionModal, setshowCreateQuestionModal] = useState(false);


    if (showImportQuestionModal) {
        return <div>
            <ImportQuestionModal onClose={() => setshowImportQuestionModal(false)}/>
        </div>
    }

    if (showCreateQuestionModal) {
        return <div>
            <CreateQuestionModal onClose={() => setshowCreateQuestionModal(false)}/>
        </div>
    }

    return <div className="flex flex-col flex-1 min-h-0 gap-3">
        <div className="flex justify-between">
            <div className="text-left font-bold textColor text-[20px] ">
                Questions
            </div>
            <div className="flex justify-between text-[18px] gap-3">
                <div onClick={() => setshowImportQuestionModal((prev) => !prev)}
                    className="button2 px-3 font-medium">
                    Import Questions
                </div>
                <div onClick={() => setshowCreateQuestionModal((prev) => !prev)} 
                    className="button3 px-3 font-medium">
                    Create New Question
                </div>
            </div>
        </div>

        <div className="flex flex-1 h-full min-h-0">
            {
                selectedQuestions.length == 0
                ?
                <div className="flex flex-col flex-1 justify-center items-center">
                    No Questions selected yet!
                </div>
                :
                <div className="flex flex-1 min-h-0 overflow-y-auto">
                    <QuestionInfoList questionList={selectedQuestions} showAttemptButton={false}/>
                </div>    
            }
        </div>
    </div>
}

