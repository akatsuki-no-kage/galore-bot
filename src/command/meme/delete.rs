use anyhow::{anyhow, Result};
use poise::{serenity_prelude::ChannelId, CreateReply};

use crate::{Context, CONFIG};

use super::fuzzy;

async fn autocomplete_name<'a>(
    ctx: Context<'a>,
    partial: &'a str,
) -> impl Iterator<Item = String> + 'a {
    fuzzy(&ctx.data().memes, partial).await
}

#[poise::command(slash_command)]
pub async fn delete(
    ctx: Context<'_>,
    #[autocomplete = "autocomplete_name"] name: String,
) -> Result<()> {
    let id = *ctx
        .data()
        .memes
        .get(&name)
        .ok_or(anyhow!("Meme does not exist"))?
        .value();

    ChannelId::new(CONFIG.data_channel_id).delete_message(ctx.http(), id).await?;
    ctx.data().memes.remove(&name);

    ctx.send(
        CreateReply::default()
            .ephemeral(true)
            .content(format!("Delete {}", name)),
    )
    .await?;

    Ok(())
}
