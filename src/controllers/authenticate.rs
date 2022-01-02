use rocket::serde::{json::Json, Deserialize, Serialize};

use crate::connection::DbConn;
use crate::models::user::{NewUser, User as DbUser};
use crate::repository::user_repository::create_user;
use crate::utils::password::generate_password;

#[derive(Serialize, Deserialize, Debug)]
pub struct User<'r> {
    email: &'r str,
    password: String,
}

#[post("/signup", data = "<user>")]
pub fn signup(user: Json<User<'_>>, connection: DbConn) -> Json<DbUser> {
    let password = generate_password(user.password.clone());
    let new_user = NewUser {
        email: user.email.to_string(),
        password,
    };
    let user_created = create_user(new_user, &connection).unwrap();

    Json(user_created)
}
