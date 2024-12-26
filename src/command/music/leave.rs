use anyhow::Result;

use crate::Context;

#[poise::command(slash_command, guild_only)]
pub async fn leave(ctx: Context<'_>) -> Result<()> {
    let guild_id = ctx.guild_id().unwrap();
    let manager = &ctx.data().songbird;
    let has_handler = manager.get(guild_id).is_some();
    if has_handler {
        if let Err(err) = manager.remove(guild_id).await {
            ctx.say(format!(
                ":(( I dont wanna leave you, beacause {err}.\nBut u can try again!"
            ))
            .await?;
        }
        ctx.say("Left voice channel!").await?;
    } else {
        ctx.say("Not in voice channel!").await?;
    }

    Ok(())
}
