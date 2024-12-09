use anyhow::Result;
use poise::serenity_prelude::{Attachment, ChannelId, CreateAttachment, CreateMessage};

use crate::{Context, CONFIG};

#[poise::command(slash_command)]
pub async fn add(ctx: Context<'_>, name: String, text: String, image: Attachment) -> Result<()> {
    let id = ChannelId::new(CONFIG.data_channel_id)
        .send_files(
            ctx.http(),
            vec![CreateAttachment::url(ctx.http(), &image.url).await?],
            CreateMessage::default()
                .content(format!("{}{}{}", name, &CONFIG.content_separator, text)),
        )
        .await?
        .id
        .get();

    let meme_finder = ctx.data().meme_finder.lock().await;
    let injector = meme_finder.injector();

    injector.push((id, name), |(_, name), row| {
        row[0] = name.as_str().into();
    });

    Ok(())
}
