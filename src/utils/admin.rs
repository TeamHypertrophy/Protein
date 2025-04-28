/*
______          _       _
| ___ \        | |     (_)
| |_/ / __ ___ | |_ ___ _ _ __
|  __/ '__/ _ \| __/ _ \ | '_ \
| |  | | | (_) | ||  __/ | | | |
\_|  |_|  \___/ \__\___|_|_| |_|

        Made with ❤️
*/
use std::sync::{Arc, LazyLock};

use uuid::Uuid;
use rocket::request::Request;

use crate::{
    db::DBConnection,
    errors::Error,
    models::keys::{APIKeyLog, NewAPIKeyLog},
};

pub type Config = Arc<Admin>;

// Struct Containing Admin Information
pub struct Admin {
    // Static Vector of Admin Routes
    pub routes: LazyLock<Vec<&'static str>>,
    // API Master Key
    pub master_key: String,
    // Application Environment (Development, Production, etc.)
    pub app_env: String,
    // Host URL For API
    pub host_url: String,
    // Host URL For User Avatars
    pub avatar_host_url: String,
    // Password Salt Used For Argon-2 Hashes
    pub password_salt: String,
    // SMTP Username Config
    pub smtp_user: String,
    // STMP User Config
    pub smtp_username: String,
}

#[derive(Clone)]
pub struct RequestInfo {
    pub method: String,
    pub route: String,
    pub ip_address: String,
    pub user_agent: String,
}

pub async fn generate_api_key_log(
    request: RequestInfo,
    mut connection: DBConnection,
    api_key: Uuid,
    user_id: Uuid,
    status_code: i32,
) -> Result<(), Error> {
    let api_key_log = NewAPIKeyLog {
        api_key: api_key,
        user_id: user_id,
        method: request.method,
        route: request.route,
        ip_address: request.ip_address,
        status_code: status_code,
        user_agent: request.user_agent,
    };

    APIKeyLog::create(api_key_log, &mut connection).await?;

    Ok(())
}

pub fn get_request_info(request: &Request<'_>) -> RequestInfo {
    let method = request.method().as_str();

    let route = request
        .route()
        .and_then(|route| route.name.as_deref())
        .unwrap_or("Unknown");

    let ip_address = request
        .client_ip()
        .map(|ip| ip.to_string())
        .unwrap_or("Unknown".to_string());

    let user_agent = request.headers().get_one("User-Agent").unwrap_or("Unknown");

    RequestInfo {
        method: method.to_string(),
        route: route.to_string(),
        ip_address: ip_address,
        user_agent: user_agent.to_string(),
    }
}
