use anyhow::Result;
use std::ops::Deref;

use crate::{
    messages::{Message, MessageKind},
    util::guild::{GuildUtils, VoiceChannelStates},
    Context,
};

#[poise::command(slash_command, guild_only)]
pub async fn join(ctx: Context<'_>) -> Result<()> {
    let user_id = ctx.author().id;
    let bot_id = ctx.cache().current_user().id;

    let manager = &ctx.data().songbird;
    let vc_state = ctx
        .guild()
        .unwrap()
        .deref()
        .cmp_voice_channel(&bot_id, &user_id);

    match vc_state {
        VoiceChannelStates::Same => {
            ctx.say(Message {
                kind: MessageKind::SameVoiceChannel,
            })
            .await?;
        }
        VoiceChannelStates::None | VoiceChannelStates::OnlyBot => {
            ctx.say(Message {
                kind: MessageKind::UserNotInVoiceChannel,
            })
            .await?;
        }
        VoiceChannelStates::Different => {
            ctx.say(Message {
                kind: MessageKind::DifferentVoiceChannel,
            })
            .await?;
        }
        VoiceChannelStates::OnlyUser => {
            ctx.defer().await?;
            let guild_id = ctx.guild_id().unwrap();
            let channel_id = ctx.channel_id();

            manager.join(guild_id, channel_id).await?;
            ctx.say("I'm comingggggggggggggg!").await?;
        }
    }

    Ok(())
}
