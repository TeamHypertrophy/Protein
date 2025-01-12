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
    fairing::{Fairing, Info, Kind},
    Data, Request,
};
// Tracing
use tracing::info;

pub struct Logging;

#[rocket::async_trait]
impl Fairing for Logging {
    fn info(&self) -> Info {
        Info {
            name: "[!] Request Logging",
            kind: Kind::Request,
        }
    }

    async fn on_request(&self, request: &mut Request<'_>, _data: &mut Data<'_>) {
        info!("Request Received, URI: {}", { request.uri() })
    }
}
