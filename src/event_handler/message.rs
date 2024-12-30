use anyhow::{anyhow, Result};
use ollama_rs::{
    generation::chat::{request::ChatMessageRequest, ChatMessage},
    Ollama,
};
use poise::serenity_prelude::{self as serenity, CreateMessage, MESSAGE_CODE_LIMIT};
use rand::Rng;
use text_splitter::MarkdownSplitter;

use crate::{data::Data, util, CONFIG};

pub async fn message_event_handler(
    ctx: &serenity::Context,
    message: &serenity::Message,
    data: &Data,
) -> Result<()> {
    if message.author.id == ctx.cache.current_user().id {
        return Ok(());
    }

    let should_reply = rand::thread_rng().gen_bool(CONFIG.random_reply_probabiliy);
    if !should_reply && !message.mentions.contains(&ctx.cache.current_user()) {
        return Ok(());
    }

    let prompt = util::make_ai_prompt(message, &ctx.cache);
    tracing::info!(prompt);

    let channel_id = message.channel_id.get();

    let mut ollama = Ollama::default();

    let chat_message = ChatMessage::user(prompt);

    let response = ollama
        .send_chat_messages_with_history(
            data.ai_chat_history
                .lock()
                .await
                .entry(channel_id)
                .or_default(),
            ChatMessageRequest::new(CONFIG.chat_model.clone(), vec![chat_message]),
        )
        .await
        .map_err(|err| anyhow!(err.to_string()))?
        .message
        .content;

    let splitter = MarkdownSplitter::new(MESSAGE_CODE_LIMIT);
    let chunks: Vec<_> = splitter.chunks(&response).collect();

    message.reply_mention(&ctx.http, chunks[0]).await?;

    for chunk in chunks.into_iter().skip(1) {
        message
            .channel_id
            .send_message(&ctx.http, CreateMessage::new().content(chunk))
            .await?;
    }

    Ok(())
}
