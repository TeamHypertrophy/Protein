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

use crate::{constants::MASTER_API_KEY, db, models::keys::APIKey, responders::ProteinError};

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

        let user_id = request
            .uri()
            .query()
            .and_then(|q| q.as_str().strip_prefix("user_id="))
            .and_then(|id| Uuid::parse_str(id).ok());

        if let Some(uid) = user_id {
            match APIKey::verify(uid, api_key, connection).await {
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

        let master = match Uuid::parse_str(MASTER_API_KEY) {
            Ok(master) => master,
            Err(_) => {
                return Outcome::Error((
                    Status::InternalServerError,
                    ProteinError::Database("Failed to Parse Master API Key, Please Set A Correct UUID Value In `constants.rs`".to_string()),
                ))
            }
        };

        if let Ok(key) = APIKey::find_by_key(api_key, connection).await {
            if key.is_developer_key {
                return Outcome::Success(API);
            } else {
                Outcome::Error((
                    Status::Unauthorized,
                    ProteinError::Authorization("API Key Does Not Match!".to_string()),
                ))
            }
        } else if api_key == master {
            return Outcome::Success(API);
        } else {
            return Outcome::Error((
                Status::Unauthorized,
                ProteinError::Authorization("Unauthorized API Key!".to_string()),
            ));
        }
    }
}
