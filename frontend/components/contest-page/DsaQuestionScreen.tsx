"use client"

import { boilerplateCodesAtom, contestEndDateAtom, contestJoinedAtAtom, contestSecondsAtom, currentContestIdAtom, currentDsaAttemptIdAtom, currentRankAtom, showPostQuestionSubmitModalAtom } from "@/store/atom";
import { BoilerplateCode, Question } from "@/types/db"
import { convertEpochToIsoFormat, languageIdToNameMap } from "@/utils/common";
import { useAtom, useAtomValue } from "jotai";
import { useEffect, useState } from "react";
import { showErrorToast, showSuccessToast } from "../ContestInfo";
import { evaluateDsaQuestion, getActiveAttemptId, getAllBoilerplateCodes, getJudge0SubmissionStatus, submitDsaQuestion, submitMcqQuestion } from "@/utils/api";
import { Editor } from "@monaco-editor/react";
import { FetchAttemptResponse, SubmissionStatusResponse } from "@/types/routes";
import { Group, Panel, Separator } from "react-resizable-panels";

type QuestionScreenProps = {
    question: Question | null
}


export default function DsaQuestionScreen({question}: QuestionScreenProps) {
    const currentContestId = useAtomValue(currentContestIdAtom);
    const [currentDsaAttemptId, setCurrentDsaAttemptId] = useAtom(currentDsaAttemptIdAtom);
    const [languageId, setLanguageId] = useState<number>(54);
    const [boilerplateCodes, setBoilerplateCodes] = useAtom(boilerplateCodesAtom);
    const [defaultCode, setDefaultCode] = useState<string | null>(null);
    const [candidateCode, setCandidateCode] = useState<string | null | undefined>(null);
    const [statusResponse, setStatusResponse] = useState<SubmissionStatusResponse | null>(null);
    const [selectedTab, setSelectedTab] = useState(0);
    const [disableRun, setDisableRun] = useState(false);
    const [disableSubmit, setDisableSubmit] = useState(false);
    const [showPostQuestionSubmitModal, setShowPostQuestionSubmitModal] = useAtom(showPostQuestionSubmitModalAtom);

    async function handleDsaEvaluate() {
        if (disableRun) {
            return;
        }

        if (!question || !currentContestId) {
            return;
        }

        setDisableRun(true);
        setDisableSubmit(true);

        let attemptId = crypto.randomUUID();

        const response = await evaluateDsaQuestion({
            attempt_id: attemptId,
            contest_id: currentContestId,
            question_id: question.id,
            code: candidateCode!,
            language_id: languageId
        });

        if (!response) {
            showErrorToast("Error in evaluating answer");
        } else if (response.status != 200) {
            const data = response.data as any;
            showErrorToast(data.error);
        } else {
            setCurrentDsaAttemptId(attemptId);
        }
    }

    async function handleDsaSubmit() {
        if (disableSubmit) {
            return;
        }

        if (!question || !currentContestId|| !currentDsaAttemptId) {
            return;
        }

        setDisableSubmit(true);

        const response = await submitDsaQuestion({
            attempt_id: currentDsaAttemptId,
            contest_id: currentContestId,
            question_id: question.id
        });

        if (!response) {
            showErrorToast("Error in submitting answer");
        } else if (response.status != 200) {
            const data = response.data as any;
            showErrorToast(data.error);
        } else {
            showSuccessToast("Question submitted successfully");
        }
        setDisableSubmit(false);
        setShowPostQuestionSubmitModal(true);
    }

    useEffect(() => {
        async function fetchActiveAttemptIdIfAny() {
            let response = await getActiveAttemptId({
                contest_id: currentContestId!,
                problem_id: question?.id!
            });
            if (!response) {
                showErrorToast("Error in fetching active attemptId");
            } else if (response.status != 200) {
                const data = response.data as any;
                showErrorToast(data.error);
            } else {
                let data = response.data as FetchAttemptResponse;
                if (data.found) {
                    setCurrentDsaAttemptId(data.attempt_id!);
                    setDisableRun(true);
                    setDisableSubmit(true);
                }
            }
        }

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

        async function run() {
            await fetchBoilerplateCodes();
            await fetchActiveAttemptIdIfAny();
        }

        run()

    }, []);

    useEffect(() => {
        if (!boilerplateCodes) {
            return;
        }
        setDefaultCode(boilerplateCodes!.get(languageId)!.partial_code);
        setCandidateCode(boilerplateCodes!.get(languageId)!.partial_code);
    }, [languageId]);

    useEffect(() => {
        if (!currentDsaAttemptId) {
            setDisableRun(false);
            setDisableSubmit(false);
            return;
        }

        let stopped = false;

        async function poll() {
            if (stopped) {
                setDisableRun(false);
                setDisableSubmit(false);
                return;
            }

            try {
                let response = await getJudge0SubmissionStatus(currentDsaAttemptId!);
                if (!response) {
                    //TODO: prevent infinite polling here
                    stopped = true
                    setDisableRun(false);
                    setDisableSubmit(false);
                    return;
                } else if (response.status != 200) {
                    //TODO: prevent infinite polling here
                    stopped = true
                    setDisableRun(false);
                    setDisableSubmit(false);
                    return;
                } else {
                    let statusResponse = response.data as SubmissionStatusResponse;
                    setStatusResponse(statusResponse);
                    if (statusResponse.pending == 0) {
                        stopped = true;
                        setDisableRun(false);
                        setDisableSubmit(false);
                        return;
                    }
                }

                setTimeout(poll, 2000);
            } catch (e) {
                stopped = true;
                setDisableRun(false);
                setDisableSubmit(false);
                return;
            }
        }

        poll();

        return () => {
            stopped = true;
            setDisableRun(false);
            setDisableSubmit(false);
        }

    }, [currentDsaAttemptId])


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
            <div className="flex justify-between gap-5">
                <div className="flex justify-start gap-5">
                    <select 
                        className="bg-[#161B26] px-2 py-2 items-center justify-center rounded-[7px]"        
                        onChange={(e) => setLanguageId(parseInt(e.target.value))}
                        name="selectedLanguage" defaultValue="54">
                        <option value="54">C++</option>
                        <option value="63">Javascript</option>
                        <option value="73">Rust</option>
                    </select>
                    <div onClick={() => {handleDsaEvaluate()}}
                        className={`button5WithoutPointer px-3 py-2 font-bold ${disableRun ? "cursor-not-allowed opacity-50": "cursor-pointer"}`}>
                        Run
                    </div> 
                    {
                        statusResponse
                        &&
                        <div className="flex flex-col font-bold justify-center">
                            {`${statusResponse?.passed}/${statusResponse?.total_testcases} Testcases passed`}
                        </div>
                    }
                </div>
                <div onClick={() => {handleDsaSubmit()}}
                    className={`button4WithoutPointer px-3 py-2 font-bold ${disableSubmit ? "cursor-not-allowed opacity-50": "cursor-pointer"}`}>
                    Submit
                </div> 
            </div>
            
            <Group orientation="vertical">
                <Panel defaultSize={60}>
                    <Editor
                        height="98%"
                        language={languageIdToNameMap.get(languageId)}
                        theme="vs-dark"
                        value={defaultCode!}
                        onChange={(c) => {setCandidateCode(c)}}
                    />
                </Panel>
                <Separator/>
                <Panel defaultSize={40}>
                    <div className="flex flex-1 h-full flex-col overflow-y-auto overflow-x-auto gap-5 border-2 border-[#1F2937] p-2">
                        <div className="flex justify-start gap-2">
                            <div onClick={() => setSelectedTab(0)}
                            className={`px-2 py-1 font-bold ${selectedTab == 0 ? "button2" : "button1"}`}>
                                Failed Test Case
                            </div>
                            <div onClick={() => setSelectedTab(1)}
                            className={`px-2 py-1 font-bold ${selectedTab == 1 ? "button2" : "button1"}`}>
                                Compiled Result
                            </div>
                        </div>
                        {
                            selectedTab == 0
                            ?
                            <div className="flex flex-col gap-3">
                                <div className="flex flex-col gap-2">
                                    Expected Input: 
                                    <div className="bg-[#161B26] whitespace-pre-line p-2">
                                        {statusResponse?.failed_testcase?.input}
                                    </div>
                                </div>
                                <div className="flex flex-col gap-1">
                                    Expected Output: 
                                    <div className="bg-[#161B26] whitespace-pre-line p-2">
                                        {statusResponse?.failed_testcase?.output}
                                    </div>
                                </div>
                            </div>
                            :
                            <div className="bg-[#161B26] whitespace-pre-line p-2">
                                {statusResponse && statusResponse.compile_result ? atob(statusResponse.compile_result) : ""}
                            </div>
                        }
                    </div>
                </Panel>
            </Group>
        </div>
    </div>
}