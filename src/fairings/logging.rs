// Rocket
use rocket::{Request, Data};
use rocket::fairing::{Fairing, Info, Kind};

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
        info!("Request Received, URI: {}", {request.uri()})
    }
}
