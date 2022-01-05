use rocket::http::ContentType;
use rocket::http::Status;
use rocket::request::Request;
use rocket::response::{Responder, Response};
use rocket::serde::json::Value;

#[derive(Debug)]
pub struct ApiResponse {
    pub status: Status,
    pub body: Value,
}

impl ApiResponse {
    pub fn new(status: Status, body: Value) -> Self {
        Self { status, body }
    }
}

impl<'r> Responder<'r, 'static> for ApiResponse {
    fn respond_to(self, req: &'r Request<'_>) -> Result<Response<'static>, Status> {
        Response::build_from(self.body.respond_to(req).unwrap())
            .status(self.status)
            .header(ContentType::JSON)
            .ok()
    }
}
