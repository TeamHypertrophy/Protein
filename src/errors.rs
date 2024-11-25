// Rocket
use rocket::Request;
use rocket::http::Status;

//JSON
use rocket::serde::json::{Value, json};


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