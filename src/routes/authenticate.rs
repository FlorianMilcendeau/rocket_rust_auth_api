use rocket;

use crate::controllers::authenticate::signup;

pub fn authenticate_routes() -> Vec<rocket::Route> {
    routes![signup]
}
