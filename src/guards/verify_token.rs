use jsonwebtoken::errors;
use rocket::http::Status;
use rocket::request::{self, FromRequest, Request};
use rocket::serde::json::json;
use std::fmt::Display;

use crate::utils::http::ApiResponse;
use crate::utils::jsonwebtoken::{decode_json_web_token, extract_bearer_token};

pub struct JWTCredential {
    pub id: i32,
    pub name: String,
    pub email: String,
}

#[derive(Debug)]
pub enum GuardError {
    ApiError(ApiResponse),
    JWTError(errors::Error),
}

impl Display for GuardError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GuardError::ApiError(api_error) => write!(f, "{:?}", api_error),
            GuardError::JWTError(jwt_error) => write!(f, "{}", jwt_error),
        }
    }
}

impl std::error::Error for GuardError {}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for JWTCredential {
    type Error = GuardError;

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<JWTCredential, Self::Error> {
        match req.headers().get_one("Authorization") {
            Some(authorization) => {
                let token = extract_bearer_token(authorization);

                match token {
                    Ok(value) => match decode_json_web_token(&value.as_str()) {
                        Ok(user) => request::Outcome::Success(user),
                        Err(e) => request::Outcome::Failure((
                            Status::Unauthorized,
                            GuardError::JWTError(e),
                        )),
                    },
                    Err(error) => request::Outcome::Failure((
                        Status::Unauthorized,
                        GuardError::ApiError(error),
                    )),
                }
            }
            None => request::Outcome::Failure((
                Status::Unauthorized,
                GuardError::ApiError(ApiResponse::new(
                    Status::Unauthorized,
                    json!({ "message": "Need a token"}),
                )),
            )),
        }
    }
}
