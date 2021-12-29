#[macro_use]
extern crate rocket;
extern crate serde;

use rocket::serde::{json::Json, Deserialize, Serialize};

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
    Json(
        User {
            email: user.email,
            password: password
        }
    )
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/api", routes![index, signin])
}
