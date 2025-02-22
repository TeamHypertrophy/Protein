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
use iso8061_timestamp::Timestamp;
use discord_webhook2::{message::Message, webhook::DiscordWebhook};

use crate::{constants::*, errors::ProteinError};

pub type Webhook = DiscordWebhook;

pub async fn send_audit_log(
    webhook: &State<Webhook>,
    description: &str,
    user: &str,
    ip: &str,
    action: &str,
) -> Result<(), ProteinError> {
    webhook
        .send(&Message::new(|message| {
            message.embed(|embed| {
                embed
                    .title(EMBED_TITLE)
                    .color(EMBED_COLOR)
                    .thumbnail(|thumbnail| thumbnail.url(EMBED_THUMBNAIL))
                    .footer(|footer| footer.text(EMBED_FOOTER).url_icon(EMBED_THUMBNAIL))
                    .timestamp(Timestamp::now_utc())
                    .description(description)
                    .field(|field| field.name("User").value(user).inline(true))
                    .field(|field| field.name("IP Address").value(ip).inline(true))
                    .field(|field| field.name("Action").value(action).inline(true))
            })
        }))
        .await
        .map_err(|e| ProteinError::Webhook(e.to_string()))?;

    Ok(())
}
