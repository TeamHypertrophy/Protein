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
    http::Status,
    serde::json::{json, Value},
    Request,
};

use crate::constants::*;

// Default Catcher
#[catch(default)]
pub fn default(status: Status, request: &Request) -> Value {
    json!(
        {
            "status": status.code,
            "path": request.uri(),
            "method": request.method(),
            "message": DEFAULT_ERROR_MESSAGE
        }
    )
}

// 404 - Not Found
#[catch(404)]
pub fn not_found(status: Status, request: &Request) -> Value {
    json!(
        {
            "status": status.code,
            "path": request.uri(),
            "method": request.method(),
            "message": NOT_FOUND_ERROR_MESSAGE
        }
    )
}

// 401 - Unauthorized
#[catch(401)]
pub fn unauthorized(status: Status, request: &Request) -> Value {
    json!(
        {
            "status": status.code,
            "path": request.uri(),
            "method": request.method(),
            "message": UNAUTHORIZED_ERROR_MESSAGE
        }
    )
}

// 500 - Internal Server Error
#[catch(500)]
pub fn internal_server_error(status: Status, request: &Request) -> Value {
    json!(
        {
            "status": status.code,
            "path": request.uri(),
            "method": request.method(),
            "message": INTERNAL_SERVER_ERROR_MESSAGE
        }
    )
}

// 422 - Unprocessable Entity
#[catch(422)]
pub fn unprocessable_entity(status: Status, request: &Request) -> Value {
    json!(
        {
            "status": status.code,
            "path": request.uri(),
            "method": request.method(),
            "message": UNPROCESSABLE_ENTITY_MESSAGE,
            "notes": UNPROCESSABLE_ENTITY_NOTE
        }
    )
}
