use actix_web::{HttpResponse, get, post, web::{self, Data}};
use common::{CreateDsaSubmissionArgs, CreateMcqQuestionArgs, CreateSubmissionArgs, EvaluateDsaQuestionArgs, GetQuestionArgs, GetQuestionsByIdArgs, Judge0BatchSubmissionDto, Judge0SubmissionDto, Judge0SubmissionResponseDto, JwtClaims, SubmitDsaQuestionArgs, SubmitQuestionArgs};
use page_hunter::paginate_records;
use serde_json::json;
use sqlx::types::Uuid;

use crate::{AppData};

#[post("/question/create/mcq")]
pub async fn create_question(app_data: Data<AppData>, body: web::Json<CreateMcqQuestionArgs>, jwt_claims: JwtClaims) -> HttpResponse {
    let app_data= app_data.get_ref();
    let db = &app_data.db;


    let question_result =  db.create_question(body.0.question_type, Some(jwt_claims.id)).await;
    if let Err(e) = question_result {
        return HttpResponse::InternalServerError().json(json!({
            "error": e.to_string()
        }))
    }

    let question = question_result.unwrap();
    match db.create_mcq_question(body.0, question.id).await {
        Ok(mcq_question) => {
            HttpResponse::Ok().json(mcq_question)
        }
        Err(e) => {
            HttpResponse::InternalServerError().json(json!({
                "error": e.to_string()
            }))
        }
    }
}

#[post("/question/all")]
pub async fn get_questions_by_id(app_data: Data<AppData>, body: web::Json<GetQuestionsByIdArgs>) -> HttpResponse {
    let app_data = app_data.get_ref();
    let db = &app_data.db;

    match db.get_questions_by_id(body.0.ids).await {
        Ok(questions) => {
            HttpResponse::Ok().json(questions)
        }
        Err(e) => {
            HttpResponse::InternalServerError().json(json!({
                "error": e.to_string()
            }))
        }
    }
}

#[get("/question/dsa/all")]
pub async fn get_all_dsa_questions(app_data: Data<AppData>) -> HttpResponse {
    let app_data = app_data.get_ref();
    let db = &app_data.db;

    match db.get_all_dsa_questions().await {
        Ok(dsa_questions) => {
            HttpResponse::Ok().json(dsa_questions)
        }
        Err(e) => {
            HttpResponse::InternalServerError().json(json!({
                "error": e.to_string()
            }))
        }
    }
}

#[post("/question/all/examiner/mcq")]
pub async fn get_all_examiner_mcq_questions(app_data: Data<AppData>, body: web::Json<GetQuestionArgs>, jwt_claims: JwtClaims) -> HttpResponse {
    let app_data= app_data.get_ref();
    let db = &app_data.db;

    let questions_result = db.get_all_examiner_mcq_questions(jwt_claims.id).await;
    
    if let Err(e) = questions_result {
        return HttpResponse::InternalServerError().json(json!({
            "error": e.to_string()
        }));
    }

    let question_ids = questions_result.unwrap()
        .iter()
        .map(|q| q.id)
        .collect::<Vec<Uuid>>();

    match db.get_mcq_questions_by_id(question_ids).await{
        Ok(mcq_questions) => {
            if body.limit.is_none() || body.page.is_none() {
                return HttpResponse::Ok().json(mcq_questions);
            }
            
            let mut page = 0;
            let mut limit = 5;
            if body.page.is_some() {
                page = body.page.unwrap();
            }
            if body.limit.is_some() {
                limit = body.limit.unwrap();
            }
            match paginate_records(&mcq_questions, page, limit) {
                Ok(p) => HttpResponse::Ok().json(p),

                Err(e) => HttpResponse::InternalServerError().json(json!({
                    "error": e.to_string()
                }))
            }
        }
        Err(e) => {
            HttpResponse::InternalServerError().json(json!({
                "error": e.to_string()
            }))
        }
    }
}

#[post("/question/submit/mcq")]
pub async fn submit_mcq_question(app_data: Data<AppData>, body: web::Json<SubmitQuestionArgs>, jwt_claims: JwtClaims) -> HttpResponse {
    let app_data = app_data.get_ref();
    let db = &app_data.db;
    let leaderboard_service = &app_data.leaderboard_service;

    let contest_result = db.get_contest_by_id(body.contest_id).await;
    if let Err(e) = contest_result {
        return HttpResponse::InternalServerError().json(json!({
            "error": e.to_string()
        }));
    };

    let result = db.get_correct_option_and_points(body.question_id).await;
    if let Err(e) = result {
        return HttpResponse::InternalServerError().json(json!({
            "error": e.to_string()
        }));
    };
    let (correct_option, points) = result.unwrap();
    let points_earned;
    if body.selected_option != correct_option {
        points_earned = 0;
    } else {
        points_earned = points;
    }

    if let Err(e) = leaderboard_service.update_score(body.contest_id, jwt_claims.id, points_earned).await {
        return HttpResponse::InternalServerError().json(json!({
            "error": e.to_string()
        }));
    }

    let args = CreateSubmissionArgs {
        contest_id: body.contest_id,
        question_id: body.question_id,
        user_id: jwt_claims.id,
        points_earned: points_earned
    };

    if let Err(e) = db.create_submission(args).await {
        return HttpResponse::InternalServerError().json(json!({
            "error": e.to_string()
        }));
    }

    return HttpResponse::Ok().json(json!({
        "data": "Answer Submitted"
    }));
}


