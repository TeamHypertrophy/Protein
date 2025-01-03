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
use rocket::fairing::{Fairing, Info, Kind};
use rocket::request::Outcome;
use rocket::http::Status;
use rocket::{Request, Data};

// Tracing
use tracing::warn;

// Errors
use crate::responders::ProteinError;
use crate::db::DatabasePool;
use crate::cache::redis::RedisPool;

pub struct Auth;

#[rocket::async_trait]
impl Fairing for Auth {
    fn info(&self) -> Info {
        Info {
            name: "[!] API Token Authentication",
            kind: Kind::Request,
        }
    }

    async fn on_request(&self, request: &mut Request<'_>, _data: &mut Data<'_>) -> Outcome<Self, Self::Error> {
        warn!("Beginning Auth Validation For URI: {}", { request.uri() });

        let api_key = match request.headers().get_one("API_KEY") {
            Some(key) => key,
            None => {
                warn!("No API Key Provided");
                return Outcome::Error((Status::Unauthorized, ()))
            }
        };

        let redis = request.rocket().state::<RedisPool>().unwrap();
        let pool = request.rocket().state::<DatabasePool>().unwrap();

        
    }
}
