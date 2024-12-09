mod add;
mod get;

use anyhow::{anyhow, Error, Result};
use poise::serenity_prelude::Message;

use add::*;
use get::*;

use crate::Context;
use crate::CONFIG;

pub struct Meme {
    pub text: String,
    pub image_url: Option<String>,
}

impl TryFrom<&Message> for Meme {
    type Error = Error;

    fn try_from(message: &Message) -> Result<Self> {
        let (_, text) = message
            .content
            .split_once(&CONFIG.content_separator)
            .ok_or(anyhow!("Wrong format"))?;
        let text = text.to_string();
        let image_url = message
            .attachments
            .first()
            .map(|attachment| attachment.url.clone());

        Ok(Meme { text, image_url })
    }
}

#[poise::command(slash_command, subcommands("add", "get"), subcommand_required)]
pub async fn meme(_: Context<'_>) -> Result<()> {
    Ok(())
}
