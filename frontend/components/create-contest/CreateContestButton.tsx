"use client"

import { useRouter } from "next/navigation";

export function CreateContestButton() {
    const router = useRouter();

    return <div onClick={() => router.replace("/contest/new")}
    className="flex button3 items-center justify-center text-center px-8 py-2 font-bold">
        Create New Contest
    </div>
}