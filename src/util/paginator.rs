use std::time::Duration;

use anyhow::Result;
use poise::serenity_prelude::{futures::StreamExt, Context, EditMessage, Message, ReactionType};

async fn paginator(mut message: Message, pages: Vec<String>, ctx: &Context) -> Result<()> {
    let chunk_count = pages.len();

    let mut current_page = 0;

    if chunk_count == 1 {
        return Ok(());
    }

    message
        .react(&ctx.http, ReactionType::Unicode("◀️".to_string()))
        .await?;
    message
        .react(&ctx.http, ReactionType::Unicode("▶️".to_string()))
        .await?;

    let mut reaction_collector = message.await_reactions(ctx).timeout(Duration::MAX).stream();

    while let Some(ref reaction) = reaction_collector.next().await {
        if reaction.user_id == Some(ctx.cache.current_user().id) {
            continue;
        }

        current_page = match reaction.emoji.to_string().as_str() {
            "◀️" => (current_page + chunk_count - 1) % chunk_count,
            "▶️" => (current_page + 1) % chunk_count,
            _ => continue,
        };

        message
            .edit(&ctx.http, EditMessage::new().content(&pages[current_page]))
            .await?;

        message
            .delete_reaction(&ctx.http, reaction.user_id, reaction.emoji.clone())
            .await?;
    }

    Ok(())
}

pub async fn reply_paginator(
    original_message: &Message,
    pages: Vec<String>,
    ctx: &Context,
) -> Result<()> {
    let reply = original_message.reply_ping(&ctx.http, &pages[0]).await?;
    paginator(reply, pages, ctx).await
}
