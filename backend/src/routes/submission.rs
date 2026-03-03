use actix_web::{HttpResponse, get, web};
use common::{GetSubmissionParams, JwtClaims};
use serde_json::json;

use crate::AppData;

#[get("/submission")]
pub async fn get_submission(data: web::Data<AppData>, params: web::Query<GetSubmissionParams>, jwt_claims: JwtClaims) -> HttpResponse {
    let db = &data.db;
    
    let submission_result = db.get_submission(params.contest_id, params.question_id, jwt_claims.id).await;

    if let Err(e) = submission_result {
        return HttpResponse::InternalServerError().json(json!({
            "error": e.to_string()
        }))
    }
    
    let submission_option = submission_result.unwrap();

    let is_found = submission_option.is_some();

    return HttpResponse::Ok().json(json!({
        "found": is_found,
        "submission": submission_option
    }));
}