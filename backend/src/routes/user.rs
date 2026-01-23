use std::env;

use actix_web::{HttpResponse, post, web::{self, Data}};
use bcrypt::{DEFAULT_COST, hash, verify};
use common::{CreateUserArgs, JwtClaims, SignInArgs};
use jsonwebtoken::{EncodingKey, Header, encode};
use serde_json::json;

use crate::AppData;

#[post("/signup")]
pub async fn create_user(app_data: Data<AppData>, mut body: web::Json<CreateUserArgs>) -> HttpResponse {
    let app_data= app_data.get_ref();
    let db = &app_data.db;

    let hash_res = hash(body.password.clone(), DEFAULT_COST);
    if let Err(e) = hash_res {
        return HttpResponse::InternalServerError().json(json!({
            "error": format!("Error while hashing: {}", e.to_string())
        }));
    }

    let hashed_pass = hash_res.unwrap();
    body.password = hashed_pass;

    let jwt_secret_result = env::var("JWT_SECRET");
    if jwt_secret_result.is_err() {
        return HttpResponse::InternalServerError().json(json!({
            "error": "JWT Secret is missing"
        }));
    }

    let create_user_res = db.create_user(body.0).await;
    if let Err(e) = create_user_res {
        return HttpResponse::InternalServerError().json(json!({
            "error": e.to_string()
        }));
    }
    let db_user = create_user_res.unwrap();

    let jwt_secret = jwt_secret_result.unwrap();
    let jwt_claims = JwtClaims::new(db_user.id, db_user.role, db_user.name);

    let token_res = encode(
        &Header::default(), 
        &jwt_claims,
        &EncodingKey::from_secret(&jwt_secret.as_bytes())
    );

    if let Err(e) = token_res {
        return HttpResponse::InternalServerError().json(json!({
            "error": format!("Error while constructing jwt: {}", e.to_string())
        }));
    }

    return HttpResponse::Ok().json(json!({
        "token": token_res.unwrap()
    }));
    
}

#[post("/signin")]
pub async fn signin(app_data: Data<AppData>, body: web::Json<SignInArgs>) -> HttpResponse {
    let app_data= app_data.get_ref();
    let db = &app_data.db;

    let jwt_secret_result = env::var("JWT_SECRET");
    if jwt_secret_result.is_err() {
        return HttpResponse::InternalServerError().json(json!({
            "error": "JWT Secret is missing"
        }));
    }

    let get_user_res = db.get_user_by_email(body.email.clone()).await;
    if let Err(e) = get_user_res {
        return HttpResponse::InternalServerError().json(json!({
            "error": e.to_string()
        }));
    }
    let db_user = get_user_res.unwrap();

    let verify_res = verify(body.password.clone(), &db_user.password);
    if let Err(e) = verify_res {
        return HttpResponse::InternalServerError().json(json!({
            "error": format!("Error while verifying hash: {}", e.to_string())
        }));
    }

    if !verify_res.unwrap() {
        return HttpResponse::Unauthorized().json(json!({
            "error": format!("Wrong password")
        }));
    }

    let jwt_secret = jwt_secret_result.unwrap();
    let jwt_claims = JwtClaims::new(db_user.id, db_user.role, db_user.name);

    let token_res = encode(
        &Header::default(), 
        &jwt_claims,
        &EncodingKey::from_secret(&jwt_secret.as_bytes())
    );

    if let Err(e) = token_res {
        return HttpResponse::InternalServerError().json(json!({
            "error": format!("Error while constructing jwt: {}", e.to_string())
        }));
    }

    return HttpResponse::Ok().json(json!({
        "token": token_res.unwrap()
    }));
    
}