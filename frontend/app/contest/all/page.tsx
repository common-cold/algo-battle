"use client"

import { showErrorToast } from "@/components/ContestInfo";
import ContestInfoList from "@/components/ContestInfoList";
import { userAtom } from "@/store/atom";
import { Contest, ContestStatus, Role } from "@/types/db";
import { getAllContests, getAllExaminerContests } from "@/utils/api";
import { useAtomValue } from "jotai";
import { useRouter } from "next/navigation";
import { useEffect, useState } from "react";


export default function AllContest() {
    const [upcomingContests, setUpcomingContests] = useState<Contest[]>([]);
    const [pastContests, setPastContests] = useState<Contest[]>([]);
    const user = useAtomValue(userAtom);

    const router = useRouter();


    useEffect(() => {
        async function fetchContestByStatus(status: ContestStatus, role: Role) {
            let response;
            if (role === "Examiner") {
                response = await getAllExaminerContests({
                    status: status
                });
            } else {
                response = await getAllContests({
                    status: status
                });
            }
            
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
            await fetchContestByStatus("Active", user!.role);
            await fetchContestByStatus("Scheduled", user!.role);
            await fetchContestByStatus("Closed", user!.role);
        }
        
        let token = localStorage.getItem("token");
        if (!token) {
            router.replace("/");
        }
        if (!user) {
            return;
        }

        fetchAllExaminerContests();
        return () => {
            
        }
    }, [user])

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