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
use rocket::http::Status;
use rocket::Request;

//JSON
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