use actix_web::{HttpResponse, get, web};
use serde_json::json;
use sqlx::types::Uuid;

use crate::AppData;

#[get("/boilerplate/{id}")]
pub async fn get_all_boilerplate_codes(app_data: web::Data<AppData>, path: web::Path<Uuid>) -> HttpResponse {
    let question_id = path.into_inner();
    let db = &app_data.db;

    match db.get_all_boilerplate_codes(question_id).await {
        Ok(codes) => {
            HttpResponse::Ok().json(json!(codes))
        }
        Err(e) => {
            HttpResponse::InternalServerError().json(json!({
                "error": e.to_string()
            }))
        }
    }
}