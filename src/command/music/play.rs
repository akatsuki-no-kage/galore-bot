use anyhow::Result;

use crate::{
    messages::{Message, MessageKind},
    utils::{
        guild::{GuildUtils, VoiceChannelStates},
        play::play_url,
    },
    Context,
};

#[poise::command(slash_command, guild_only)]
pub async fn play(
    ctx: Context<'_>,
    #[description = "Url to music video in youtube"] url: String,
) -> Result<()> {
    let user_id = ctx.author().id;
    let bot_id = ctx.cache().current_user().id;
    let guild_id = ctx.guild_id().unwrap();
    let manager = &ctx.data().songbird;
    let vc_state = ctx.guild().unwrap().cmp_voice_channel(&bot_id, &user_id);

    match vc_state {
        VoiceChannelStates::Same => {
            ctx.defer().await?;
            let meta = play_url(&ctx.data().http_client, manager, guild_id, url).await?;
            ctx.say(Message {
                kind: MessageKind::Play {
                    title: meta.title.as_ref().unwrap(),
                },
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
            ctx.say(Message {
                kind: MessageKind::BotNotInVoiceChannel,
            })
            .await?;
        }
    }

    Ok(())
}
