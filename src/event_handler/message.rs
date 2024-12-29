use anyhow::Result;
use ollama_rs::generation::completion::request::GenerationRequest;
use poise::serenity_prelude::{self as serenity, CacheHttp};

use crate::{data::Data, CONFIG};

pub async fn message_event_handler(
    ctx: &serenity::Context,
    message: &serenity::Message,
    data: &Data,
) -> Result<()> {
    if !message.mentions.contains(&ctx.cache.current_user()) {
        return Ok(());
    }

    println!("{}", message.content);

    let response = data
        .ollama
        .generate(GenerationRequest::new(
            CONFIG.chat_model.clone(),
            message.content_safe(ctx.cache().unwrap()),
        ))
        .await?
        .response;

    message.reply_ping(ctx.http(), response).await?;

    Ok(())
}
