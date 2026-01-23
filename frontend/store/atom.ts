import { showErrorToast, showSuccessToast } from "@/components/ContestInfo";
import { Question } from "@/types/db";
import { JwtPayload } from "@/types/frontend";
import { CreateContestArgs, WS_BASE_URL } from "@/types/routes";
import { LogArgs, ResponseData, WebSocketResponse } from "@/types/ws";
import { atom } from "jotai";
import { atomWithStorage } from "jotai/utils";


export const candidateIdAtom = atom("eba3ded8-43d7-4f65-86ea-2900147cb05b");
export const selectedQuestionsAtom = atom<Question[]>([]);
export const contestDetailsAtom = atom<CreateContestArgs | null>(null);
export const contestJoinedAtAtom = atom<number | null>(null);
export const contestEndDateAtom = atom<number | null>(null);
export const contestSecondsAtom = atom<number | null>(null);
export const userAtom = atomWithStorage<JwtPayload | null>('user', null);
export const wsAtom = atom<WebSocket | null>(null);
export const connectWs = atom(null, (get, set) => {
    if (get(wsAtom) != null) {
        return wsAtom;
    }

    let token = localStorage.getItem("token");
    const ws = new WebSocket(`${WS_BASE_URL}/ws?token=${token}`);

    ws.onopen = () => {
        console.log("Websocket connected");
    }

    ws.onmessage = (data) => {
        const responseWrapper: WebSocketResponse = JSON.parse(data.data);
        const response: ResponseData = responseWrapper.data;

        switch(response.type) {
            case 'Log':
                let log = response.payload as LogArgs;
                if (log.isError) {
                    showErrorToast(log.message);
                } else {
                    showSuccessToast(log.message);
                }
            break;    
        }
    }
})