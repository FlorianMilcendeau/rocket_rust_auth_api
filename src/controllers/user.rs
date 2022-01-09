use diesel::result::Error;
use rocket::http::Status;
use rocket::serde::json::Json;

use crate::connection::DbConn;
use crate::guards::verify_token::JWTCredential;
use crate::models::user::User;
use crate::repository::user_repository::get_user_by_id;

/// Place is temporary.
fn error_status(error: Error) -> Status {
    match error {
        Error::NotFound => Status::NotFound,
        _ => Status::InternalServerError,
    }
}

#[get("/<id>", format = "json")]
pub fn get_user(_user: JWTCredential, id: i32, connection: DbConn) -> Result<Json<User>, Status> {
    let user_db = get_user_by_id(id, &connection)
        .map(|user_db| Json(user_db))
        .map_err(|error| error_status(error));

    user_db
}
