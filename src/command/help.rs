use anyhow::Result;
use poise::samples::HelpConfiguration;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::{util, Context};

async fn autocomplete_command_name<'a>(ctx: Context<'a>, partial: &'a str) -> Vec<String> {
    util::fuzzy(
        ctx.framework()
            .options()
            .commands
            .par_iter()
            .map(|command| &command.name),
        partial,
    )
    .await
}

#[poise::command(slash_command, track_edits)]
pub async fn help(
    ctx: Context<'_>,
    #[description = "Command to get help for"]
    #[autocomplete = "autocomplete_command_name"]
    command: Option<String>,
) -> Result<()> {
    let config = HelpConfiguration {
        show_subcommands: true,
        show_context_menu_commands: true,
        ephemeral: true,
        include_description: true,

        ..Default::default()
    };
    poise::builtins::help(ctx, command.as_deref(), config).await?;

    Ok(())
}
