/*
______          _       _
| ___ \        | |     (_)
| |_/ / __ ___ | |_ ___ _ _ __
|  __/ '__/ _ \| __/ _ \ | '_ \
| |  | | | (_) | ||  __/ | | | |
\_|  |_|  \___/ \__\___|_|_| |_|

    Made with ❤️
*/

use lettre::{
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
    message::{Mailbox, MultiPart, SinglePart, header::ContentType},
    transport::smtp::{authentication::Credentials, response::Response},
};

use crate::{errors::Error, models::user::User};

pub type Email = AsyncSmtpTransport<Tokio1Executor>;

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

pub async fn send(
    mail: &Email,
    data: &User,
    subject: &str,
    body: String,
) -> Result<Response, Error> {
    // Get SMTP Credentials
    let user: String =
        std::env::var("SMTP_USER").expect("[!] SMTP_USER Environment Variable Must Be Set");
    let username: String =
        std::env::var("SMTP_USERNAME").expect("[!] SMTP_USERNAME Environment Variable Must Be Set");

    // System Mailbox
    let from: Mailbox = format!("{} <{}>", user, username)
        .parse::<Mailbox>()
        .map_err(|error| Error::Email(format!("Error Parsing From Address: {:?}", error)))?;

    // User Mailbox
    let to: Mailbox = format!("{} <{}>", data.username, data.email)
        .parse::<Mailbox>()
        .map_err(|error| Error::Email(format!("Error Parsing To Address: {:?}", error)))?;

    // Create Email
    let email: Message = Message::builder()
        .to(to)
        .from(from)
        .subject(subject)
        .multipart(
            MultiPart::alternative().singlepart(
                SinglePart::builder()
                    .header(ContentType::TEXT_HTML)
                    .body(body.clone()),
            ),
        )
        .map_err(|error| Error::Email(format!("Error Building Email: {:?}", error)))?;

    tracing::info!("[Email] ⚙️ Sending {} Email To: {}", subject, data.email);

    // Send Email
    mail.send(email)
        .await
        .map_err(|error| Error::Email(format!("Error Sending Email: {:?}", error)))
}
