use std::sync::Arc;

use anyhow::Result;
use poise::serenity_prelude::GuildId;
use reqwest::Client;
use songbird::{
    input::{AuxMetadata, Compose, Input, LiveInput, YoutubeDl},
    Songbird,
};

pub async fn play_url(
    http_client: &Client,
    manager: &Arc<Songbird>,
    guild_id: GuildId,
    url: String,
) -> Result<Arc<AuxMetadata>> {
    let do_search = !url.starts_with("http");
    let mut src = if do_search {
        YoutubeDl::new_search(http_client.clone(), url)
    } else {
        YoutubeDl::new(http_client.clone(), url)
    };

    let audio = src.clone().create_async().await?;
    let input = Input::Live(LiveInput::Raw(audio), Some(Box::new(src.clone())));

    if let Some(handler_lock) = manager.get(guild_id) {
        let mut handler = handler_lock.lock().await;
        handler.play_input(input);
    }

    Ok(Arc::new(src.aux_metadata().await?))
}
