use anyhow::Result;

use crate::Context;

#[poise::command(slash_command, guild_only)]
pub async fn join(ctx: Context<'_>) -> Result<()> {
    let guild_id = ctx.guild_id().unwrap();
    let channel_id = ctx.channel_id();

    let manager = &ctx.data().songbird;
    manager.join(guild_id, channel_id).await?;

    ctx.say("I'm comingggggggggggggg!").await?;
    Ok(())
}
