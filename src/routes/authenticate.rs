use rocket;

use crate::controllers::authenticate::{signin, signup};

pub fn authenticate_routes() -> Vec<rocket::Route> {
    routes![signup, signin]
}
