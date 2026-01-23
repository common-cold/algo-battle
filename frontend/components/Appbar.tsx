import { Signout } from "./auth/Signout";
import { CreateContestButton } from "./create-contest/CreateContestButton";

export type AppbarProps = {
  showCreateContestButton: boolean
}

export default function Appbar({showCreateContestButton}: AppbarProps) {
    return <div className="flex justify-between items-center px-10 py-5">
      <div className="font-bold text-4xl primaryTextColor">
        AlgoBattle ⚔️
      </div>
      <div className="flex justify-between gap-5">
        {
          showCreateContestButton
          &&
          <CreateContestButton/>
        }
        <Signout/>
      </div>
    </div>
}