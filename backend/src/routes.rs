use actix_web::{HttpResponse, post, web::{self, Data}};
use common::CreateUserArgs;
use serde_json::json;

use crate::AppData;

#[post("/signup")]
pub async fn create_user(app_data: Data<AppData>, args: web::Json<CreateUserArgs>) -> HttpResponse {
    let app_data= app_data.get_ref();
    let db = &app_data.db;

    match db.create_user(args.0).await {
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