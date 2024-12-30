use anyhow::{anyhow, Result};
use ollama_rs::{
    generation::chat::{request::ChatMessageRequest, ChatMessage},
    Ollama,
};
use poise::serenity_prelude::MESSAGE_CODE_LIMIT;
use text_splitter::MarkdownSplitter;

pub async fn generate_ai_response(
    prompt: String,
    model: String,
    history: &mut Vec<ChatMessage>,
) -> Result<Vec<String>> {
    let mut ollama = Ollama::default();

    let chat_message = ChatMessage::user(prompt);

    let response = ollama
        .send_chat_messages_with_history(
            history,
            ChatMessageRequest::new(model, vec![chat_message]),
        )
        .await
        .map_err(|err| anyhow!(err.to_string()))?;

    let ai_response = response.message.content;

    let splitter = MarkdownSplitter::new(MESSAGE_CODE_LIMIT);
    Ok(splitter
        .chunks(&ai_response)
        .map(|chunk| chunk.to_string())
        .collect())
}
