use actix_web::{HttpResponse, get, web};
use serde_json::json;
use sqlx::types::Uuid;

use crate::AppData;


#[get("/leaderboard/{id}")]
pub async fn get_leaderboard(data: web::Data<AppData>, path: web::Path<Uuid>) -> HttpResponse {
    let contest_id = path.into_inner();
    let db = &data.db;

    let leaderboard_result = db.get_leaderboard(contest_id).await;
    if let Err(e) = leaderboard_result {
        return HttpResponse::InternalServerError().json(json!({
            "error": e.to_string()
        }));
    }

    return HttpResponse::Ok().json(leaderboard_result.unwrap());
}