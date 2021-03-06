use rocket::http::Status;
use rocket::serde::{json::json, json::Json, Deserialize, Serialize};

use crate::connection::DbConn;
use crate::models::user::{NewUser, User};
use crate::repository::user_repository::{create_user, get_user_by_email};
use crate::utils::http::ApiResponse;
use crate::utils::jsonwebtoken::{generate_json_web_token, Claims};
use crate::utils::password::{generate_password, verify_password};

#[derive(Serialize, Deserialize)]
pub struct UserSignIn {
    email: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserSignup {
    name: String,
    email: String,
    password: String,
}

#[derive(Serialize)]
pub struct AuthResponse {
    user: User,
    token: String,
}

#[post("/signup", data = "<user>")]
pub fn signup(
    user: Json<UserSignup>,
    connection: DbConn,
) -> Result<Json<AuthResponse>, ApiResponse> {
    let current_user = get_user_by_email(&user.email, &connection).map(|u| Json(u));

    println!("User found: {:?}", current_user);

    match current_user {
        Ok(_) => Err(ApiResponse::new(
            Status::Unauthorized,
            json!({ "message": "Account already exist"}),
        )),
        Err(_) => {
            let password = generate_password(user.password.clone());
            let new_user = NewUser {
                name: user.name.clone(),
                email: user.email.clone(),
                password,
            };

            let user_created = create_user(new_user, &connection).unwrap();
            let token = generate_json_web_token(Claims::new(
                user_created.id,
                &user_created.name,
                &user_created.email,
            ))
            .unwrap();

            return Ok(Json(AuthResponse {
                user: user_created,
                token,
            }));
        }
    }
}

#[post("/signin", data = "<user>")]
pub fn signin(
    user: Json<UserSignIn>,
    connection: DbConn,
) -> Result<Json<AuthResponse>, ApiResponse> {
    let current_user = get_user_by_email(&user.email, &connection).map_err(|_| {
        ApiResponse::new(
            Status::Unauthorized,
            json!({ "message": "Account does not exist"}),
        )
    });

    match current_user {
        Ok(value) => {
            let hash_valid =
                verify_password(&value.password.as_str(), &user.password.as_bytes()).unwrap();

            if !hash_valid {
                return Err(ApiResponse::new(
                    Status::Unauthorized,
                    json!({ "message": "Password incorrect" }),
                ));
            }

            let token =
                generate_json_web_token(Claims::new(value.id, &value.name, &value.email)).unwrap();
            return Ok(Json(AuthResponse { user: value, token }));
        }
        Err(error) => Err(error),
    }
}
