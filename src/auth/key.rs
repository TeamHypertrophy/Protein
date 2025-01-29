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

pub struct API;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for API {
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

        let query = match request.uri().query() {
            Some(query) => query,
            None => {
                return Outcome::Error((
                    Status::BadRequest,
                    ProteinError::Validation("Cannot Find Query!".to_string()),
                ))
            }
        };

        let value = match query.strip_prefix("user_id=") {
            Some(value) => value.as_str(),
            None => {
                return Outcome::Error((
                    Status::BadRequest,
                    ProteinError::Validation("Cannot Find User ID!".to_string()),
                ))
            }
        };

        let user_id = match Uuid::parse_str(value) {
            Ok(uuid) => uuid,
            Err(_) => {
                return Outcome::Error((
                    Status::BadRequest,
                    ProteinError::Validation("Invalid User ID".to_string()),
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

        match APIKey::verify(user_id, api_key, connection).await {
            Ok(_verified) => {
                return Outcome::Success(API);
            }
            Err(_) => {
                return Outcome::Error((
                    Status::Unauthorized,
                    ProteinError::Database("API Key Verification Failed".to_string()),
                ));
            }
        };
    }
}
