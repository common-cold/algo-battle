"use client"

import { useRouter } from "next/navigation";

export function SaveContestButton() {
    const router = useRouter();
    
    return <div onClick={() => router.push("/contest/new")}
        className="flex button4 items-center justify-center text-center px-8 py-2 font-bold">
            Save Contest
    </div>
}