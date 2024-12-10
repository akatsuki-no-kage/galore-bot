use anyhow::{anyhow, Result};
use poise::serenity_prelude::ChannelId;

use crate::{command::Meme, Context, CONFIG};

use super::fuzzy;

async fn autocomplete_name<'a>(ctx: Context<'a>, partial: &'a str) -> Vec<String> {
    let mut meme_finder = ctx.data().meme_finder.lock().await;

    fuzzy(&mut meme_finder, partial)
        .await
        .map(|(_, name)| name.clone())
        .collect()
}

#[poise::command(slash_command)]
pub async fn get(
    ctx: Context<'_>,
    #[autocomplete = "autocomplete_name"] name: String,
) -> Result<()> {
    let mut meme_finder = ctx.data().meme_finder.lock().await;
    let id = fuzzy(&mut meme_finder, &name)
        .await
        .next()
        .map(|(id, _)| *id)
        .ok_or(anyhow!("No matching meme"))?;

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
