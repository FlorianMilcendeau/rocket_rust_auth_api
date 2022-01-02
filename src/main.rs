#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
extern crate dotenv;


use dotenv::dotenv;
use rocket::serde::{json::Json, Deserialize, Serialize};

mod connection;
use connection::{init_pool, DbConn};
mod repository;
use repository::user_repository::create_user;
mod models;
use models::user::{NewUser, User as DbUser};
mod routes;
use routes::user::user_routes;
mod utils;
use utils::password::generate_password;
mod schema;
mod controllers;

#[get("/")]
fn index() -> &'static str {
    "hello world"
}

#[derive(Serialize, Deserialize, Debug)]
struct User<'r> {
    email: &'r str,
    password: String,
}

#[post("/signup", data = "<user>")]
fn signup(user: Json<User<'_>>, connection: DbConn) -> Json<DbUser> {
    let password = generate_password(user.password.clone());
    let new_user = NewUser {
        email: user.email.to_string(),
        password,
    };
    let user_created = create_user(new_user, &connection).unwrap();

    Json(user_created)
}

#[launch]
fn rocket() -> _ {
    dotenv().ok();
    rocket::build()
        .manage(init_pool())
        .mount("/", routes![index, signup])
        .mount("/users", user_routes())
}
