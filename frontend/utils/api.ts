import { API_BASE_URL, CreateContestArgs, CreateQuestionArgs, GetContestsArgs, GetQuestionArgs, GetQuestionsByIdArgs, SignInArgs, SignupArgs } from "@/types/routes";
import axios from "axios";


export async function signup(body: SignupArgs) {
    try {
        const response = await axios.post(API_BASE_URL + "/signup", body, {
            validateStatus: () => true
        });

    return response;
    } catch (e) {
        return null;
    }
}

export async function signin(body: SignInArgs) {
    try {
        const response = await axios.post(API_BASE_URL + "/signin", body, {
            validateStatus: () => true
        });

    return response;
    } catch (e) {
        return null;
    }
}

export async function createQuestion(body: CreateQuestionArgs) {
    try {
        let token = localStorage.getItem("token");
        const response = await axios.post(API_BASE_URL + "/question/create", body, {
            headers: {
                Authorization: token
            },
            validateStatus: () => true
        });

    return response;
    } catch (e) {
        return null;
    }
}

export async function getQuestionsById(body: GetQuestionsByIdArgs) {
    try {
        const response = await axios.post(API_BASE_URL + "/question/all", body, {
            validateStatus: () => true
        });

    return response;
    } catch (e) {
        return null;
    }
}

export async function getAllExaminerQuestions(body: GetQuestionArgs) {
    try {
        let token = localStorage.getItem("token");
        const response = await axios.post(API_BASE_URL + "/question/all/examiner", body, {
            headers: {
                Authorization: token
            },
            validateStatus: () => true
        });

    return response;
    } catch (e) {
        return null;
    }
}

export async function createContest(body: CreateContestArgs) {
    try {
        let token = localStorage.getItem("token");
        const response = await axios.post(API_BASE_URL + "/contest/create", body, {
            headers: {
                Authorization: token
            },
            validateStatus: () => true
        });

    return response;
    } catch (e) {
        return null;
    }
}

export async function getAllExaminerContests(body: GetContestsArgs) {
    try {
        let token = localStorage.getItem("token");
        const response = await axios.post(API_BASE_URL + "/contest/all/examiner", body, {
            headers: {
                Authorization: token
            },
            validateStatus: () => true
        });

    return response;
    } catch (e) {
        return null;
    }
}

export async function getAllContests(body: GetContestsArgs) {
    try {
        let token = localStorage.getItem("token");
        const response = await axios.post(API_BASE_URL + "/contest/all", body, {
            headers: {
                Authorization: token
            },
            validateStatus: () => true
        });

    return response;
    } catch (e) {
        return null;
    }
}

export async function getFullContest(contestId: string) {
    try {
        const response = await axios.get(API_BASE_URL + `/contest/full/${contestId}`, {
            validateStatus: () => true
        });

    return response;
    } catch (e) {
        return null;
    }
}

export async function joinContest(contestId: string) {
    try {
        let token = localStorage.getItem("token");
        const response = await axios.get(API_BASE_URL + `/contest/join/${contestId}`, {
            headers: {
                Authorization: token
            },
            validateStatus: () => true
        });

    return response;
    } catch (e) {
        return null;
    }
}

export async function getContestJoinedAt(userId: string, contestId: string) {
    try {
        const response = await axios.get(API_BASE_URL + `/contest/joinedAt?userId=${userId}&contestId=${contestId}`, {
            validateStatus: () => true
        });

    return response;
    } catch (e) {
        return null;
    }
}