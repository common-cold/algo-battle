"use client"

import QuestionScreen from "@/components/contest-page/QuestionScreen";
import QuestionSidebar from "@/components/contest-page/QuestionSidebar";
import { showErrorToast } from "@/components/ContestInfo";
import { candidateIdAtom, contestEndDateAtom, contestJoinedAtAtom, contestSecondsAtom } from "@/store/atom";
import { Question } from "@/types/db";
import { FullContest } from "@/types/routes";
import { getContestJoinedAt, getFullContest } from "@/utils/api";
import { useAtom, useAtomValue } from "jotai";
import { use, useEffect, useState } from "react"



export default function ContestPage({params} : {
    params: Promise<{
        id: string
    }>  
}) {
    const [contestJoinedAt, setContestJoinedAt] = useAtom(contestJoinedAtAtom);
    const [contestEndDate, setContestEndDate] = useAtom(contestEndDateAtom);
    const [contestSeconds, setContestSeconds] = useAtom(contestSecondsAtom);
    const candidateId = useAtomValue(candidateIdAtom);
    const [fullContest, setFullContest] = useState<FullContest | null>(null);
    const [currQuestion, setCurrQuestion] = useState<Question | null>(null);
    const [error, setError] = useState<string | null>(null);

    const {id} = use(params);

    function onQuestionClick(question: Question) {
        setCurrQuestion(question);
    }

    useEffect(() => {
        async function fetchContestJoinedAt() {
            const response = await getContestJoinedAt(candidateId, id);

            if (!response) {
                showErrorToast("Error in fetching contest joining time");
                return "Error in fetching contest joining time";
            } else if (response.status != 200) {
                const data = response.data as any;
                showErrorToast(data.error);
                return data.error;
            } else {
                let data = response.data as any;
                setContestJoinedAt(data.joined_at);
                setContestSeconds(Math.trunc(Date.now()/1000));
                return null;
            }
        }
        
        async function fetchFullContest() {
            const response = await getFullContest(id);

            if (!response) {
                showErrorToast("Error in fetching contestt");
            } else if (response.status != 200) {
                const data = response.data as any;
                showErrorToast(data.error);
            } else {
                let contestData = response.data as FullContest
                setFullContest(contestData);
                setContestEndDate(contestData.contest.end_date);
                setCurrQuestion(contestData.questions[0]);
            }
        }


        async function run() {
            let error = await fetchContestJoinedAt();
            if (error) {
                setError(error);
                return;
            }
            await fetchFullContest();
        }
        
        run()

        return () => {
            setContestJoinedAt(null);
        }
    }, []);

    useEffect(() => {
        const interval = setInterval(() => setContestSeconds(prev => prev! + 1), 1000);

        return () => {
            clearInterval(interval);
            setContestSeconds(null);
        }
    }, []);

    if (error) {
        return <div className="flex-1 h-screen min-h-0 py-5 px-10 flex items-center justify-center">
            No contests to display
        </div>
    }

    if (!fullContest) {
        return <div className="flex-1 h-screen min-h-0 py-5 px-10 flex items-center justify-center">
            Contest Does not exists!
        </div>
    }

    return <div className="flex-1 h-screen min-h-0 py-5 px-10">
        <div className="flex flex-1 h-full min-h-0 justify-between gap-20">
            <QuestionSidebar questions={fullContest.questions as Question[]} onClick={onQuestionClick} />
            <QuestionScreen question={currQuestion}/>
        </div>
    </div>
}