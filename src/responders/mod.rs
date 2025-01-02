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
}

#[derive(Debug, Clone)]
pub enum ProteinError {
    Internal(String),

    NotFound(String),

    BadRequest(String),

    Cache(String),

    Database(String),
}

impl ProteinError {
    fn get_http_status(&self) -> Status {
        match self {
            ProteinError::Internal(_) => Status::InternalServerError,
            ProteinError::Cache(_) => Status::InternalServerError,
            ProteinError::Database(_) => Status::InternalServerError,
            ProteinError::NotFound(_) => Status::NotFound,
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
        let error_response = json::to_string(&ErrorResponse{
            message: self.to_string(),
        }).unwrap();

        Response::build()
            .status(self.get_http_status())
            .header(ContentType::JSON)
            .sized_body(error_response.len(), Cursor::new(error_response))
            .ok()
    }
}