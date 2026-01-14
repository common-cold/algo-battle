import { CreateContestArgs, CreateQuestionArgs, GetContestsArgs, GetQuestionArgs, GetQuestionsByIdArgs } from "@/types/routes";
import axios from "axios";

const API_BASE_URL = 'http://localhost:8080';

export async function createQuestion(body: CreateQuestionArgs) {
    try {
        const response = await axios.post(API_BASE_URL + "/question/create", body, {
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
        const response = await axios.post(API_BASE_URL + "/question/all/examiner", body, {
            validateStatus: () => true
        });

    return response;
    } catch (e) {
        return null;
    }
}

export async function createContest(body: CreateContestArgs) {
    try {
        const response = await axios.post(API_BASE_URL + "/contest/create", body, {
            validateStatus: () => true
        });

    return response;
    } catch (e) {
        return null;
    }
}

export async function getAllExaminerContests(body: GetContestsArgs) {
    try {
        const response = await axios.post(API_BASE_URL + "/contest/all/examiner", body, {
            validateStatus: () => true
        });

    return response;
    } catch (e) {
        return null;
    }
}