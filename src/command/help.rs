use anyhow::Result;
use poise::samples::HelpConfiguration;

use crate::Context;

#[poise::command(slash_command, track_edits)]
pub async fn help(
    ctx: Context<'_>,
    #[description = "Command to get help for"]
    #[rest]
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
