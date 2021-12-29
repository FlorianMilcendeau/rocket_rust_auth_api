#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
extern crate dotenv;


use dotenv::dotenv;
use rocket::serde::{json::Json, Deserialize, Serialize};

mod connection;
use connection::init_pool;

mod utils;
use utils::password::generate_password;

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
fn signin(user: Json<User<'_>>) -> Json<User<'_>> {
    let password = generate_password(user.password.clone());
    Json(User {
        email: user.email,
        password: password,
    })
}

#[launch]
fn rocket() -> _ {
    dotenv().ok();
    rocket::build()
        .manage(init_pool())
        .mount("/api", routes![index, signin])
}
