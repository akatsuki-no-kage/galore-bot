use anyhow::Result;
use nucleo::pattern::CaseMatching;
use poise::{
    serenity_prelude::{ChannelId, CreateAttachment},
    CreateReply,
};

use crate::{command::Meme, Context, CONFIG};

#[poise::command(slash_command)]
pub async fn get(ctx: Context<'_>, name: String) -> Result<()> {
    let mut meme_finder = ctx.data().meme_finder.lock().await;

    meme_finder.pattern.reparse(
        0,
        "gei",
        CaseMatching::Ignore,
        nucleo::pattern::Normalization::Smart,
        false,
    );

    let status = meme_finder.tick(500);
    if status.changed {
        tracing::debug!("New result from nucleo");
    }
    if !status.running {
        tracing::debug!("Finish search");
    }

    let snapshot = meme_finder.snapshot();
    let a = snapshot
        .matched_items(..)
        .map(|item| item.data.1.clone())
        .collect::<Vec<_>>();

    tracing::warn!("{:?}", a);

    let Some(item) = snapshot.get_matched_item(0) else {
        let reply = CreateReply::default()
            .content(format!("No matched meme with name {}", name))
            .ephemeral(true);

        ctx.send(reply).await?;

        return Ok(());
    };

    let (id, name) = item.data;

    let meme_raw = ChannelId::new(CONFIG.data_channel_id)
        .message(ctx.http(), *id)
        .await?;

    let Meme { text, image_url } = Meme::try_from(&meme_raw)?;

    let mut reply = CreateReply::default()
        .reply(true)
        .content(format!("{}{}{}", name, CONFIG.content_separator, text));
    if let Some(image_url) = image_url {
        reply = reply.attachment(CreateAttachment::url(ctx.http(), &image_url).await?);
    }

    ctx.send(reply).await?;

    Ok(())
}