#[post("/question/submit/dsa")]
pub async fn submit_dsa_question(app_data: Data<AppData>, body: web::Json<SubmitDsaQuestionArgs>, jwt_claims: JwtClaims) -> HttpResponse {
    let app_data = app_data.get_ref();
    let db = &app_data.db;
    let leaderboard_service = &app_data.leaderboard_service;

    let contest_result = db.get_contest_by_id(body.contest_id).await;
    if let Err(e) = contest_result {
        return HttpResponse::InternalServerError().json(json!({
            "error": e.to_string()
        }));
    };

    let question_result = db.get_dsa_questions_by_id(vec![body.question_id]).await;
    if let Err(e) = question_result {
        return HttpResponse::InternalServerError().json(json!({
            "error": e.to_string()
        }));
    } 


    let submissions_result = db.get_dsa_submissions_by_attempt_id(body.attempt_id.clone()).await;
    if let Err(e) = submissions_result {
        return HttpResponse::InternalServerError().json(json!({
            "error": e.to_string()
        }));
    }

    let submissions = submissions_result.unwrap();
    
    let mut total: i32 = 0;
    let mut passed: i32 = 0;

    for submission in submissions {
        total += 1;
        if submission.status == "Accepted" {
            passed +=1;
        }
    }

    println!("Total: {}", total);
    println!("Passed: {}", passed);

    let ratio = if total > 0 {
        passed as f32 / total as f32
    } else {
        0.0
    };

    let points_earned = (ratio * question_result.unwrap()[0].points as f32) as i32;

    if let Err(e) = leaderboard_service.update_score(body.contest_id, jwt_claims.id, points_earned).await {
        return HttpResponse::InternalServerError().json(json!({
            "error": e.to_string()
        }));
    }

    let args = CreateSubmissionArgs {
        contest_id: body.contest_id,
        question_id: body.question_id,
        user_id: jwt_claims.id,
        points_earned: points_earned
    };

    if let Err(e) = db.create_submission(args).await {
        return HttpResponse::InternalServerError().json(json!({
            "error": e.to_string()
        }));
    }

    return HttpResponse::Ok().json(json!({
        "data": "Answer Submitted"
    }));
}


#[post("/question/evaluate/dsa")]
pub async fn evaluate_dsa_question(app_data: Data<AppData>, body: web::Json<EvaluateDsaQuestionArgs>, jwt_claims: JwtClaims) -> HttpResponse {
    
    let app_data = app_data.get_ref();
    let db = &app_data.db;

    let contest_result = db.get_contest_by_id(body.contest_id).await;
    if let Err(e) = contest_result {
        return HttpResponse::InternalServerError().json(json!({
            "error": e.to_string()
        }));
    };

    let boilerplate_result = db.get_boilerplate_code_for_language_id(body.question_id, body.language_id).await;
    if let Err(e) = boilerplate_result {
        return HttpResponse::InternalServerError().json(json!({
            "error": e.to_string()
        }));
    }

    let source_code = boilerplate_result.unwrap().full_code.replace("##CODE_HERE##", &body.code);

    let testcase_result = db.get_all_testcases_for_question(body.question_id).await;
    if let Err(e) = testcase_result {
        return HttpResponse::InternalServerError().json(json!({
            "error": e.to_string()
        }));
    };

    let mut testcase_id_list = Vec::<Uuid>::new();
    let mut batch_submisison_dto = Vec::<Judge0SubmissionDto>::new();
    for testcase in testcase_result.unwrap() {
        let dto = Judge0SubmissionDto {
            source_code: source_code.clone(),
            language_id: body.language_id,
            stdin: testcase.input,
            expected_output: testcase.output,
            callback_url: String::from("http://172.17.0.1:8082/judge0-submission/callback")
        };
        batch_submisison_dto.push(dto);
        testcase_id_list.push(testcase.id);
    };

    let client = reqwest::Client::new();
    let url = String::from("http://localhost:2358/submissions/batch?base64_encoded=false");

    let response = client
        .post(url)
        .json(&Judge0BatchSubmissionDto {
            submissions: batch_submisison_dto.clone()
        })
        .send()
        .await;
    
    match response {
        Ok(res) => {
            let parsed_result: Result<Vec<Judge0SubmissionResponseDto>, reqwest::Error> = res.json().await;
            if let Err(e) = parsed_result {
                return HttpResponse::InternalServerError().json(json!({
                    "error": e.to_string()
                }));
            }
            
            let parsed = parsed_result.unwrap();

            for (idx, obj) in parsed.iter().enumerate() {
                let testcase_id = testcase_id_list[idx];
                let args = CreateDsaSubmissionArgs {
                    attempt_id: body.attempt_id.clone(),
                    submission_id: obj.token.clone(),
                    problem_id: body.question_id,
                    contest_id: body.contest_id,
                    testcase_id: testcase_id,
                    user_id: jwt_claims.id,
                    status: String::from("Pending")
                };

                let submission_result = db.create_dsa_submission(args).await;
                if let Err(e) = submission_result {
                    return HttpResponse::InternalServerError().json(json!({
                        "error": e.to_string()
                    }));
                }
            }            
        },
        Err(e) => {
            return HttpResponse::InternalServerError().json(json!({
                "error": e.to_string()
            }));
        }
    }

    return HttpResponse::Ok().finish();
}