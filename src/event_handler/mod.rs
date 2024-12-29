mod message;

use anyhow::{Error, Result};
use poise::serenity_prelude as serenity;

use crate::data::Data;
use message::message_event_handler;

pub async fn event_handler(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    data: &Data,
) -> Result<()> {
    match event {
        serenity::FullEvent::Ready { data_about_bot, .. } => {
            tracing::info!("Logged in as {}", data_about_bot.user.name);
            Ok(())
        }
        serenity::FullEvent::Message { new_message } => {
            message_event_handler(ctx, new_message, data).await
        }
        _ => Ok(()),
    }
}
