"use client"

import Appbar from "@/components/Appbar";
import { usePathname } from "next/navigation";

export default function Layout({ children }: { children: React.ReactNode }) {
  
  let path = usePathname();
  let showCreateContestButton = path.startsWith("/contest/all");
  
  return (
    <>
      <Appbar showCreateContestButton={showCreateContestButton}/>
      {children}
    </>
  );
}