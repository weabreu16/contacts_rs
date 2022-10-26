use std::io::Cursor;

use thiserror::Error;
use rocket::http::{Status, ContentType};
use rocket::request::Request;
use rocket::response::{Response, Responder, Result};
use rocket::{serde::{Serialize, json}};

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ErrorResponse {
    message: String
}

#[derive(Error, Debug, Clone)]
pub enum ApiError {
    #[error("{0}")]
    BadRequest(String),

    #[error("{0}")]
    NotFound(String),

    #[error("An error occurred with the server")]
    InternalServer
}

impl ApiError {
    fn get_http_status(&self) -> Status {
        match self {
            ApiError::BadRequest(_) => Status::BadRequest,
            ApiError::NotFound(_) => Status::NotFound,
            _ => Status::InternalServerError
        }
    }
}

impl<'r> Responder<'r, 'static> for ApiError {
    fn respond_to(self, _: &'r Request<'_>) -> Result<'static> {
        let error_response = json::to_string(&ErrorResponse {
            message: self.to_string()
        }).unwrap();

        Response::build()
            .status(self.get_http_status())
            .header(ContentType::JSON)
            .sized_body(error_response.len(), Cursor::new(error_response))
            .ok()
    }
}
