use poise::serenity_prelude::{model::id::ChannelId, Guild, UserId};

pub trait GuildUtils {
    fn get_user_voice_channel(&self, user_id: &UserId) -> Option<ChannelId>;
    fn cmp_voice_channel(&self, first_user: &UserId, second_user: &UserId) -> VoiceChannelStates;
}

impl GuildUtils for Guild {
    fn get_user_voice_channel(&self, user_id: &UserId) -> Option<ChannelId> {
        self.voice_states
            .get(user_id)
            .and_then(|voice_state| voice_state.channel_id)
    }
    fn cmp_voice_channel(&self, first_user: &UserId, second_user: &UserId) -> VoiceChannelStates {
        let first = self.get_user_voice_channel(first_user);
        let second = self.get_user_voice_channel(second_user);
        match (first, second) {
            (None, None) => VoiceChannelStates::None,
            (None, Some(_user_channel_id)) => VoiceChannelStates::OnlyUser,
            (Some(_bot_channel_id), None) => VoiceChannelStates::OnlyBot,
            (Some(bot_channel_id), Some(user_channel_id)) => {
                if bot_channel_id == user_channel_id {
                    VoiceChannelStates::Same
                } else {
                    VoiceChannelStates::Different
                }
            }
        }
    }
}

pub enum VoiceChannelStates {
    /// Bot and user are on the same channel
    Same,
    /// Bot is on another channel
    Different,
    /// Just user in a voice channel, bot is not
    OnlyUser,
    /// Just bot is on a voice channel, user is not
    OnlyBot,
    /// Both are not on channel
    None,
}
