use std::fmt::Write;

use poise::serenity_prelude::{Cache, Mentionable, Message, User};

fn replace_user_mention(raw: String, user: &User, bot_id: u64) -> String {
    let mut m = user.mention().to_string();
    // Check whether we're replacing a nickname mention or a normal mention.
    // `UserId::mention` returns a normal mention. If it isn't present in the message, it's
    // a nickname mention.
    if !raw.contains(&m) {
        m.insert(2, '!');
    }

    if user.id.get() == bot_id {
        return raw.replace(&m, "");
    }

    let mut at_distinct = String::with_capacity(38);
    at_distinct.push('@');
    at_distinct.push_str(&user.name);
    if let Some(discriminator) = user.discriminator {
        at_distinct.push('#');
        write!(at_distinct, "{:04}", discriminator.get()).unwrap();
    }

    raw.replace(&m, &at_distinct)
}

pub fn make_ai_prompt(message: &Message, cache: impl AsRef<Cache>) -> String {
    let raw = message.content.clone();

    let raw = message.mentions.iter().fold(raw, |raw, user| {
        replace_user_mention(raw, user, cache.as_ref().current_user().id.get())
    });

    raw.replace("@everyone", "@\u{200B}everyone")
        .replace("@here", "@\u{200B}here")
        .trim()
        .to_string()
}
