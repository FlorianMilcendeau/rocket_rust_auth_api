use chrono::prelude::*;
use jsonwebtoken::{
    decode, encode, errors::Error, Algorithm, DecodingKey, EncodingKey, Header, Validation,
};
use regex::Regex;
use rocket::http::Status;
use rocket::serde::{json::json, Deserialize, Serialize};

use crate::models::user::User;
use crate::utils::http::ApiResponse;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i32,
    pub name: String,
    pub email: String,
    exp: i64,
}

impl Claims {
    pub fn new(sub: i32, name: &String, email: &String) -> Self {
        Self {
            sub,
            name: name.to_owned(),
            email: email.to_owned(),
            exp: Utc::now().timestamp() + 1000,
        }
    }
}

pub fn generate_json_web_token(claims: Claims) -> Result<String, Error> {
    let token = encode(
        &Header::new(Algorithm::RS256),
        &claims,
        &EncodingKey::from_rsa_pem(include_bytes!("../../id_rsa_priv.pem"))?,
    );

    return token;
}

pub fn decode_json_web_token(token: &str) -> Result<User, Error> {
    let claims = decode::<Claims>(
        token,
        &DecodingKey::from_rsa_pem(include_bytes!("../../id_rsa_pub.pem"))?,
        &Validation::new(Algorithm::RS256),
    );

    match claims {
        Ok(value) => Ok(User {
            id: value.claims.sub,
            name: value.claims.name,
            email: value.claims.email,
            password: "".to_string(),
        }),
        Err(error) => Err(error),
    }
}

pub fn extract_bearer_token(token: &str) -> Result<String, ApiResponse> {
    let reg = Regex::new(r"(Bearer)\s+(\S+)").unwrap();
    let captures = reg.captures(token).unwrap();

    if captures.len() == 3 {
        return Ok(String::from(&captures[2]));
    }

    return Err(ApiResponse::new(
        Status::Unauthorized,
        json!("You must provided Bearer token"),
    ));
}
