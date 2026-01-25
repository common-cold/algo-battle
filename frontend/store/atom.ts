import { showErrorToast, showSuccessToast } from "@/components/ContestInfo";
import { Question } from "@/types/db";
import { JwtPayload } from "@/types/frontend";
import { CreateContestArgs, WS_BASE_URL } from "@/types/routes";
import { EndContestArgs, LogArgs, NextQuestionArgs, ResponseData, WebSocketResponse } from "@/types/ws";
import { atom } from "jotai";
import { atomWithStorage } from "jotai/utils";


export const selectedQuestionsAtom = atom<Question[]>([]);

export const contestDetailsAtom = atom<CreateContestArgs | null>(null);

export const contestJoinedAtAtom = atom<number | null>(null);

export const contestEndDateAtom = atom<number | null>(null);

export const contestSecondsAtom = atom<number | null>(null);

export const userAtom = atomWithStorage<JwtPayload | null>('user', null);

export const wsAtom = atom<WebSocket | null>(null);

export const isWsOpenAtom = atom<boolean>(false);

export const connectWsAtom = atom(null, (get, set) => {
    if (get(wsAtom) != null) {
        return wsAtom;
    }

    let token = localStorage.getItem("token");
    const ws = new WebSocket(`${WS_BASE_URL}/ws?token=${token}`);

    ws.onopen = () => {
        console.log("Websocket connected");
        set(isWsOpenAtom, true);
    }

    console.log(ws.readyState);

    ws.onmessage = (data) => {
        const responseWrapper: WebSocketResponse = JSON.parse(data.data);
        const response: ResponseData = responseWrapper.data;

        let args;
        switch(response.type) {
            
            case 'Log':
                let log = response.payload as LogArgs;
                console.log(JSON.stringify(response.payload));
                if (log.isError) {
                    showErrorToast(log.message);
                } else {
                    showSuccessToast(log.message);
                }
                break;

            case 'NextQuestion': 
                args = response.payload as NextQuestionArgs;
                console.log(JSON.stringify(response.payload));
                set(currentQuestionIdAtom, args.question_id);
                break;

            case 'EndContest':
                args = response.payload as EndContestArgs;
                console.log(JSON.stringify(response.payload));
                set(isContestOverAtom, true);
                break;
        }
    }

    ws.onclose = () => {
        set(wsAtom, null);
    }

    set(wsAtom, ws);
});

export const currentQuestionIdAtom = atom<string | null>(null);
export const isContestOverAtom = atom<boolean>(false);