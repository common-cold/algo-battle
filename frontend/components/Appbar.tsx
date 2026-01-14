import { CreateContestButton } from "./create-contest/CreateContestButton";

export default function Appbar() {
    return <div className="flex justify-between items-center px-10 py-5">
      <div className="font-bold text-4xl primaryTextColor">
        AlgoBattle ⚔️
      </div>
      <CreateContestButton/>
    </div>
}