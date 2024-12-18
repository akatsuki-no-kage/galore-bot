pub mod command;
pub mod config;
pub mod data;

use std::{
    sync::Arc,
    time::Duration,
};

use anyhow::{Error, Result};
use data::Data;
use poise::{
    serenity_prelude::{Client, GatewayIntents},
    Framework, FrameworkOptions,
};
use tracing::level_filters::LevelFilter;
use tracing_subscriber::{fmt::time::ChronoLocal, layer::SubscriberExt, util::SubscriberInitExt};

pub use config::CONFIG;

type Context<'a> = poise::Context<'a, Data, Error>;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .pretty()
                .with_timer(ChronoLocal::default()),
        )
        .with(LevelFilter::INFO)
        .init();

    let options = FrameworkOptions {
        commands: vec![command::say(), command::meme()],
        prefix_options: poise::PrefixFrameworkOptions {
            prefix: Some("~".into()),
            edit_tracker: Some(Arc::new(poise::EditTracker::for_timespan(
                Duration::from_secs(3600),
            ))),
            additional_prefixes: vec![
                poise::Prefix::Literal("hey bot,"),
                poise::Prefix::Literal("hey bot"),
            ],
            ..Default::default()
        },
        ..Default::default()
    };

    let framework = Framework::builder()
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                println!("Logged in as {}", _ready.user.name);
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;

                Data::new(ctx).await
            })
        })
        .options(options)
        .build();

    let intents = GatewayIntents::privileged();

    let mut client = Client::builder(&CONFIG.discord_token, intents)
        .framework(framework)
        .await?;

    client.start().await?;

    Ok(())
}
