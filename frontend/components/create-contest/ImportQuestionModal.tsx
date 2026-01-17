import { useEffect, useRef, useState } from "react"
import { getAllExaminerQuestions } from "@/utils/api";
import { useAtom, useAtomValue } from "jotai";
import { selectedQuestionsAtom, userIdAtom } from "@/store/atom";
import { showErrorToast } from "../ContestInfo";
import { Question } from "@/types/db";


type QuestionInfoProps = {
    question: Question;
    onImport: (q: Question) => void;
}

type ImportQuestionModalProps = {
    onClose: () => void
}

export default function ImportQuestionModal({onClose} : ImportQuestionModalProps) {
    const userId = useAtomValue(userIdAtom);
    const [selectedQuestions, setSelectedQuestions] = useAtom(selectedQuestionsAtom);
    const [questions, setQuestions] = useState<Array<Question>>([]);
    const modalRef = useRef<HTMLDivElement>(null);

    function onImport(question: Question) {
        setSelectedQuestions(prev => [...prev, question]);
        let updatedQuestions = questions.filter(q => q.id != question.id);
        setQuestions(updatedQuestions);
    }

    useEffect(() => {
        function handleClick(e: MouseEvent) {
            if (modalRef.current && !modalRef.current.contains(e.target as Node)) {
                onClose();
            }
        }

        function handleEscape(e: KeyboardEvent) {
            if (e.key == "Escape") {
                onClose();
            }
        }

        window.addEventListener("mousedown", handleClick);
        window.addEventListener("keydown", handleEscape);

        return () => {
            document.removeEventListener("mousedown", handleClick);
            document.removeEventListener("keydown", handleEscape);
        }
    }, [])

    useEffect(() => {
        async function fetchExaminerQuestions() {
            const response = await getAllExaminerQuestions({
                id: userId
            });
            if (!response) {
                showErrorToast("Error in fetching saved questions");
            } else if (response.status != 200) {
                const data = response.data as any;
                showErrorToast(data.error);
            } else {
                const questions = response.data as Array<Question>;
                if (questions.length == 0) {
                    showErrorToast("You have not saved any questions yet!");
                } else {
                    let filteredQuestions = questions.filter(q => {
                        return selectedQuestions.find(item => q.id == item.id) == undefined
                    })
                    setQuestions(filteredQuestions);
                }
            }
        }

        fetchExaminerQuestions();
    }, [])



    return <div className="fixed inset-0 z-50 flex items-center justify-center min-h-0">
        <div className="absolute inset-0 bg-black/50 min-h-0">
            <div ref={modalRef}
                className="relative z-10 mx-50 my-20 px-3 py-3 flex flex-col bg-[#0f1117] h-125 max-h-125">
                {
                    questions.length == 0
                    ?
                    <div className="text-center h-full flex items-center justify-center text-xl">
                        You don't have any questons yet! 
                    </div>
                    :
                    <div className="flex flex-col gap-5 overflow-y-auto">
                        {
                            questions.map((q, index) => {
                                return <div key={index}>
                                    <QuestionRow question={q} onImport={onImport}/>
                                </div>
                            })
                        }
                    </div>
                }
            </div>
        </div>
    </div>
}

function QuestionRow({question, onImport}: QuestionInfoProps) {
    function onClick() {
        onImport(question);
    }

   return <div className="flex justify-between contestRow text-[18px] px-5 py-3 hover:!bg-[#1F2433]">
        <div className="textColor">
            {question.title}
        </div>
        <div onClick={() => onClick()}
            className="button2 px-3 font-medium">
            Import
        </div>
    </div>
}