use anyhow::{anyhow, Result};
use nucleo::Nucleo;
use poise::serenity_prelude::{CacheHttp, ChannelId, Context, GetMessages};
use reqwest::Client as HttpClient;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::CONFIG;

pub async fn build_finder(ctx: &Context) -> Result<Nucleo<(u64, String)>> {
    let meme_finder = Nucleo::new(nucleo::Config::DEFAULT, Arc::new(|| {}), None, 1);
    let injector = meme_finder.injector();

    let channel = ChannelId::new(CONFIG.data_channel_id);
    let mut last_id = None;

    loop {
        let mut query_option = GetMessages::default().limit(100);
        if let Some(last_id) = last_id {
            query_option = query_option.before(last_id);
        }

        let fetched_meme = channel.messages(ctx.http(), query_option).await?;
        if fetched_meme.is_empty() {
            break;
        }

        for message in fetched_meme.iter() {
            if message.author.id != ctx.cache.current_user().id {
                continue;
            }
            let id = message.id.get();
            let (name, _) = message
                .content
                .split_once(&CONFIG.content_separator)
                .ok_or(anyhow!("Wrong format"))?;
            let name = name.to_string();

            injector.push((id, name), |(_, name), row| {
                row[0] = name.as_str().into();
            });
        }
        last_id = fetched_meme.last().map(|message| message.id);
    }

    Ok(meme_finder)
}

pub struct Data {
    pub meme_finder: Mutex<Nucleo<(u64, String)>>,
    pub http_client: HttpClient,
    pub songbird: Arc<songbird::Songbird>,
}

impl Data {
    pub async fn new(ctx: &Context, songbird: Arc<songbird::Songbird>) -> Result<Self> {
        let meme_finder = Mutex::new(build_finder(ctx).await?);
        let http_client = HttpClient::new();

        Ok(Self {
            meme_finder,
            http_client,
            songbird,
        })
    }
}
