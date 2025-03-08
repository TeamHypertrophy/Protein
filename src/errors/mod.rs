/*
______          _       _
| ___ \        | |     (_)
| |_/ / __ ___ | |_ ___ _ _ __
|  __/ '__/ _ \| __/ _ \ | '_ \
| |  | | | (_) | ||  __/ | | | |
\_|  |_|  \___/ \__\___|_|_| |_|

        Made with ❤️
*/

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
pub enum Error {
    Internal(String),

    NotFound(String),

    BadRequest(String),

    Cache(String),

    Authorization(String),

    Validation(String),

    Database(String),

    Email(String),

    Webhook(String),

    IO(String),
}

impl Error {
    fn get_http_status(&self) -> Status {
        match self {
            Error::Internal(_) => Status::InternalServerError,
            Error::Cache(_) => Status::InternalServerError,
            Error::Database(_) => Status::InternalServerError,
            Error::Authorization(_) => Status::Unauthorized,
            Error::NotFound(_) => Status::NotFound,
            Error::Validation(_) => Status::BadRequest,
            Error::Email(_) => Status::InternalServerError,
            Error::Webhook(_) => Status::BadRequest,
            Error::IO(_) => Status::InternalServerError,
            _ => Status::BadRequest,
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(fmt, "Error {}.", self.get_http_status())
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        match self {
            Error::Internal(_) => "[!] Internal Server Error",
            Error::NotFound(_) => "[!] Data Not Found",
            Error::BadRequest(_) => "[!] Bad Request Formed",
            Error::Cache(_) => "[!] Cache Error",
            Error::Authorization(_) => "[!] Authorization Error",
            Error::Validation(_) => "[!] Validation Error",
            Error::Database(_) => "[!] Database Error",
            Error::Email(_) => "[!] Email Error",
            Error::Webhook(_) => "[!] Webhook Error",
            Error::IO(_) => "[!] IO Error",
        }
    }
}

impl<'r> Responder<'r, 'static> for Error {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
        let status_code: Status = self.get_http_status();

        let message = match self {
            Error::Internal(error) => error,
            Error::NotFound(error) => error,
            Error::BadRequest(error) => error,
            Error::Cache(error) => error,
            Error::Authorization(error) => error,
            Error::Validation(error) => error,
            Error::Database(error) => error,
            Error::Email(error) => error,
            Error::Webhook(error) => error,
            Error::IO(error) => error,
        };

        let response = json::to_string(&ErrorResponse {
            message,
            status_code,
        })
        .unwrap();

        Response::build()
            .status(status_code)
            .header(ContentType::JSON)
            .sized_body(response.len(), std::io::Cursor::new(response))
            .ok()
    }
}
