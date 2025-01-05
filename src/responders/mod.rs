/*
______          _       _
| ___ \        | |     (_)
| |_/ / __ ___ | |_ ___ _ _ __
|  __/ '__/ _ \| __/ _ \ | '_ \
| |  | | | (_) | ||  __/ | | | |
\_|  |_|  \___/ \__\___|_|_| |_|

        Made with ❤️
*/

// Rocket
use rocket::response::{self, Response, Responder};
use rocket::http::ContentType;
use rocket::request::Request;
use rocket::http::Status;
use rocket::serde::json;

// JSON
use serde::Serialize;
use std::io::Cursor;

#[derive(Serialize)]
pub struct ErrorResponse {
    pub message: String,
    pub status_code: Status,
}

#[derive(Debug, Clone)]
pub enum ProteinError {
    Internal(String),

    NotFound(String),

    BadRequest(String),

    Cache(String),

    Authorization(String),

    Validation(String),

    Database(String),
}

impl ProteinError {
    fn get_http_status(&self) -> Status {
        match self {
            ProteinError::Internal(_) => Status::InternalServerError,
            ProteinError::Cache(_) => Status::InternalServerError,
            ProteinError::Database(_) => Status::InternalServerError,
            ProteinError::Authorization(_) => Status::Unauthorized,
            ProteinError::NotFound(_) => Status::NotFound,
            ProteinError::Validation(_) => Status::BadRequest,
            _ => Status::BadRequest,
        }
    }
}

impl std::fmt::Display for ProteinError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(fmt, "Error {}.", self.get_http_status())
    }
}

impl<'r> Responder<'r, 'static> for ProteinError {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
        let status = self.get_http_status();

        let message = match self {
            ProteinError::Internal(msg) => msg,
            ProteinError::NotFound(msg) => msg,
            ProteinError::BadRequest(msg) => msg,
            ProteinError::Cache(msg) => msg,
            ProteinError::Authorization(msg) => msg,
            ProteinError::Validation(msg) => msg,
            ProteinError::Database(msg) => msg,
        };

        let error_response = json::to_string(&ErrorResponse {
            message: message,
            status_code: status,
        })
        .unwrap();

        Response::build()
            .status(status)
            .header(ContentType::JSON)
            .sized_body(error_response.len(), Cursor::new(error_response))
            .ok()
    }
}
