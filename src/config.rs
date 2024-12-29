use std::sync::LazyLock;

use serde::Deserialize;

fn default_content_separator() -> String {
    "\n\n\n".to_string()
}

fn default_chat_model() -> String {
    "llama3.2".to_string()
}

#[derive(Deserialize)]
pub struct Config {
    pub discord_token: String,

    pub data_channel_id: u64,

    #[serde(default = "default_content_separator")]
    pub content_separator: String,

    #[serde(default = "default_chat_model")]
    pub chat_model: String,
}

pub static CONFIG: LazyLock<Config> = LazyLock::new(|| {
    ::config::Config::builder()
        .add_source(::config::Environment::default().try_parsing(true))
        .add_source(::config::File::with_name("config"))
        .build()
        .unwrap()
        .try_deserialize()
        .unwrap()
});
