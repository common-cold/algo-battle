use actix_web::{HttpResponse, post, web::{self, Data}};
use common::{CreateContestArgs, GetContestArgs};
use page_hunter::paginate_records;
use serde_json::json;

use crate::AppData;

#[post("/contest/create")]
pub async fn create_contest(app_data: Data<AppData>, body: web::Json<CreateContestArgs>) -> HttpResponse {
    let app_data= app_data.get_ref();
    let db = &app_data.db;

    match db.create_contest(body.0).await {
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

#[post("/contest/all/examiner")]
pub async fn get_all_examiner_contests(app_data: Data<AppData>, body: web::Json<GetContestArgs>) -> HttpResponse {
    let app_data= app_data.get_ref();
    let db = &app_data.db;

    match db.get_all_examiner_contests(body.0.clone()).await {
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