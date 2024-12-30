use anyhow::Result;
use poise::serenity_prelude as serenity;

use crate::{data::Data, utils, CONFIG};

pub async fn message_event_handler(
    ctx: &serenity::Context,
    message: &serenity::Message,
    data: &Data,
) -> Result<()> {
    if !message.mentions.contains(&ctx.cache.current_user()) {
        tracing::info!("Test");
        return Ok(());
    }

    let channel_id = message.channel_id.get();
    let pages = utils::generate_ai_response(
        message.content_safe(&ctx.cache),
        CONFIG.chat_model.clone(),
        data.ai_chat_history
            .entry(channel_id)
            .or_default()
            .value_mut(),
    )
    .await?;
    utils::reply_paginator(message, pages, ctx).await
}
