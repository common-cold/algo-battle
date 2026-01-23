"use client"

import { AuthHeaderComponent } from "@/components/auth/AuthHeaderComponent";
import SignIn from "@/components/auth/Signin";
import SignUp from "@/components/auth/Signup";
import { TabComponent } from "@/components/common-components/TabComponent";
import { userAtom } from "@/store/atom";
import { useAtomValue } from "jotai";
import { useRouter } from "next/navigation";

import { useEffect, useState } from "react";


export default function AuthPage() {
    const user = useAtomValue(userAtom);
    const [tab, setTab] = useState(0);

    const router = useRouter();

    useEffect(() => {
        let token = localStorage.getItem("token");
        if (token != null) {
            router.push("/contest/all");
        }
    }, []);

    return <div className="flex-1 py-5 px-20 h-screen">
        <div className="flex flex-col justify-center items-center gap-5">
            <AuthHeaderComponent/>
            <div className="flex flex-col justify-center items-center gap-5 bg-[#161B26] rounded-[10px] py-5">
                <div className="flex justify-start gap-6">
                    <TabComponent tab={tab} setTab={setTab} index={0} label="SignUp" />
                    <TabComponent tab={tab} setTab={setTab} index={1} label="SignIn" />
                </div>
                {
                    tab === 0
                    ?
                    <SignUp/>
                    :
                    <SignIn/>
                }
            </div>
        </div>
    </div>
}