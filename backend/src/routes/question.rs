use actix_web::{HttpResponse, post, web::{self, Data}};
use common::{CreateQuestionArgs, GetQuestionArgs, GetQuestionsByIdArgs, JwtClaims, SubmitQuestionArgs};
use page_hunter::paginate_records;
use serde_json::json;

use crate::AppData;

#[post("/question/create")]
pub async fn create_question(app_data: Data<AppData>, body: web::Json<CreateQuestionArgs>, jwt_claims: JwtClaims) -> HttpResponse {
    let app_data= app_data.get_ref();
    let db = &app_data.db;


    match db.create_question(body.0, jwt_claims.id).await {
        Ok(user) => {
            HttpResponse::Ok().json(user)
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

#[post("/question/all/examiner")]
pub async fn get_all_examiner_questions(app_data: Data<AppData>, body: web::Json<GetQuestionArgs>, jwt_claims: JwtClaims) -> HttpResponse {
    let app_data= app_data.get_ref();
    let db = &app_data.db;

    match db.get_all_examiner_questions(jwt_claims.id).await {
        Ok(questions) => {
            if body.limit.is_none() || body.page.is_none() {
                return HttpResponse::Ok().json(questions);
            }
            
            let mut page = 0;
            let mut limit = 5;
            if body.page.is_some() {
                page = body.page.unwrap();
            }
            if body.limit.is_some() {
                limit = body.limit.unwrap();
            }
            match paginate_records(&questions, page, limit) {
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

    if body.selected_option != correct_option {
        return HttpResponse::Ok().json(json!({
            "data": "Answer Submitted"
        }));
    }

    if let Err(e) = leaderboard_service.update_score(body.contest_id, jwt_claims.id, points).await {
        return HttpResponse::InternalServerError().json(json!({
            "error": e.to_string()
        }));
    }

    return HttpResponse::Ok().json(json!({
        "data": "Answer Submitted"
    }));
}