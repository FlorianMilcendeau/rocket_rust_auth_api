use rocket;

use crate::controllers::user::{get_user};

pub fn user_routes() -> Vec<rocket::Route> {
    routes![get_user]
}
