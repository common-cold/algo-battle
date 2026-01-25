import { useEffect, useRef } from "react"

type ContestEndModalProps = {
    closeModal: () => void
}

export function ContestEndModal({closeModal} : ContestEndModalProps) {
    return <div className="fixed inset-0 z-50 flex items-center justify-center">
        <div className="absolute inset-0 bg-black/50">
            <div className="z-10 mx-100 my-20 px-3 py-5 flex flex-col items-center justify-center gap-30 bg-[#0f1117] h-100 rounded-[15px]">
                <div className="textColor font-bold text-[40px]">
                    Contest Has Ended 🏁
                </div>
                <div className="flex justify-between gap-10">
                    <div className="flex button2 items-center justify-center text-center px-8 py-2 font-bold">
                        Go to Homepage
                    </div>
                    <div className="flex button3 items-center justify-center text-center px-8 py-2 font-bold">
                        Check Leaderboard
                    </div>
                </div>
            </div>

        </div>

    </div>
}