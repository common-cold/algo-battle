export function PostQuestionSubmitModal() {
    return <div className="fixed inset-0 z-50 flex items-center justify-center">
        <div className="absolute z-50 bg-black/30">
            <div className="z-10 mx-100 my-20 px-5 py-5 flex flex-col items-center justify-center gap-30 bg-[#0f1117] h-100 rounded-[15px]">
                <div className="textColor font-bold text-[40px]">
                    ⏳ Answer Submitted
                </div>
                <div className="text-center">
                    Your answer has been recorded.
                    <br/>
                    You’ll be able to continue when this question’s time window ends.
                </div>
                
            </div>
        </div>

    </div>
}