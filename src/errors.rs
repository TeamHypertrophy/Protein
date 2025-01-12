/*
______          _       _
| ___ \        | |     (_)
| |_/ / __ ___ | |_ ___ _ _ __
|  __/ '__/ _ \| __/ _ \ | '_ \
| |  | | | (_) | ||  __/ | | | |
\_|  |_|  \___/ \__\___|_|_| |_|

        Made with ❤️
*/

use rocket::{http::Status, Request};

use rocket::serde::json::{json, Value};

// Default Catcher
#[catch(default)]
pub fn default(status: Status, request: &Request) -> Value {
    json!(
        {
            "status": status.code,
            "path": request.uri(),
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
            "message": "[!!] There was an Internal Server Issue!"
        }
    )
}
