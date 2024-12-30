use std::ops::Deref;

use anyhow::{anyhow, Result};
use poise::serenity_prelude::ChannelId;

use crate::{command::Meme, Context, CONFIG};

use super::fuzzy;

async fn autocomplete_name<'a>(ctx: Context<'a>, partial: &'a str) -> Vec<String> {
    fuzzy(ctx.data().memes.read().await.deref(), partial).await
}

#[poise::command(slash_command)]
pub async fn get(
    ctx: Context<'_>,
    #[autocomplete = "autocomplete_name"] name: String,
) -> Result<()> {
    let memes = ctx.data().memes.read().await;

    let id = *memes.get(&name).ok_or(anyhow!("Meme does not exist"))?;

    let meme_raw = ChannelId::new(CONFIG.data_channel_id)
        .message(ctx.http(), id)
        .await?;

    let Meme { text, image_url } = Meme::try_from(&meme_raw)?;

    let mut content = text;
    if let Some(image_url) = image_url {
        content.push('\n');
        content.push_str(&image_url);
    }
    ctx.reply(content).await?;

    Ok(())
}
