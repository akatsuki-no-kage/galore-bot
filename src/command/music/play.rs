use anyhow::Result;
use songbird::input::{Compose, Input, LiveInput, YoutubeDl};

use crate::Context;

#[poise::command(slash_command, guild_only)]
pub async fn play(
    ctx: Context<'_>,
    #[description = "Url to music video in youtube"] url: String,
) -> Result<()> {
    let guild_id = ctx.guild_id().unwrap();
    let manager = &ctx.data().songbird;
    let do_search = !url.starts_with("http");

    if let Some(handler_lock) = manager.get(guild_id) {
        let mut handler = handler_lock.lock().await;

        let src = if do_search {
            YoutubeDl::new_search(ctx.data().http_client.clone(), url)
        } else {
            YoutubeDl::new(ctx.data().http_client.clone(), url)
        };

        let audio = src.clone().create_async().await?;
        let input = Input::Live(LiveInput::Raw(audio), Some(Box::new(src)));
        handler.play_input(input);
        ctx.say("Playing song!").await?;
    }

    Ok(())
}
