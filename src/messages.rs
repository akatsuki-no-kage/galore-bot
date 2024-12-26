pub enum MessageKind<'a> {
    SameVoiceChannel,
    DifferentVoiceChannel,
    UserNotInVoiceChannel,
    BotNotInVoiceChannel,
    Play { title: &'a str },
    NotPlaying,
}

pub struct Message<'a> {
    pub kind: MessageKind<'a>,
}

impl<'a> Into<String> for Message<'a> {
    fn into(self) -> String {
        match self.kind {
            MessageKind::DifferentVoiceChannel => "Bot is on another channel!".to_string(),
            MessageKind::SameVoiceChannel => "Bot is on the same channel with you!".to_string(),
            MessageKind::UserNotInVoiceChannel => "You is not on voice channel!".to_string(),
            MessageKind::BotNotInVoiceChannel => "Bot is not on any voice channel!".to_string(),
            MessageKind::Play { title } => format!("Playing {}", title),
            MessageKind::NotPlaying => "đm mấy thằng gay, có nhạc éo đâu mà phát!".to_string(),
        }
    }
}
