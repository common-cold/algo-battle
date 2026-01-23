"use client"

import { contestDetailsAtom, selectedQuestionsAtom } from "@/store/atom"
import { useAtom, useAtomValue } from "jotai"
import { showErrorToast, showSuccessToast } from "../ContestInfo";
import { CreateContestArgs } from "@/types/routes";
import { isStringBlank } from "@/utils/common";
import { createContest } from "@/utils/api";
import { useRouter } from "next/navigation";

export function SaveContestButton() {
    const [selectedQuestions, setSelectedQuestions] = useAtom(selectedQuestionsAtom);
    const [contestDetail, setContestDetail] = useAtom(contestDetailsAtom);
    const router = useRouter();

    async function onClick() {        
        if (!contestDetail || isStringBlank(contestDetail.description) || isStringBlank(contestDetail.title) 
            || !contestDetail.end_date || !contestDetail.start_date
            || !selectedQuestions || selectedQuestions.length == 0) {
                showErrorToast("Please add all fields");
                return;
        }

        if (contestDetail.end_date <= contestDetail.start_date) {
            showErrorToast("End Date cannot be before Start Date");
            return;
        }

        let questionIds = selectedQuestions.map(q => q.id);
        
        let body: CreateContestArgs = {
            title: contestDetail?.title,
            description: contestDetail?.description,
            start_date: contestDetail.start_date,
            end_date: contestDetail.end_date,
            question_ids: questionIds
        }

        const response = await createContest(body);
        if (!response) {
            showErrorToast("Error in saving contest");
        } else if (response.status != 200) {
            const data = response.data as any;
            showErrorToast(data.error);
        } else {
            showSuccessToast("Contest saved successfully");
            setSelectedQuestions([]);
            setContestDetail(null);
            router.push("/contest/all");
        }
    }
    
    return <div onClick={() => onClick()}
        className="flex button4 items-center justify-center text-center px-8 py-2 font-bold">
            Save Contest
    </div>
}