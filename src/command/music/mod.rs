use anyhow::Result;

use crate::Context;

mod join;
mod leave;
mod play;
mod skip;
mod volume;

use join::*;
use leave::*;
use play::*;
use skip::*;
use volume::*;

#[poise::command(slash_command, subcommands("join", "leave", "play", "skip"))]
pub async fn music(_ctx: Context<'_>) -> Result<()> {
    Ok(())
}
