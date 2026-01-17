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

const questions = [
    {
            "id": "d9ad452f-c4a3-4631-bf74-6f7845a3f0aa",
            "question_type": "Mcq",
            "title": "Which sorting algorithm is fastest",
            "description": "Chhose one option",
            "options": [
                "Bubble Sort",
                "Quick Sort",
                "Merge Sort",
                "Insertion Sort"
            ],
            "correct_option": 1,
            "time_limit": 1200000,
            "points": 20,
            "owner_id": "62e0e171-a0e0-4ab9-ad07-d23c3e15a75f",
            "created_at": 1768411159
        },
        {
            "id": "fcfd1122-3ed3-4673-adea-73f0ec64b214",
            "question_type": "Mcq",
            "title": "Who is the best coder",
            "description": "Chhose one option",
            "options": [
                "Me",
                "You",
                "Him",
                "Her"
            ],
            "correct_option": 1,
            "time_limit": 1800000,
            "points": 50,
            "owner_id": "62e0e171-a0e0-4ab9-ad07-d23c3e15a75f",
            "created_at": 1768411215
        },
        {
            "id": "bb84333d-cab8-4b77-97d1-79c6a5bea3e0",
            "question_type": "Mcq",
            "title": "Time complexity of binary search",
            "description": "Choose one option",
            "options": [
                "O(n^2)",
                "O(n log n)",
                "O(1)",
                "O(log n)"
            ],
            "correct_option": 3,
            "time_limit": 600000,
            "points": 5,
            "owner_id": "62e0e171-a0e0-4ab9-ad07-d23c3e15a75f",
            "created_at": 1768421157
        },
        {
            "id": "303fcbff-c36b-456a-9824-0e0b56d0b075",
            "question_type": "Mcq",
            "title": "Which sorting algorithm is fastest",
            "description": "Chhose one option",
            "options": [
                "Bubble Sort",
                "Quick Sort",
                "Merge Sort",
                "Insertion Sort"
            ],
            "correct_option": 1,
            "time_limit": 3600000,
            "points": 40,
            "owner_id": "62e0e171-a0e0-4ab9-ad07-d23c3e15a75f",
            "created_at": 1768421783
        },
        {
            "id": "d9ad452f-c4a3-4631-bf74-6f7845a3f0aa",
            "question_type": "Mcq",
            "title": "Which sorting algorithm is fastest",
            "description": "Chhose one option",
            "options": [
                "Bubble Sort",
                "Quick Sort",
                "Merge Sort",
                "Insertion Sort"
            ],
            "correct_option": 1,
            "time_limit": 1200000,
            "points": 20,
            "owner_id": "62e0e171-a0e0-4ab9-ad07-d23c3e15a75f",
            "created_at": 1768411159
        },
        {
            "id": "fcfd1122-3ed3-4673-adea-73f0ec64b214",
            "question_type": "Mcq",
            "title": "Who is the best coderhfjdhjfk dhjhjkss dijsdio",
            "description": "Chhose one option",
            "options": [
                "Me",
                "You",
                "Him",
                "Her"
            ],
            "correct_option": 1,
            "time_limit": 1800000,
            "points": 50,
            "owner_id": "62e0e171-a0e0-4ab9-ad07-d23c3e15a75f",
            "created_at": 1768411215
        },
        {
            "id": "bb84333d-cab8-4b77-97d1-79c6a5bea3e0",
            "question_type": "Mcq",
            "title": "Time complexity of binary search",
            "description": "Choose one option",
            "options": [
                "O(n^2)",
                "O(n log n)",
                "O(1)",
                "O(log n)"
            ],
            "correct_option": 3,
            "time_limit": 600000,
            "points": 5,
            "owner_id": "62e0e171-a0e0-4ab9-ad07-d23c3e15a75f",
            "created_at": 1768421157
        },
        {
            "id": "303fcbff-c36b-456a-9824-0e0b56d0b075",
            "question_type": "Mcq",
            "title": "Which sorting algorithm is fastest",
            "description": "Chhose one option",
            "options": [
                "Bubble Sort",
                "Quick Sort",
                "Merge Sort",
                "Insertion Sort"
            ],
            "correct_option": 1,
            "time_limit": 3600000,
            "points": 40,
            "owner_id": "62e0e171-a0e0-4ab9-ad07-d23c3e15a75f",
            "created_at": 1768421783
        },{
            "id": "d9ad452f-c4a3-4631-bf74-6f7845a3f0aa",
            "question_type": "Mcq",
            "title": "Which sorting algorithm is fastest",
            "description": "Chhose one option",
            "options": [
                "Bubble Sort",
                "Quick Sort",
                "Merge Sort",
                "Insertion Sort"
            ],
            "correct_option": 1,
            "time_limit": 1200000,
            "points": 20,
            "owner_id": "62e0e171-a0e0-4ab9-ad07-d23c3e15a75f",
            "created_at": 1768411159
        },
        {
            "id": "fcfd1122-3ed3-4673-adea-73f0ec64b214",
            "question_type": "Mcq",
            "title": "Who is the best coder",
            "description": "Chhose one option",
            "options": [
                "Me",
                "You",
                "Him",
                "Her"
            ],
            "correct_option": 1,
            "time_limit": 1800000,
            "points": 50,
            "owner_id": "62e0e171-a0e0-4ab9-ad07-d23c3e15a75f",
            "created_at": 1768411215
        },
        {
            "id": "bb84333d-cab8-4b77-97d1-79c6a5bea3e0",
            "question_type": "Mcq",
            "title": "Time complexity of binary search",
            "description": "Choose one option",
            "options": [
                "O(n^2)",
                "O(n log n)",
                "O(1)",
                "O(log n)"
            ],
            "correct_option": 3,
            "time_limit": 600000,
            "points": 5,
            "owner_id": "62e0e171-a0e0-4ab9-ad07-d23c3e15a75f",
            "created_at": 1768421157
        },
        {
            "id": "303fcbff-c36b-456a-9824-0e0b56d0b075",
            "question_type": "Mcq",
            "title": "Which sorting algorithm is fastest",
            "description": "Chhose one option",
            "options": [
                "Bubble Sort",
                "Quick Sort",
                "Merge Sort",
                "Insertion Sort"
            ],
            "correct_option": 1,
            "time_limit": 3600000,
            "points": 40,
            "owner_id": "62e0e171-a0e0-4ab9-ad07-d23c3e15a75f",
            "created_at": 1768421783
        },
        {
            "id": "d9ad452f-c4a3-4631-bf74-6f7845a3f0aa",
            "question_type": "Mcq",
            "title": "Which sorting algorithm is fastest",
            "description": "Chhose one option",
            "options": [
                "Bubble Sort",
                "Quick Sort",
                "Merge Sort",
                "Insertion Sort"
            ],
            "correct_option": 1,
            "time_limit": 1200000,
            "points": 20,
            "owner_id": "62e0e171-a0e0-4ab9-ad07-d23c3e15a75f",
            "created_at": 1768411159
        },
        {
            "id": "fcfd1122-3ed3-4673-adea-73f0ec64b214",
            "question_type": "Mcq",
            "title": "Who is the best coder",
            "description": "Chhose one option",
            "options": [
                "Me",
                "You",
                "Him",
                "Her"
            ],
            "correct_option": 1,
            "time_limit": 1800000,
            "points": 50,
            "owner_id": "62e0e171-a0e0-4ab9-ad07-d23c3e15a75f",
            "created_at": 1768411215
        },
        {
            "id": "bb84333d-cab8-4b77-97d1-79c6a5bea3e0",
            "question_type": "Mcq",
            "title": "Time complexity of binary search",
            "description": "Choose one option",
            "options": [
                "O(n^2)",
                "O(n log n)",
                "O(1)",
                "O(log n)"
            ],
            "correct_option": 3,
            "time_limit": 600000,
            "points": 5,
            "owner_id": "62e0e171-a0e0-4ab9-ad07-d23c3e15a75f",
            "created_at": 1768421157
        },
        {
            "id": "303fcbff-c36b-456a-9824-0e0b56d0b075",
            "question_type": "Mcq",
            "title": "Which sorting algorithm is fastest",
            "description": "Chhose one option",
            "options": [
                "Bubble Sort",
                "Quick Sort",
                "Merge Sort",
                "Insertion Sort"
            ],
            "correct_option": 1,
            "time_limit": 3600000,
            "points": 40,
            "owner_id": "62e0e171-a0e0-4ab9-ad07-d23c3e15a75f",
            "created_at": 1768421783
        }
]


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
            <QuestionSidebar questions={questions as Question[]} onClick={onQuestionClick} />
            <QuestionScreen question={currQuestion}/>
        </div>
    </div>
}