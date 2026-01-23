"use client"

import { useRouter } from "next/navigation";
import { useState } from "react";
import jwt from "jsonwebtoken";
import { useAtom } from "jotai";
import { showErrorToast } from "../ContestInfo";
import { AuthBoxInputs } from "./Authbox";
import { signin } from "@/utils/api";
import { userAtom } from "@/store/atom";

export default function SignIn() {
    const [user, setUser] = useAtom(userAtom);
    const [password, setPassword] = useState("");
    const [email, setEmail] = useState("");
    const router = useRouter();

    async function handleSignIn() {
        const response = await signin({
            email: email,
            password: password
        });
        if (!response || response.status != 200) {
            showErrorToast("Unable to Signup");
        } else if (response.status === 200) {
            const data = response.data as any;
            const token = data.token;
            localStorage.setItem("token", token);
            const obj = jwt.decode(token) as any;
            setUser(obj);
            router.push("/contest/all");
        }    
    } 

    return <>
        <AuthBoxInputs
        name="Sign In" 
        argsTupleArray=
            {
                [
                    ["Email", setEmail],
                    ["Password", setPassword]
                ]
            } 
        handleSubmit={handleSignIn}
        isSignUp={false}
    />
    </>
}