use anyhow::Result;

use crate::Context;

mod join;
mod leave;
mod play;

use join::*;
use leave::*;
use play::*;

#[poise::command(slash_command, subcommands("join", "leave", "play"))]
pub async fn music(_ctx: Context<'_>) -> Result<()> {
    Ok(())
}
