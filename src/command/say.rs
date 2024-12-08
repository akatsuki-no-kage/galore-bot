use color_eyre::Result;

use crate::Context;

#[poise::command(slash_command)]
pub async fn say(
    ctx: Context<'_>,
    #[description = "Message to sent"] message: String,
) -> Result<()> {
    ctx.say(message).await?;

    Ok(())
}
