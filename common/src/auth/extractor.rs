use std::{env, future::{Ready, ready}};

use actix_web::{FromRequest, error::{ErrorInternalServerError, ErrorUnauthorized}, web::Query};
use chrono::Utc;
use jsonwebtoken::{DecodingKey, Validation, decode};
use serde::{Deserialize, Serialize};

use crate::JwtClaims;

#[derive(Serialize, Deserialize)]
pub struct WsQuery {
    pub token: String
}


impl FromRequest for JwtClaims {
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        let params_result = Query::<WsQuery>::from_query(req.query_string());
        let header_option = req.headers().get("Authorization");

        let mut token_option: Option<String> = None;

        if params_result.is_ok() {
            let param = params_result.unwrap();
            token_option = Some(param.token.clone());
        } else if header_option.is_some() {
            let header_val = header_option.unwrap().to_str();
            if header_val.is_ok() {
                token_option = Some(header_val.unwrap().to_string())
            }
        }

        if token_option.is_none() {
            return ready(Err(ErrorUnauthorized("JWT Token is missing")));
        }

        let jwt_secret_result = env::var("JWT_SECRET");
        if jwt_secret_result.is_err() {
            return ready(Err(ErrorInternalServerError("JWT secret is missing")));
        }

        let token = token_option.unwrap();
        let jwt_secret = jwt_secret_result.unwrap();

        let decoded = decode::<JwtClaims>(
            token, 
            &DecodingKey::from_secret(&jwt_secret.as_bytes()), 
            &Validation::default()
        );

        if decoded.is_err() {
            return ready(Err(ErrorUnauthorized("Invalid JWT token")));
        }

        let jwt_claim = decoded.unwrap();
        if jwt_claim.claims.exp < Utc::now().timestamp_millis() {
            return ready(Err(ErrorUnauthorized("JWT token has expired")));
        }

        return ready(Ok(jwt_claim.claims));
    }
}