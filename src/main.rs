#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
extern crate dotenv;

use dotenv::dotenv;

mod connection;
use connection::init_pool;
mod routes;
use routes::authenticate::authenticate_routes;
use routes::user::user_routes;
mod controllers;
mod models;
mod repository;
mod schema;
mod utils;

#[launch]
fn rocket() -> _ {
    dotenv().ok();
    rocket::build()
        .manage(init_pool())
        .mount("/", authenticate_routes())
        .mount("/users", user_routes())
}
