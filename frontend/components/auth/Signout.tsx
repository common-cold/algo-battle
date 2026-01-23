"use client"

import { userAtom } from "@/store/atom";
import { useAtom } from "jotai";
import { useRouter } from "next/navigation";

export function Signout() {
    const [user, setUser] = useAtom(userAtom);
    const router = useRouter();

    return  <div onClick={() => {
        localStorage.removeItem("token");
        setUser(null);
        router.replace("/");
    }} 
        className="flex button1 items-center justify-center text-center px-8 py-2 font-bold">
        Signout
    </div>

}