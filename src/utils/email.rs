/*
______          _       _
| ___ \        | |     (_)
| |_/ / __ ___ | |_ ___ _ _ __
|  __/ '__/ _ \| __/ _ \ | '_ \
| |  | | | (_) | ||  __/ | | | |
\_|  |_|  \___/ \__\___|_|_| |_|

    Made with ❤️
*/

use std::borrow::Cow;

use chrono::NaiveDateTime;
use rocket_client_addr::ClientRealAddr;
use user_agent_parser::OS;
use lettre::{
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
    message::{Mailbox, MultiPart, SinglePart, header::ContentType},
    transport::smtp::{authentication::Credentials, response::Response},
};

use crate::{errors::Error, models::user::User, utils::admin::Config};

pub type Email = AsyncSmtpTransport<Tokio1Executor>;

// Setup Email Transport With SMTP Credentials
pub async fn setup() -> Result<AsyncSmtpTransport<Tokio1Executor>, Box<dyn std::error::Error>> {
    // Get SMTP Credentials
    let username: String =
        std::env::var("SMTP_USERNAME").expect("[!] SMTP_USERNAME Environment Variable Must Be Set");
    let password: String =
        std::env::var("SMTP_PASSWORD").expect("[!] SMTP_PASSWORD Environment Variable Must Be Set");
    let server: String =
        std::env::var("SMTP_SERVER").expect("[!] SMTP_SERVER Environment Variable Must Be Set");

    // Create Credentials
    let credentials: Credentials = Credentials::new(username, password);

    // Create Email
    let email: Email = match AsyncSmtpTransport::<Tokio1Executor>::relay(server.as_str()) {
        Ok(transport) => transport.credentials(credentials).build(),
        Err(error) => panic!("[!] Failed To Create Email: {:?}", error),
    };

    // Email
    Ok(email)
}

// Sends An Email To A User Using The Email Transport
pub async fn send(
    mail: &Email,
    smtp: &Config,
    data: &User,
    subject: &str,
    body: String,
    is_admin: bool,
) -> Result<Response, Error> {
    // System Mailbox
    let from: Mailbox = format!("{} <{}>", smtp.smtp_user, smtp.smtp_username)
        .parse::<Mailbox>()
        .map_err(|error| Error::Email(format!("Error Parsing From Address: {:?}", error)))?;

    // Determine Recipient Mailbox
    let to: Mailbox = if is_admin {
        // Admin Emails Are Sent To The System Email
        from.clone()
    } else {
        // Otherwise, Send To The User
        format!("{} <{}>", data.username, data.email)
            .parse::<Mailbox>()
            .map_err(|error| Error::Email(format!("Error Parsing To Address: {:?}", error)))?
    };

    // Create Email
    let email: Message = Message::builder()
        .to(to)
        .from(from)
        .subject(subject)
        .multipart(
            MultiPart::alternative().singlepart(
                SinglePart::builder()
                    .header(ContentType::TEXT_HTML)
                    .body(body),
            ),
        )
        .map_err(|error| Error::Email(format!("Error Building Email: {:?}", error)))?;

    tracing::info!("[Email] ⚙️ Sending {} Email To: {}", subject, data.email);

    // Send Email
    mail.send(email)
        .await
        .map_err(|error| Error::Email(format!("Error Sending Email: {:?}", error)))
}

pub fn get_ip_address(address: &ClientRealAddr) -> Result<String, Error> {
    match address.get_ipv4_string() {
        Some(ip) => Ok(ip),
        None => return Err(Error::Internal("Failed To Get IP Address".to_string())),
    }
}

pub fn get_user_agent(os: &OS) -> String {
    // Get User Agent
    let name: Cow<'_, str> = os.name.clone().unwrap_or("Device".to_owned().into());
    let version: Cow<'_, str> = os.major.clone().unwrap_or("Not Found".to_owned().into());

    let user_agent: String = format!("{} {}", name, version);
    user_agent
}

pub fn format_date(date: &NaiveDateTime) -> String {
    // %A = Day of the week
    // %B = Month of the year
    // %d = Day of the month
    // %Y = Year
    // %H = Hour
    // %M = Minute
    // %S = Second
    // %p = AM/PM
    // For Example:
    // Sunday, January 01 2026 at 12:00:00 AM
    date.format("%A, %B %d %Y at %H:%M:%S %p").to_string()
}
