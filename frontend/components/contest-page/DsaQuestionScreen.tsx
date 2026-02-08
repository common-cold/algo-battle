import { boilerplateCodesAtom, contestEndDateAtom, contestJoinedAtAtom, contestSecondsAtom, currentContestIdAtom, currentRankAtom } from "@/store/atom";
import { BoilerplateCode, Question } from "@/types/db"
import { convertEpochToIsoFormat, languageIdToNameMap } from "@/utils/common";
import { useAtom, useAtomValue } from "jotai";
import { useEffect, useState } from "react";
import { showErrorToast, showSuccessToast } from "../ContestInfo";
import { getAllBoilerplateCodes, submitMcqQuestion } from "@/utils/api";
import { Editor } from "@monaco-editor/react";

type QuestionScreenProps = {
    question: Question | null
}

type OptionProps = {
    index: number,
    onClick: () => void
}

export default function DsaQuestionScreen({question}: QuestionScreenProps) {
    const currentContestId = useAtomValue(currentContestIdAtom);
    const [selectedOption, setSelectedOption] = useState<number | null>(null);
    const [languageId, setLanguageId] = useState<number>(54);
    const [boilerplateCodes, setBoilerplateCodes] = useAtom(boilerplateCodesAtom);
    const [defaultCode, setDefaultCode] = useState<string | null>(null);
    const [candidateCode, setCandidateCode] = useState<string | null | undefined>(null);

    async function handleDsaSubmit() {
        if (selectedOption == null) {
            showErrorToast("Select an option");
            return;
        }

        if (!question || !currentContestId) {
            return;
        }

        const response = await submitMcqQuestion({
            contest_id: currentContestId,
            question_id: question.id,
            selected_option: selectedOption
        });

        if (!response) {
            showErrorToast("Error in submitting answer");
        } else if (response.status != 200) {
            const data = response.data as any;
            showErrorToast(data.error);
        } else {
            showSuccessToast("Question submitted successfully");
        }
    }

    useEffect(() => {
        async function fetchBoilerplateCodes() {
            let response = await getAllBoilerplateCodes(question!.id);
            if (!response) {
                showErrorToast("Error in fetching contest");
            } else if (response.status != 200) {
                const data = response.data as any;
                showErrorToast(data.error);
            } else {
                let boilerplateData = response.data as Array<BoilerplateCode>;
                if (boilerplateData.length == 0) {
                    showErrorToast("Boilerplates are empty");
                    return;
                }
                let map: Map<number, BoilerplateCode> = new Map();
                for (const obj of boilerplateData) {
                    map.set(obj.language_id, obj);
                }
                setBoilerplateCodes(map);
                setDefaultCode(map.get(54)!.partial_code)
                setCandidateCode(map.get(54)!.partial_code);
            }
        }

        if (!question) {
            return;
        }

        fetchBoilerplateCodes();

    }, []);

    useEffect(() => {
        if (!boilerplateCodes) {
            return;
        }
        setDefaultCode(boilerplateCodes!.get(languageId)!.partial_code);
        setCandidateCode(boilerplateCodes!.get(languageId)!.partial_code);
    }, [languageId]);


    if (!question || !boilerplateCodes) {
        return <div></div>
    }

    return <div className="flex flex-1 min-h-0 justify-between gap-5 items-center w-full">
        <div className="flex flex-col gap-7 min-h-0 w-[42%] h-full border-2 border-[#1F2937] rounded-[10px] text-left px-4 py-4 font-medium overflow-y-auto">
            <div className="font-bold text-2xl">
                {question.title}
            </div>
            <div className="font-normal">
                {question.description}
            </div>
            <div className="flex flex-col gap-3">
                Test Cases:
                <div className="flex flex-col gap-2">
                    Input: 
                    <div className="bg-[#161B26] whitespace-pre-line p-2">
                        {question.testcase_input!}
                    </div>
                </div>
                <div className="flex flex-col gap-1">
                    Output: 
                    <div className="bg-[#161B26] whitespace-pre-line p-2">
                        {question.testcase_output!}
                    </div>
                </div>
            </div>
            
        </div>

        <div className="flex flex-1 flex-col gap-2 min-h-0 h-full">
            <div className="flex justify-start gap-5">
                <select 
                    className="bg-[#161B26] px-2 py-2 items-center justify-center rounded-[7px]"        
                    onChange={(e) => setLanguageId(parseInt(e.target.value))}
                    name="selectedLanguage" defaultValue="54">
                    <option value="54">C++</option>
                    <option value="63">Javascript</option>
                    <option value="73">Rust</option>
                </select>
                <div onClick={() => {handleDsaSubmit()}}
                    className="textBgStyle5 px-3 py-2 rounded-[10px]">
                    Submit
                </div> 
            </div>
            <Editor
                height="100%"
                language={languageIdToNameMap.get(languageId)}
                theme="vs-dark"
                value={defaultCode!}
                onChange={(c) => {setCandidateCode(c)}}
            />
        </div>
    </div>
}