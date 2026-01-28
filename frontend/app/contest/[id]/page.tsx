"use client"

import { ContestEndModal } from "@/components/contest-page/ContestEndModal";
import QuestionScreen from "@/components/contest-page/QuestionScreen";
import QuestionSidebar from "@/components/contest-page/QuestionSidebar";
import { showErrorToast } from "@/components/ContestInfo";
import { connectWsAtom, contestEndDateAtom, contestJoinedAtAtom, contestSecondsAtom, currentContestIdAtom, currentQuestionIdAtom, isContestOverAtom, isWsOpenAtom, userAtom, wsAtom } from "@/store/atom";
import { Question } from "@/types/db";
import { FullContest } from "@/types/routes";
import { WebSocketMessage } from "@/types/ws";
import { getContestJoinedAt, getFullContest } from "@/utils/api";
import { useAtom, useAtomValue } from "jotai";
import { useRouter } from "next/navigation";
import { use, useEffect, useState } from "react"



export default function ContestPage({params} : {
    params: Promise<{
        id: string
    }>  
}) {
    const [contestJoinedAt, setContestJoinedAt] = useAtom(contestJoinedAtAtom);
    const [contestEndDate, setContestEndDate] = useAtom(contestEndDateAtom);
    const [contestSeconds, setContestSeconds] = useAtom(contestSecondsAtom);
    const [currentContestId, setCurrentContestId] = useAtom(currentContestIdAtom);
    const [ws, setWs] = useAtom(wsAtom);
    const isWsOpen = useAtomValue(isWsOpenAtom);
    const [_, connectWs] = useAtom(connectWsAtom);
    const user = useAtomValue(userAtom);
    const currentQuestionId = useAtomValue(currentQuestionIdAtom);
    const isContestOver = useAtomValue(isContestOverAtom);
    const [showContestOverModal, setShowContestOverModal] = useState(false);
    const [fullContest, setFullContest] = useState<FullContest | null>(null);
    const [currQuestion, setCurrQuestion] = useState<Question | null>(null);
    const [error, setError] = useState<string | null>(null);

    const {id} = use(params);

    const router = useRouter();

    useEffect(() => {
        async function fetchContestJoinedAt() {
            const response = await getContestJoinedAt(user!.id, id);

            if (!response) {
                showErrorToast("Error in fetching contest joining time");
                return "Error in fetching contest joining time";
            } else if (response.status != 200) {
                const data = response.data as any;
                showErrorToast("You have not joined this contest");
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

        async function init() {
            let error = await fetchContestJoinedAt();
            if (error) {
                setError("You have not joined this contest");
                return;
            }
            connectWs();
            await fetchFullContest(); 
        }
        
        let token = localStorage.getItem("token");
        if (!token) {
            router.replace("/");
        } 

        if (!user) {
            return;
        }

        init();

        return () => {
            setContestJoinedAt(null);
        }
    }, [user]);

    useEffect(() => {
        if (!ws) {
            console.log("WS is null");
            return;
        }

        if (ws.readyState !== WebSocket.OPEN) {
            console.log("WS is not open");
            return;
        }

        let msg: WebSocketMessage = {
            JoinContest: {
                contest_id: id
            }
        }; 

        ws.send(JSON.stringify(msg));
    }, [ws, isWsOpen]);

    useEffect(() => {
        if (currentQuestionId != null && fullContest != null) {
            let currQuestion = fullContest.questions.filter(q => q.id === currentQuestionId)[0];
            setCurrQuestion(currQuestion);
        }
    }, [currentQuestionId])

    useEffect(() => {
        const interval = setInterval(() => setContestSeconds(prev => prev! + 1), 1000);
        setCurrentContestId(id);

        return () => {
            clearInterval(interval);
            setContestSeconds(null);
            setCurrentContestId(null);
            if (ws != null) {
                ws.close();
                setWs(null);
            }
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

    if (fullContest.contest.status === "Closed" || isContestOver) {
        return <div>
            <ContestEndModal closeModal={() => setShowContestOverModal(false)}/>
        </div>
    }

    return <div className="flex-1 h-screen min-h-0 py-5 px-10">
        <div className="flex flex-1 h-full min-h-0 justify-between gap-20">
            <QuestionSidebar questions={fullContest.questions as Question[]} />
            <QuestionScreen question={currQuestion}/>
        </div>
    </div>
}