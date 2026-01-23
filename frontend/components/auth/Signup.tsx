"use client"


import { userAtom } from "@/store/atom";
import { signup } from "@/utils/api";
import { useAtom } from "jotai";
import { useRouter } from "next/navigation";
import { useState } from "react";
import jwt from "jsonwebtoken";
import { showErrorToast } from "../ContestInfo";
import { AuthBoxInputs } from "./Authbox";
import { Role } from "@/types/db";


export default function SignUp() {
    const [user, setUser] = useAtom(userAtom);
    const [email, setEmail] = useState("");
    const [password, setPassword] = useState("");
    const [username, setUsername] = useState("");
    const [role, setRole] = useState<Role>("Candidate");
    const router = useRouter();

    async function saveUser() {
        const response = await signup({
            email: email,
            name: username,
            password: password,
            role: role
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

    return <AuthBoxInputs 
        name="Sign Up" 
        argsTupleArray=
            {
                [
                    ["Email", setEmail], 
                    ["Username", setUsername],
                    ["Password", setPassword]
                ]
            } 
        handleSubmit={saveUser}
        isSignUp={true}
        setRole={setRole}
    />
}