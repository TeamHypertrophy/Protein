/*
______          _       _
| ___ \        | |     (_)
| |_/ / __ ___ | |_ ___ _ _ __
|  __/ '__/ _ \| __/ _ \ | '_ \
| |  | | | (_) | ||  __/ | | | |
\_|  |_|  \___/ \__\___|_|_| |_|

        Made with ❤️
*/

use std::io::Cursor;

use rocket::{
    http::{ContentType, Status},
    request::Request,
    response::{self, Responder, Response},
    serde::json,
};
use serde::Serialize;

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

    Email(String),

    Webhook(String),
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
            ProteinError::Email(_) => Status::InternalServerError,
            ProteinError::Webhook(_) => Status::BadRequest,
            _ => Status::BadRequest,
        }
    }
}

impl std::fmt::Display for ProteinError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(fmt, "Error {}.", self.get_http_status())
    }
}

impl std::error::Error for ProteinError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        match self {
            ProteinError::Internal(_) => "[!] Internal Server Error",
            ProteinError::NotFound(_) => "[!] Data Not Found",
            ProteinError::BadRequest(_) => "[!] Bad Request Formed",
            ProteinError::Cache(_) => "[!] Cache Error",
            ProteinError::Authorization(_) => "[!] Authorization Error",
            ProteinError::Validation(_) => "[!] Validation Error",
            ProteinError::Database(_) => "[!] Database Error",
            ProteinError::Email(_) => "[!] Email Error",
            ProteinError::Webhook(_) => "[!] Webhook Error",
        }
    }
}

impl<'r> Responder<'r, 'static> for ProteinError {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
        let status_code: Status = self.get_http_status();

        let message = match self {
            ProteinError::Internal(error) => error,
            ProteinError::NotFound(error) => error,
            ProteinError::BadRequest(error) => error,
            ProteinError::Cache(error) => error,
            ProteinError::Authorization(error) => error,
            ProteinError::Validation(error) => error,
            ProteinError::Database(error) => error,
            ProteinError::Email(error) => error,
            ProteinError::Webhook(error) => error,
        };

        let response = json::to_string(&ErrorResponse {
            message,
            status_code,
        })
        .unwrap();

        Response::build()
            .status(status_code)
            .header(ContentType::JSON)
            .sized_body(response.len(), Cursor::new(response))
            .ok()
    }
}
