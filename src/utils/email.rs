/*
______          _       _
| ___ \        | |     (_)
| |_/ / __ ___ | |_ ___ _ _ __
|  __/ '__/ _ \| __/ _ \ | '_ \
| |  | | | (_) | ||  __/ | | | |
\_|  |_|  \___/ \__\___|_|_| |_|

    Made with ❤️
*/

use rocket::State;
use lettre::{
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
    message::Mailbox,
    transport::smtp::{authentication::Credentials, response::Response},
};

use crate::{errors::ProteinError, models::user::User};

pub type Mailer = AsyncSmtpTransport<Tokio1Executor>;

pub async fn setup_email() -> Result<AsyncSmtpTransport<Tokio1Executor>, Box<dyn std::error::Error>>
{
    // Get SMTP Credentials
    let username: String =
        std::env::var("SMTP_USERNAME").expect("[!] SMTP_USERNAME Environment Variable Must Be Set");
    let password: String =
        std::env::var("SMTP_PASSWORD").expect("[!] SMTP_PASSWORD Environment Variable Must Be Set");
    let server: String =
        std::env::var("SMTP_SERVER").expect("[!] SMTP_SERVER Environment Variable Must Be Set");

    // Create Credentials
    let credentials: Credentials = Credentials::new(username, password);

    // Create Mailer
    let mailer: Mailer = match AsyncSmtpTransport::<Tokio1Executor>::relay(server.as_str()) {
        Ok(transport) => transport.credentials(credentials).build(),
        Err(error) => panic!("[!] Failed To Create Mailer: {:?}", error),
    };

    // Mailer
    Ok(mailer)
}

pub async fn send_email(
    mailer: &State<Mailer>,
    data: &User,
    subject: &str,
    body: String,
) -> Result<Response, ProteinError> {
    // Get SMTP Credentials
    let user: String =
        std::env::var("SMTP_USER").expect("[!] SMTP_USER Environment Variable Must Be Set");
    let username: String =
        std::env::var("SMTP_USERNAME").expect("[!] SMTP_USERNAME Environment Variable Must Be Set");

    // System Mailbox
    let from: Mailbox = format!("{} <{}>", user, username)
        .parse::<Mailbox>()
        .map_err(|error| ProteinError::Email(format!("Error Parsing From Address: {:?}", error)))?;

    // User Mailbox
    let to: Mailbox = format!("{} <{}>", data.username, data.email)
        .parse::<Mailbox>()
        .map_err(|error| ProteinError::Email(format!("Error Parsing To Address: {:?}", error)))?;

    // Create Email
    let email: Message = Message::builder()
        .to(to)
        .from(from)
        .subject(subject)
        .body(body)
        .map_err(|error| ProteinError::Email(format!("Error Building Email: {:?}", error)))?;

    // Send Email
    mailer
        .send(email)
        .await
        .map_err(|error| ProteinError::Email(format!("Error Sending Email: {:?}", error)))
}
