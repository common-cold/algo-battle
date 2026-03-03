'use client'

import { showErrorToast } from "@/components/ContestInfo";
import { Leaderboard, LeaderboardRow } from "@/types/db";
import { getLeaderboard } from "@/utils/api";
import { use, useEffect, useState } from "react"

type LeaderboardRowComponentProps = {
    name: string,
    rank: number,
    score: number,
    id: number
}

export default function LeaderboardPage({params}: {
    params: Promise<{
        id: string
    }>
}) {

    const {id} = use(params);

    const [leaderboard, setLeaderboard] = useState<Leaderboard | null>(null);

    useEffect(() => {
        async function fetchLeaderboard() {
            const response = await getLeaderboard(id);
            if (!response) {
                showErrorToast("Unable to fetch Leaderboard");
            } else if (response.status != 200) {
                const data = response.data as any;
                showErrorToast(data.error);
            } else {
                let leaderboard = response.data as Leaderboard;
                setLeaderboard(leaderboard);
            }
        }
        fetchLeaderboard()
    }, []);

    return <div className="flex-1 h-screen min-h-0 py-5 px-10">
        <div className="flex flex-col gap-5">
            <div className="text-5xl text-center font-medium text-[#F59E0B]">
                Leaderboard
            </div>
            <div className="flex justify-between text-2xl">
                <div className="w-50 max-w-50 text-center">
                    Name
                </div>
                <div className="w-50 max-w-50 text-center">
                    Rank
                </div>
                <div className="w-50 max-w-50 text-center">
                    Score
                </div>
            </div>
            <div className="flex flex-col gap-1 text-[20px]">
                {
                    leaderboard?.map((l, index) => {
                        return <LeaderboardRowComponent name={l.name} rank={l.rank} score={l.score/100} id={index}/>
                    })
                }
            </div>
            
        </div>
       
    </div>
}

function LeaderboardRowComponent({name, rank, score, id}: LeaderboardRowComponentProps) {
    return <div key={id} className="flex justify-between contestRow hover:!bg-[#1F2937] px-2 py-2 text-[#E5E7EB]">
        <div className="w-50 max-w-50 text-center">
            {name}
        </div>
        <div className="w-50 max-w-50 text-center">
            {rank}
        </div>
        <div className="w-50 max-w-50 text-center">
            {score}
        </div>
    </div>
}