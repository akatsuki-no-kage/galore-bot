pub enum MessageKind {
    SameVoiceChannel,
    DifferentVoiceChannel,
    UserNotInVoiceChannel,
}

pub struct Message {
    pub kind: MessageKind,
}

impl Into<String> for Message {
    fn into(self) -> String {
        match self.kind {
            MessageKind::DifferentVoiceChannel => "Bot is on another channel!".to_string(),
            MessageKind::SameVoiceChannel => "Bot is on the same channel with you!".to_string(),
            MessageKind::UserNotInVoiceChannel => "You is not on voice channel!".to_string(),
        }
    }
}
