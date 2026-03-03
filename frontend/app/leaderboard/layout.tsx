"use client"

import Appbar from "@/components/Appbar";

export default function Layout({ children }: { children: React.ReactNode }) {
return (
    <>
      <Appbar showCreateContestButton={false}/>
      {children}
    </>
  );
}