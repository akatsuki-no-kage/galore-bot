use anyhow::Result;

use crate::{
    messages::{Message, MessageKind},
    Context,
};
// TODO: Return not playing when it not
#[poise::command(slash_command)]
pub async fn skip(ctx: Context<'_>) -> Result<()> {
    let manager = &ctx.data().songbird;

    let call = match manager.get(ctx.guild_id().unwrap()) {
        Some(handler) => handler,
        None => {
            ctx.say(Message {
                kind: MessageKind::NotPlaying,
            })
            .await?;
            return Ok(());
        }
    };

    let mut call = call.lock().await;
    call.stop();
    ctx.say(":3 Stop!").await?;
    Ok(())
}
