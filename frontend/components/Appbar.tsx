import { useRouter } from "next/navigation";
import { Signout } from "./auth/Signout";
import { CreateContestButton } from "./create-contest/CreateContestButton";

export type AppbarProps = {
  showCreateContestButton: boolean
}

export default function Appbar({showCreateContestButton}: AppbarProps) {
    const router = useRouter();

    return <div className="flex justify-between items-center px-10 py-5">
      <div 
        onClick={() => router.replace("/contest/all")}
        className="font-bold text-4xl primaryTextColor cursor-pointer">
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