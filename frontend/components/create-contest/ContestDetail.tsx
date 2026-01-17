"use client"

import { contestDetailsAtom } from "@/store/atom";
import { ContestDetailType } from "@/types/frontend";
import { CreateContestArgs } from "@/types/routes";
import { useAtom } from "jotai";
import { Dispatch, SetStateAction, useEffect, useState } from "react"
import DatePicker from "react-datepicker";
import "react-datepicker/dist/react-datepicker.css";

type LabelInputProps = {
    title: string,
    onChange: Dispatch<SetStateAction<CreateContestArgs | null>>,
    keyName: string,
    isDateComponent: boolean,
    selectedDate?: Date | null,
    handleChange?: (date: Date | null) => void
}

export default function ContestDetail() {
    const [contestDetail, setContestDetail] = useAtom(contestDetailsAtom);


    const [startDate, setStartDate] = useState<Date | null>(new Date());
    const [endDate, setEndDate] = useState<Date | null>(new Date());
    
    const handleStartDate = (date: Date | null) => {
        setStartDate(date);
        const startDateInMillis = date?.getTime();
        setContestDetail(prev => ({
            ...prev,
            start_date: startDateInMillis!/1000
        } as CreateContestArgs));
    };
    
    const handleEndDate = (date: Date | null) => {
        setEndDate(date);
        const endDateInMillis = date?.getTime();
        setContestDetail(prev => ({
            ...prev,
            end_date: endDateInMillis!/1000
        } as CreateContestArgs));
    };

    
    return <div className="flex flex-col gap-3">
        <div className="text-left font-bold textColor text-[20px]">
            Contest Details
        </div>
        <div className="flex justify-start">
            <div className="w-1/2">
                <div className="flex flex-col gap-5">
                    <div className="flex justify-between">
                        <LabelInputComponent title="Title" onChange={setContestDetail} keyName="title" isDateComponent={false}/>
                        <LabelInputComponent title="Description" onChange={setContestDetail} keyName="description" isDateComponent={false}/>
                    </div>
                    <div className="flex justify-between">
                        <LabelInputComponent title="Start Date" onChange={setContestDetail} keyName="start_ate" isDateComponent={true} selectedDate={startDate} handleChange={handleStartDate}/>
                        <LabelInputComponent title="End Date" onChange={setContestDetail} keyName="end_date" isDateComponent={true} selectedDate={endDate} handleChange={handleEndDate}/>
                    </div>
                </div>
            </div>
            
        </div>
        
    </div>
}

function LabelInputComponent({title, onChange, keyName, isDateComponent, selectedDate, handleChange}: LabelInputProps) {
    return <div className="flex flex-col gap-2">
        <div className="text-left">
            {title}
        </div>
        {
            isDateComponent
            ?
            <DatePicker 
                popperPlacement="bottom-end"
                showIcon
                selected={selectedDate} 
                onChange={handleChange} 
                showTimeSelect
                timeFormat="HH:mm"
                timeIntervals={15}
                dateFormat="dd/MM/yyyy HH:mm"
                className="border-2 border-[#9CA3AF] rounded-[7px]"
            />
            :
            <input
                onChange={(e) => onChange((prev) => ({
                    ...(prev || {}),
                    [keyName]: e.target.value
                } as CreateContestArgs))}
                className="inputBox px-3 py-2 w-80"
            />
        }
    </div>
}