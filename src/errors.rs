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

// Default Catcher
#[catch(default)]
pub fn default(status: Status, request: &Request) -> Value {
    json!(
        {
            "status": status.code,
            "path": request.uri(),
            "method": request.method(),
            "message": "[!!] Error in Protein Service"
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
            "message": "[!!] Requested Path was Not Found"
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
            "message": "[!!] There was an Internal Server Issue!"
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
            "message": "[!!] Unprocessable Entity",
            "notes": "The request was well-formed but was unable to be followed due to semantic/parsing errors."
        }
    )
}
