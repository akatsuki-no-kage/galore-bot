pub mod command;
pub mod config;

use std::{sync::Arc, time::Duration};

use color_eyre::{Result, eyre::Error};
use poise::{
    Framework, FrameworkOptions,
    serenity_prelude::{Client, GatewayIntents},
};
use tracing::level_filters::LevelFilter;
use tracing_subscriber::{fmt::time::ChronoLocal, layer::SubscriberExt, util::SubscriberInitExt};

use config::Config;

pub struct Data;

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

    color_eyre::install()?;

    let config: Config = ::config::Config::builder()
        .add_source(::config::Environment::default().try_parsing(true))
        .add_source(::config::File::with_name("config"))
        .build()?
        .try_deserialize()?;

    let options = FrameworkOptions {
        commands: vec![command::say()],
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
                Ok(Data {})
            })
        })
        .options(options)
        .build();

    let intents = GatewayIntents::privileged();

    let mut client = Client::builder(&config.discord_token, intents)
        .framework(framework)
        .await?;

    client.start().await?;

    Ok(())
}
