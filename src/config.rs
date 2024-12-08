use serde::Deserialize;

fn default_data_channel_prefix() -> String {
    "data".to_string()
}

#[derive(Deserialize)]
pub struct Config {
    pub discord_token: String,
    #[serde(default = "default_data_channel_prefix")]
    pub data_channel_prefix: String,
}
