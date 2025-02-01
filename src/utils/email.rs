/*
______          _       _
| ___ \        | |     (_)
| |_/ / __ ___ | |_ ___ _ _ __
|  __/ '__/ _ \| __/ _ \ | '_ \
| |  | | | (_) | ||  __/ | | | |
\_|  |_|  \___/ \__\___|_|_| |_|

    Made with ❤️
*/

// Build Email System and Functionality

use std::env;

use rocket::State;
use dotenvy::dotenv;
use lettre::{
    message::Mailbox,
    transport::smtp::{authentication::Credentials, response::Response},
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
};

use crate::{models::profile::Profile, responders::ProteinError};

pub type Mailer = AsyncSmtpTransport<Tokio1Executor>;

pub async fn setup_email() -> Result<AsyncSmtpTransport<Tokio1Executor>, Box<dyn std::error::Error>>
{
    // Load Environment Variables
    dotenv().ok();

    // Get SMTP Credentials
    let username: String =
        env::var("SMTP_USERNAME").expect("[!] SMTP_USERNAME Environment Variable Must Be Set");
    let password: String =
        env::var("SMTP_PASSWORD").expect("[!] SMTP_PASSWORD Environment Variable Must Be Set");
    let server: String =
        env::var("SMTP_SERVER").expect("[!] SMTP_SERVER Environment Variable Must Be Set");

    // Create Credentials
    let credentials: Credentials = Credentials::new(username, password);

    // Create Mailer
    let mailer: Mailer = AsyncSmtpTransport::<Tokio1Executor>::relay(server.as_str())
        .unwrap()
        .credentials(credentials)
        .build();

    // Mailer
    Ok(mailer)
}

pub async fn send_email(
    mailer: &State<Mailer>,
    profile: &Profile,
    subject: &str,
    body: String,
) -> Result<Response, ProteinError> {
    // Load Environment Variables
    dotenv().ok();

    // Get SMTP Credentials
    let user: String =
        env::var("SMTP_USER").expect("[!] SMTP_USER Environment Variable Must Be Set");
    let username: String =
        env::var("SMTP_USERNAME").expect("[!] SMTP_USERNAME Environment Variable Must Be Set");

    // Get Full Name
    let full_name: String = format!("{} {}", profile.first_name, profile.last_name);

    // Create Mailboxes
    let from: Mailbox = format!("{} <{}>", user, username)
        .parse::<Mailbox>()
        .map_err(|error| ProteinError::Email(format!("Error Parsing From Address: {:?}", error)))?;

    let to: Mailbox = format!("{} <{}>", full_name, profile.email)
        .parse::<Mailbox>()
        .map_err(|error| ProteinError::Email(format!("Error Parsing To Address: {:?}", error)))?;

    // Create Email
    let email: Message = Message::builder()
        .to(to)
        .from(from)
        .subject(subject)
        .body(body)
        .map_err(|error| ProteinError::Email(format!("Error Building Email: {:?}", error)))
        .unwrap();

    // Send Email
    mailer
        .send(email)
        .await
        .map_err(|error| ProteinError::Email(format!("Error Sending Email: {:?}", error)))
}
