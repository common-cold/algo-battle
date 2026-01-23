use actix_web::{HttpResponse, get, post, web::{self, Data}};
use common::{ContestStatus, CreateContestArgs, FullContest, GetContestArgs, GetContestJoinedAt, JwtClaims, Role};
use page_hunter::paginate_records;
use serde_json::json;
use sqlx::types::Uuid;

use crate::{AppData};

#[post("/contest/create")]
pub async fn create_contest(app_data: Data<AppData>, body: web::Json<CreateContestArgs>, jwt_claims: JwtClaims) -> HttpResponse {
    let app_data= app_data.get_ref();
    let db = &app_data.db;

    let create_result = db.create_contest(body.clone(), jwt_claims.id).await;
    if let Err(e) = create_result {
        return HttpResponse::InternalServerError().json(json!({
            "error": e.to_string()
        }));
    }

    let contest = create_result.unwrap();

    match db.create_contest_question_join_entry(contest.id, &body.question_ids).await {
        Ok(()) => {
            HttpResponse::Ok().json(contest)
        }
        Err(e) => {
            return HttpResponse::InternalServerError().json(json!({
                "error": e.to_string()
            }));
        }
    }
}

#[post("/contest/all/examiner")]
pub async fn get_all_examiner_contests(app_data: Data<AppData>, body: web::Json<GetContestArgs>, jwt_claims: JwtClaims) -> HttpResponse {
    let app_data= app_data.get_ref();
    let db = &app_data.db;

    match db.get_all_examiner_contests(body.0.clone(), jwt_claims.id).await {
        Ok(contests) => {
            if body.limit.is_none() || body.page.is_none() {
                return HttpResponse::Ok().json(contests);
            }

            let mut page = 0;
            let mut limit = 5;
            if body.0.page.is_some() {
                page = body.page.unwrap();
            }
            if body.0.limit.is_some() {
                limit = body.limit.unwrap();
            }
            match paginate_records(&contests, page, limit) {
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

#[post("/contest/all")]
pub async fn get_all_contests(app_data: Data<AppData>, body: web::Json<GetContestArgs>, _jwt_claims: JwtClaims) -> HttpResponse {
    let app_data= app_data.get_ref();
    let db = &app_data.db;

    match db.get_all_contests(body.0.status).await {
        Ok(contests) => {
            if body.limit.is_none() || body.page.is_none() {
                return HttpResponse::Ok().json(contests);
            }

            let mut page = 0;
            let mut limit = 5;
            if body.0.page.is_some() {
                page = body.page.unwrap();
            }
            if body.0.limit.is_some() {
                limit = body.limit.unwrap();
            }
            match paginate_records(&contests, page, limit) {
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

#[get("/contest/full/{id}")]
pub async fn get_full_contest_by_id(app_data: Data<AppData>, path: web::Path<Uuid>) -> HttpResponse {
    let contest_id = path.into_inner();
    let db = &app_data.db;

    let contest_result = db.get_contest_by_id(contest_id).await;
    if let Err(e) = contest_result {
        return HttpResponse::InternalServerError().json(json!({
            "error": e.to_string()
        }))
    }
    let contest = contest_result.unwrap();

    let question_ids_result = db.get_all_question_ids_for_contest_id(contest_id).await;
    if let Err(e) = question_ids_result {
        return HttpResponse::InternalServerError().json(json!({
            "error": e.to_string()
        }))
    }
    let question_ids = question_ids_result.unwrap();

    let question_list_result = db.get_questions_by_id(question_ids).await;
    if let Err(e) = question_list_result {
        return HttpResponse::InternalServerError().json(json!({
            "error": e.to_string()
        }))
    }
    let question_list = question_list_result.unwrap();

    return HttpResponse::Ok().json(FullContest {
        contest: contest,
        questions: question_list
    });
}

#[get("/contest/join/{contest_id}")]
pub async fn join_contest(app_data: Data<AppData>, path: web::Path<Uuid>, jwt_claims: JwtClaims) -> HttpResponse {
    let db = &app_data.db;

    let contest_id = path.into_inner();

    match db.get_user_by_id(jwt_claims.id).await {
        Ok(user) => {
            if !matches!(user.role, Role::Candidate) {
                return HttpResponse::InternalServerError().json(json!({
                    "error": "Only user with role Candidate can join"
                }));
            }
        }
        Err(e) => {
            return HttpResponse::InternalServerError().json(json!({
                "error": e.to_string()
            }));
        }
    }

    match db.get_contest_by_id(contest_id).await {
        Ok(contest) => {
            match contest.status {
                ContestStatus::Closed => {
                    return HttpResponse::InternalServerError().json(json!({
                        "error": "Contest has already ended"
                    }));
                }
                ContestStatus::Scheduled => {
                    return HttpResponse::InternalServerError().json(json!({
                        "error": "Contest has not started yet"
                    }));
                }
                _ => {}
            }
        }
        Err(e) => {
            return HttpResponse::InternalServerError().json(json!({
                "error": e.to_string()
            }));
        }
    }

    match db.create_contest_attempts_entry(jwt_claims.id, contest_id).await {
        Ok(()) => {
            HttpResponse::Ok().json(json!({
                "message": "Contest joined successfully"
            }))
        }
        Err(e) => {
            HttpResponse::InternalServerError().json(json!({
                "error": e.to_string()
            }))
        }
    }
}


#[get("/contest/joinedAt")]
pub async fn get_contest_joined_at(app_data: Data<AppData>, query: web::Query<GetContestJoinedAt>) -> HttpResponse {
    let db = &app_data.db;

    match db.get_contest_joined_at(query.user_id, query.contest_id).await {
        Ok(joined_at) => {
            HttpResponse::Ok().json(json!({
                "joined_at": joined_at
            }))
        }
        Err(e) => {
            HttpResponse::InternalServerError().json(json!({
                "error": e.to_string()
            }))
        }
    }
}