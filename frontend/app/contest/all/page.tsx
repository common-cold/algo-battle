"use client"

import { showErrorToast } from "@/components/ContestInfo";
import ContestInfoList from "@/components/ContestInfoList";
import { userIdAtom } from "@/store/atom";
import { Contest, ContestStatus } from "@/types/db";
import { getAllExaminerContests } from "@/utils/api";
import { useAtomValue } from "jotai";
import { useEffect, useState } from "react";


let upcomingContests = [
    {
        contestId: "1",
        contestName: "DSA Challenge #1",
        startTime: "5:30 PM",
        isUpcomingContest: true
    },
    {
        contestId: "1",
        contestName: "DSA Challenge #1",
        startTime: "5:30 PM",
        isUpcomingContest: true
    },
    {
        contestId: "1",
        contestName: "DSA Challenge #1",
        startTime: "5:30 PM",
        isUpcomingContest: true
    },
    {
        contestId: "1",
        contestName: "DSA Challenge #1",
        startTime: "5:30 PM",
        isUpcomingContest: true
    },
    // {
    //     contestId: "1",
    //     contestName: "DSA Challenge #1",
    //     startTime: "5:30 PM",
    //     isUpcomingContest: true
    // },
    // {
    //     contestId: "1",
    //     contestName: "DSA Challenge #1",
    //     startTime: "5:30 PM",
    //     isUpcomingContest: true
    // },
    // {
    //     contestId: "1",
    //     contestName: "DSA Challenge #1",
    //     startTime: "5:30 PM",
    //     isUpcomingContest: true
    // },,{
    //     contestId: "1",
    //     contestName: "DSA Challenge #1",
    //     startTime: "5:30 PM",
    //     isUpcomingContest: true
    // },
    // {
    //     contestId: "1",
    //     contestName: "DSA Challenge #1",
    //     startTime: "5:30 PM",
    //     isUpcomingContest: true
    // },

];

let pastContests = [
    {
        contestId: "1",
        contestName: "DSA Challenge #1",
        isUpcomingContest: false
    },
    {
        contestId: "1",
        contestName: "DSA Challenge #1",
        isUpcomingContest: false
    },
    {
        contestId: "1",
        contestName: "DSA Challenge #1",
        isUpcomingContest: false
    },
    {
        contestId: "1",
        contestName: "DSA Challenge #1",
        isUpcomingContest: false
    }
];

export default function AllContest() {
    const userId = useAtomValue(userIdAtom);
    const [upcomingContests, setUpcomingContests] = useState<Contest[]>([]);
    const [pastContests, setPastContests] = useState<Contest[]>([]);


    useEffect(() => {
        async function fetchExaminerContestByStatus(status: ContestStatus) {
            const response = await getAllExaminerContests({
                id: userId,
                status: status
            });

            if (!response) {
                showErrorToast("Error in fetching contests");
            } else if (response.status != 200) {
                const data = response.data as any;
                showErrorToast(data.error);
            } else {
                const contests = response.data as Array<Contest>;
                if (contests.length == 0) {
                    showErrorToast("You have not saved any contests yet!");
                } else if (status == "Scheduled" || status == "Active") {
                    setUpcomingContests(prev => [...prev, ...contests]);
                } else if (status == "Closed") {
                    setPastContests(contests)
                }
            }
        }

        async function fetchAllExaminerContests() {
            await fetchExaminerContestByStatus("Active");
            await fetchExaminerContestByStatus("Scheduled");
            await fetchExaminerContestByStatus("Closed");
        }

        fetchAllExaminerContests();
        return () => {
            
        }
    }, [])

    return <div className="flex-1 py-5 px-10 min-h-0 h-screen">
        <div className="flex flex-col h-full min-h-0 gap-5">
            <div className="flex-1 min-h-0 py-6.25">
                <ContestInfoList title="Active & Upcoming Contests" contestList={upcomingContests}/>
            </div>
            <div className="flex-1 min-h-0 py-6.25">
                <ContestInfoList title="Past Contests" contestList={pastContests}/>
            </div>
        </div>
    </div>
}