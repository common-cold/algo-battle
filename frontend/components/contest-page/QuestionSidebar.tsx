import { Question } from "@/types/db"
import { useState } from "react"

type QuestionSidebarProps = {
    questions: Question[],
    onClick: (question: Question) => void
}

export default function QuestionSidebar({questions, onClick}: QuestionSidebarProps) {
    const [selectedOption, setSelectedOption] = useState(0);

   return  <div className="flex min-h-0 flex-col gap-5 border-2 border-[#1F2937] rounded-[10px] w-1/4 px-3 py-3">
        <div className="text-left font-bold textColor2 text-[25px] px-2">
            Questions
        </div>
        <div className="flex flex-1 flex-col gap-2 min-h-0 overflow-y-auto">
            {
                questions.map((q, index) => {
                    return <div 
                        key={index}
                        onClick={() => {
                            setSelectedOption(index)
                            onClick(q)
                        }}
                        className={`
                            ${selectedOption == index && "textBgStyle1"}
                            px-4 py-3 rounded-[10px] cursor-pointer
                        `}
                        >
                        {
                            q.title
                        }
                    </div>    
                })
            }
        </div>
        
    </div>
}