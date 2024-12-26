use anyhow::{anyhow, Result};
use dashmap::DashMap;
use poise::serenity_prelude::{CacheHttp, ChannelId, Context, GetMessages};
use reqwest::Client as HttpClient;
use std::sync::Arc;

use crate::CONFIG;

pub async fn get_memes(ctx: &Context) -> Result<DashMap<String, u64>> {
    let memes = DashMap::new();

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

            memes.insert(name.to_string(), id);
        }
        last_id = fetched_meme.last().map(|message| message.id);
    }

    Ok(memes)
}

pub struct Data {
    pub memes: DashMap<String, u64>,
    pub http_client: HttpClient,
    pub songbird: Arc<songbird::Songbird>,
}

impl Data {
    pub async fn new(ctx: &Context, songbird: Arc<songbird::Songbird>) -> Result<Self> {
        Ok(Self {
            memes: get_memes(ctx).await?,
            http_client: HttpClient::new(),
            songbird,
        })
    }
}
