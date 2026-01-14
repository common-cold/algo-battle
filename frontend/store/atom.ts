import { Question } from "@/types/db";
import { atom } from "jotai";


export const userIdAtom = atom("62e0e171-a0e0-4ab9-ad07-d23c3e15a75f");
export const selectedQuestionsAtom = atom<Question[]>([]);