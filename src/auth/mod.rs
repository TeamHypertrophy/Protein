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
    request::{self, FromRequest, Outcome, Request},
};
use uuid::Uuid;
use crate::{db, models::keys::APIKey, responders::ProteinError};

pub struct Auth;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Auth {
    type Error = ProteinError;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let api_key = match request.headers().get_one("API_KEY") {
            Some(key) => match Uuid::parse_str(key) {
                Ok(uuid) => uuid,
                Err(_) => {
                    return Outcome::Error((
                        Status::BadRequest,
                        ProteinError::Validation("Invalid API Key".to_string()),
                    ))
                }
            },
            None => {
                return Outcome::Error((
                    Status::Unauthorized,
                    ProteinError::Authorization("No API Key Provided".to_string()),
                ))
            }
        };

        let pool = match request.rocket().state::<db::DatabasePool>() {
            Some(pool) => pool,
            _ => {
                return Outcome::Error((
                    Status::InternalServerError,
                    ProteinError::Database("Failed to Retrieve Database Pool".to_string()),
                ))
            }
        };

        let connection = match pool.get().await {
            Ok(connection) => connection,
            Err(_) => {
                return Outcome::Error((
                    Status::InternalServerError,
                    ProteinError::Database("Failed to Retrieve Database Connection".to_string()),
                ))
            }
        };

        if let Ok(key) = APIKey::find_by_key(api_key, connection).await {
            if key.api_key.to_string() == api_key.to_string() {
                return Outcome::Success(Auth);
            } else {
                Outcome::Error((
                    Status::Unauthorized,
                    ProteinError::Authorization("API Key Does Not Match!".to_string()),
                ))
            }
        } else {
            return Outcome::Error((
                Status::Unauthorized,
                ProteinError::Authorization("API Key Does Not Match!".to_string()),
            ));
        }
    }
}
