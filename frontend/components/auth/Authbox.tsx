import { Role } from "@/types/db"

export type AuthBoxInputProps = {
    name: string,
    argsTupleArray: [string, (val: string) => void][],
    setRole?: (val: Role) => void,
    handleSubmit: () => Promise<void>,
    isSignUp: boolean
}

export function AuthBoxInputs({name, argsTupleArray, handleSubmit, isSignUp, setRole}: AuthBoxInputProps) {
    return <div className={`flex flex-col gap-5 secondaryBg  w-100 rounded-[10px] py-3 px-6`}>
        <div className="flex text-white text-2xl font-bold items-center justify-center">
            {name}
        </div>
        {
            argsTupleArray.map(([label, setter], i) => {
                return <div key={i} className="flex flex-col text-white gap-1">
                    <div className="flex text-white text-lg font-bold justify-start">
                        {label}
                    </div>
                    <div>
                        <input 
                            type={label == 'Password' ? 'password' : 'text'}
                            className={`inputBox2 w-full px-2 py-2`} 
                            onChange={(e) => setter(e.target.value)}
                        />
                    </div>
                </div>
            })
        }
        {
            isSignUp
            &&
            <div className="flex flex-col text-white gap-1">
            <div className="flex text-white text-lg font-bold justify-start">
                    Role
                </div>
                <select className="bg-[#161B26] px-2 py-2 items-center justify-center rounded-[7px] inputBox2"        
                    onChange={(e) => setRole!(e.target.value as Role)} defaultValue="Candidate">
                    <option value="Candidate">Candidate</option>
                    <option value="Examiner">Examiner</option>
                </select>
            </div>
        }
        
        <div onClick={() => {
            async function callHandleSubmit() {
                await handleSubmit();
            }

            callHandleSubmit();
        }} 
            className="flex items-center justify-center w-full py-2 mt-2 button2 font-bold rounded-[10px] cursor-pointer">
                Submit
        </div>
    </div>
}