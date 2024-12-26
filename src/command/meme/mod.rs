mod add;
mod delete;
mod get;

use anyhow::{anyhow, Error, Result};
use dashmap::DashMap;
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use poise::serenity_prelude::Message;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;

use add::*;
use delete::*;
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
    finder: &'a DashMap<String, u64>,
    name: &'a str,
) -> impl Iterator<Item = String> + 'a {
    let matcher = SkimMatcherV2::default();
    let mut matches: Vec<_> = finder
        .par_iter()
        .map(|entry| entry.key().to_string())
        .flat_map(|key| matcher.fuzzy_match(&key, name).map(|score| (key, score)))
        .collect();
    matches.sort_by(|(_, a), (_, b)| b.cmp(a));
    matches.into_iter().map(|(key, _)| key)
}

#[poise::command(
    slash_command,
    subcommands("add", "get", "delete"),
    subcommand_required
)]
pub async fn meme(_: Context<'_>) -> Result<()> {
    Ok(())
}
