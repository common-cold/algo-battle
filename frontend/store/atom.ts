import { Question } from "@/types/db";
import { CreateContestArgs } from "@/types/routes";
import { atom } from "jotai";


export const userIdAtom = atom("62e0e171-a0e0-4ab9-ad07-d23c3e15a75f");
export const candidateIdAtom = atom("eba3ded8-43d7-4f65-86ea-2900147cb05b");
export const selectedQuestionsAtom = atom<Question[]>([]);
export const contestDetailsAtom = atom<CreateContestArgs | null>(null);
export const contestJoinedAtAtom = atom<number | null>(null);
export const contestEndDateAtom = atom<number | null>(null);
export const contestSecondsAtom = atom<number | null>(null);