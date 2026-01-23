use chrono::{Days, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::Role;


#[derive(Serialize, Deserialize)]
pub struct JwtClaims {
    pub id: Uuid,
    pub role: Role,
    pub username: String,
    pub exp: i64
}

impl JwtClaims {
    pub fn new(user_id: Uuid, user_role: Role, username: String) -> JwtClaims {
        Self { 
            id: user_id, 
            role: user_role, 
            username: username, 
            exp:  Utc::now().checked_add_days(Days::new(10)).unwrap().timestamp_millis()
        }
    }
}