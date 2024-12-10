mod add;
mod get;

use anyhow::{anyhow, Error, Result};
use nucleo::pattern::CaseMatching;
use nucleo::pattern::Normalization;
use nucleo::Nucleo;
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

async fn fuzzy<'a>(
    finder: &'a mut Nucleo<(u64, String)>,
    name: &'a str,
) -> impl Iterator<Item = &'a (u64, String)> + 'a {
    finder.pattern.reparse(
        0,
        name,
        CaseMatching::Ignore,
        Normalization::Smart,
        false,
    );

    let status = finder.tick(500);
    if status.changed {
        tracing::debug!("New result from nucleo");
    }
    if !status.running {
        tracing::debug!("Finish search");
    }

    let snapshot = finder.snapshot();

    snapshot.matched_items(..).map(|item| item.data)
}


#[poise::command(slash_command, subcommands("add", "get"), subcommand_required)]
pub async fn meme(_: Context<'_>) -> Result<()> {
    Ok(())
}
