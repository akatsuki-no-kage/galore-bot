use anyhow::{anyhow, Result};
use poise::serenity_prelude::ChannelId;
use rayon::iter::ParallelBridge;

use crate::{command::Meme, util, Context, CONFIG};

async fn autocomplete_name<'a>(ctx: Context<'a>, partial: &'a str) -> Vec<String> {
    util::fuzzy(ctx.data().memes.read().await.keys().par_bridge(), partial).await
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
