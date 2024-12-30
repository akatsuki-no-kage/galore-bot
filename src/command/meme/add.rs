use anyhow::{bail, Result};
use poise::{
    serenity_prelude::{Attachment, ChannelId, CreateAttachment, CreateMessage},
    CreateReply,
};

use crate::{Context, CONFIG};

#[poise::command(slash_command)]
pub async fn add(ctx: Context<'_>, name: String, text: String, image: Attachment) -> Result<()> {
    ctx.defer().await?;

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

    let mut memes = ctx.data().memes.write().await;

    if memes.contains_key(&name) {
        bail!("Meme already existed");
    }

    ctx.send(
        CreateReply::default()
            .ephemeral(true)
            .content(format!("{} added", name)),
    )
    .await?;
    memes.insert(name, id);

    Ok(())
}
