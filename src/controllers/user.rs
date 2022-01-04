use diesel::result::Error;
use rocket::http::Status;
use rocket::serde::json::Json;

use crate::connection::DbConn;
use crate::models::user::User as DbUser;
use crate::repository::user_repository::get_user_by_id;

/// Place is temporary.
fn error_status(error: Error) -> Status {
    match error {
        Error::NotFound => Status::NotFound,
        _ => Status::InternalServerError,
    }
}

#[get("/<id>", format = "json")]
pub fn get_user(id: i32, connection: DbConn) -> Result<Json<DbUser>, Status> {
    let user = get_user_by_id(id, &connection)
        .map(|user| Json(user))
        .map_err(|error| error_status(error));

    user
}
