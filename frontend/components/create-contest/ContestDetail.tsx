"use client"

import { ContestDetailType } from "@/types/frontend";
import { Dispatch, SetStateAction, useState } from "react"

type LabelInputProps = {
    title: string,
    onChange: Dispatch<SetStateAction<ContestDetailType | null>>,
    keyName: string
}

export default function ContestDetail() {
    const [contestDetail, setContestDetail] = useState<ContestDetailType | null>(null);

    console.log(contestDetail);

    return <div className="flex flex-col gap-3">
        <div className="text-left font-bold textColor text-[20px]">
            Contest Details
        </div>
        <div className="flex justify-start">
            <div className="w-1/2">
                <div className="flex flex-col gap-5">
                    <div className="flex justify-between">
                        <LabelInputComponent title="Title" onChange={setContestDetail} keyName="title"/>
                        <LabelInputComponent title="Description" onChange={setContestDetail} keyName="description"/>
                    </div>
                    <div className="flex justify-between">
                        <LabelInputComponent title="Start Date" onChange={setContestDetail} keyName="startDate"/>
                        <LabelInputComponent title="End Date" onChange={setContestDetail} keyName="endDate"/>
                    </div>
                </div>
            </div>
            
        </div>
        
    </div>
}

function LabelInputComponent({title, onChange, keyName}: LabelInputProps) {
    return <div className="flex flex-col gap-2">
        <div className="text-left">
            {title}
        </div>
        <input
            onChange={(e) => onChange((prev) => ({
                ...(prev || {}),
                [keyName]: e.target.value
            } as ContestDetailType))}
            className="inputBox px-3 py-2 w-80"
        />
    </div>
}