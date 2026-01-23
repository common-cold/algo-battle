"use client"

import ContestDetail from "@/components/create-contest/ContestDetail";
import QuestionSection from "@/components/create-contest/QuestionSection";
import { SaveContestButton } from "@/components/create-contest/SaveContestButton";
import { userAtom } from "@/store/atom";
import { useAtomValue } from "jotai";
import { useRouter } from "next/navigation";
import { useEffect } from "react";

export default function CreateContestPage() {
    const user = useAtomValue(userAtom);
    
    const router = useRouter();

    useEffect(() => {
        let token = localStorage.getItem("token");
        if (!token) {
            router.replace("/");
        }

        if (!user) {
            return;
        }
    }, [user]);

    return <div className="flex-1 h-screen py-5 px-10 overflow-hidden">
        <div className="flex flex-col h-full gap-10">
            <div className="flex justify-between">
                <div className="text-left font-bold textColor text-[30px]">
                    Create New Contest
                </div>
                <SaveContestButton/>
            </div>
            
            <ContestDetail/>
            <QuestionSection/>
        </div>
    </div>
}

