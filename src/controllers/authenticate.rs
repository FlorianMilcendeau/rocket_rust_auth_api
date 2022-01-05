use rocket::serde::{json::Json, Deserialize, Serialize};

use crate::connection::DbConn;
use crate::models::user::{NewUser, User as DbUser};
use crate::repository::user_repository::create_user;
use crate::utils::password::generate_password;

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    name: String,
    email: String,
    password: String,
}

#[post("/signup", data = "<user>")]
pub fn signup(user: Json<UserSignup>, connection: DbConn) -> Result<Json<User>, ApiResponse> {
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

    Json(user_created)
}
