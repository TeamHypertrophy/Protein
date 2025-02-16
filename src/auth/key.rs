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

use crate::{
    constants::MASTER_API_KEY,
    db,
    models::{keys::APIKey, user::Role},
    responders::ProteinError,
};

pub struct API;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for API {
    type Error = ProteinError;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        // The Second Form of Authentication for Hypertrophy Users

        // 1. Get The API Key From The Request Headers
        // This Will Not Be Set By The User, But Instead
        // The Mobile App Will Set This Header Automatically
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

        // 2. Get The Database Pool From The Request State
        let pool = match request.rocket().state::<db::DatabasePool>() {
            Some(pool) => pool,
            _ => {
                return Outcome::Error((
                    Status::InternalServerError,
                    ProteinError::Database("Failed to Retrieve Database Pool".to_string()),
                ))
            }
        };

        // 3. Get A Database Connection From The Pool
        let connection = match pool.get().await {
            Ok(connection) => connection,
            Err(_) => {
                return Outcome::Error((
                    Status::InternalServerError,
                    ProteinError::Database("Failed to Retrieve Database Connection".to_string()),
                ))
            }
        };

        // 4. Get The User ID From The Query Parameters
        // https://api.hypertrophy.app/v1/users?user_id=...
        let user_id = request
            .uri()
            .query()
            .and_then(|q| q.as_str().strip_prefix("user_id="))
            .and_then(|id| Uuid::parse_str(id).ok());

        // 5. If The User ID Exists, Verify:
        // That The Request That Is Being Performed Against The User Matches The API Key
        if let Some(uid) = user_id {
            match APIKey::verify(uid, api_key, connection).await {
                Ok(_verified) => {
                    return Outcome::Success(API);
                }
                Err(error) => {
                    return Outcome::Error((
                        Status::Unauthorized,
                        ProteinError::Database(error.to_string()),
                    ));
                }
            };
        }

        // 6. Retrieve The Master API Key From The Constants File
        let master = match Uuid::parse_str(MASTER_API_KEY) {
            Ok(master) => master,
            Err(_) => {
                return Outcome::Error((
                    Status::InternalServerError,
                    ProteinError::Internal("Failed to Parse Master API Key, Please Set A Correct UUID Value In `constants.rs`".to_string()),
                ))
            }
        };

        // 7. Now, we check if the API Key is a Developer Key or the Master Key
        if let Ok(key) = APIKey::find_by_key(api_key, connection).await {
            if key.role == Role::Admin || key.role == Role::Developer {
                return Outcome::Success(API);
            }

            // check if request.route not in list of routes that require admin/dev key
            return Outcome::Success(API);
        } else if api_key == master && std::env::var("APP_ENV").unwrap() == "development" {
            return Outcome::Success(API);
        } else {
            return Outcome::Error((
                Status::Unauthorized,
                ProteinError::Authorization("Unauthorized API Key!".to_string()),
            ));
        }
    }
}
