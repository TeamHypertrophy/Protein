/*
______          _       _
| ___ \        | |     (_)
| |_/ / __ ___ | |_ ___ _ _ __
|  __/ '__/ _ \| __/ _ \ | '_ \
| |  | | | (_) | ||  __/ | | | |
\_|  |_|  \___/ \__\___|_|_| |_|

        Made with ❤️
*/
use std::sync::Arc;

use rocket::{
    http::Status,
    request::{self, FromRequest, Outcome, Request},
};
use uuid::Uuid;

use crate::{
    db,
    errors::Error,
    models::{
        keys::{APIKey, NewAPIKeyLog},
        user::Role,
    },
    utils::admin,
};

pub struct API;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for API {
    type Error = Error;

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
                        Error::Validation("Invalid API Key".to_string()),
                    ));
                }
            },
            None => {
                return Outcome::Error((
                    Status::Unauthorized,
                    Error::Authorization("No API Key Provided".to_string()),
                ));
            }
        };

        // 2. Get The Admin Data From The Request State
        let admin = match request.rocket().state::<Arc<admin::Admin>>() {
            Some(admin) => admin,
            None => {
                return Outcome::Error((
                    Status::InternalServerError,
                    Error::Internal("Failed Retrieving Admin Data".to_string()),
                ));
            }
        };

        // 3. Check If The API Key Is The Master Key
        if let Ok(master) = Uuid::parse_str(admin.master_key.as_str()) {
            if api_key == master {
                if admin.app_env == "development" {
                    return Outcome::Success(API);
                } else {
                    return Outcome::Error((
                        Status::Unauthorized,
                        Error::Authorization(
                            "Master API key can only be used in development environment"
                                .to_string(),
                        ),
                    ));
                }
            }
        }

        // 4. Get The Database Pool From The Request State
        let pool = match request.rocket().state::<db::DB>() {
            Some(pool) => pool,
            _ => {
                return Outcome::Error((
                    Status::InternalServerError,
                    Error::Database("Failed to Retrieve Database Pool".to_string()),
                ));
            }
        };

        // 5. Get A Database Connection From The Pool
        let mut connection = match pool.get().await {
            Ok(connection) => connection,
            Err(_) => {
                return Outcome::Error((
                    Status::InternalServerError,
                    Error::Database("Failed to Retrieve Database Connection".to_string()),
                ));
            }
        };

        // 6. Get The User ID From The Query Parameters
        // https://api.hypertrophy.app/v1/users?user_id=...
        let user_id = request
            .uri()
            .query()
            .and_then(|q| q.as_str().strip_prefix("user_id="))
            .and_then(|id| Uuid::parse_str(id).ok());

        // 7. Get Request Info For Logging
        let info = admin::get_request_info(request);

        // 8. If The User ID Exists, Verify:
        // That The Request That Is Being Performed Against The User Matches The API Key
        if let Some(uid) = user_id {
            match APIKey::verify(uid, api_key, connection, &info).await {
                Ok(_verified) => {
                    return Outcome::Success(API);
                }
                Err(error) => {
                    return Outcome::Error((
                        Status::Unauthorized,
                        Error::Database(error.to_string()),
                    ));
                }
            };
        }

        // 9. Get The Trainer ID From The Query Parameters
        let trainer_id = request
            .uri()
            .query()
            .and_then(|q| q.as_str().strip_prefix("trainer_id="))
            .and_then(|id| id.parse::<i32>().ok());

        // 10. Verify Trainer Action Against API Key
        if let Some(tid) = trainer_id {
            match APIKey::verify_trainer(tid, api_key, connection, &info).await {
                Ok(_verified) => {
                    return Outcome::Success(API);
                }
                Err(error) => {
                    return Outcome::Error((
                        Status::Unauthorized,
                        Error::Database(error.to_string()),
                    ));
                }
            };
        }

        // 11. This Is The Final Check
        // Checks:
        // 1. If The Key Exists In The Database == Next Step
        // 2. If The Key Is Admin/Developer == Access
        // 3. Admin Route && Admin/Developer Key == Access
        // 5. Normal API Access or Unauthorized
        match APIKey::find_by_key(api_key, &mut connection).await {
            Ok(key) => {
                let req = admin::get_request_info(request);

                if key.role == Role::Admin || key.role == Role::Developer {
                    match admin::generate_api_key_log(
                        &req,
                        connection,
                        key.api_key,
                        key.user_id,
                        200,
                    )
                    .await
                    {
                        Ok(_) => tracing::info!("[API] ✅ Created API Key Log"),
                        Err(e) => tracing::error!("[API] ❌ Failed Creating API Key Log: {}", e),
                    }

                    return Outcome::Success(API);
                }

                let route = match request.route() {
                    Some(route) => route,
                    None => {
                        return Outcome::Error((
                            Status::InternalServerError,
                            Error::Internal("Failed Retrieving Route".to_string()),
                        ));
                    }
                };

                let name = match route.name.as_deref() {
                    Some(name) => name,
                    None => {
                        return Outcome::Error((
                            Status::InternalServerError,
                            Error::Internal("Failed Retrieving Route Name".to_string()),
                        ));
                    }
                };

                // Admin Route Check
                if admin.routes.contains(&name) {
                    if key.role == Role::Admin || key.role == Role::Developer {
                        match admin::generate_api_key_log(
                            &req,
                            connection,
                            key.api_key,
                            key.user_id,
                            200,
                        )
                        .await
                        {
                            Ok(_) => tracing::info!("[API] ✅ Created API Key Log"),
                            Err(e) => {
                                tracing::error!("[API] ❌ Failed Creating API Key Log: {}", e)
                            }
                        }

                        return Outcome::Success(API);
                    } else {
                        return Outcome::Error((
                            Status::Unauthorized,
                            Error::Authorization("Unauthorized API Key!".to_string()),
                        ));
                    }
                }

                match admin::generate_api_key_log(&req, connection, key.api_key, key.user_id, 200)
                    .await
                {
                    Ok(_) => tracing::info!("[API] ✅ Created API Key Log"),
                    Err(e) => tracing::error!("[API] ❌ Failed Creating API Key Log: {}", e),
                }

                return Outcome::Success(API);
            }
            _ => {
                return Outcome::Error((
                    Status::Unauthorized,
                    Error::Authorization("Unauthorized API Key!".to_string()),
                ));
            }
        }
    }
}
