"use client"

import Appbar from "@/components/Appbar";
import { userAtom } from "@/store/atom";
import { useAtomValue } from "jotai";
import { usePathname } from "next/navigation";

export default function Layout({ children }: { children: React.ReactNode }) {

  const user = useAtomValue(userAtom);
  
  let path = usePathname();
  let showCreateContestButton = false;
  if (path.startsWith("/contest/all") && user?.role === "Examiner") {
    showCreateContestButton = true;
  }
  
  return (
    <>
      <Appbar showCreateContestButton={showCreateContestButton}/>
      {children}
    </>
  );
}