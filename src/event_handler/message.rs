use anyhow::Result;
use poise::serenity_prelude as serenity;

use crate::{data::Data, utils, CONFIG};

pub async fn message_event_handler(
    ctx: &serenity::Context,
    message: &serenity::Message,
    data: &Data,
) -> Result<()> {
    if !message.mentions.contains(&ctx.cache.current_user()) {
        return Ok(());
    }

    let prompt = utils::make_ai_prompt(message, &ctx.cache);
    tracing::info!(prompt);

    let channel_id = message.channel_id.get();

    let pages = utils::generate_ai_response(
        prompt,
        CONFIG.chat_model.clone(),
        data.ai_chat_history
            .lock()
            .await
            .entry(channel_id)
            .or_default(),
    )
    .await?;
    utils::reply_paginator(message, pages, ctx).await
}
