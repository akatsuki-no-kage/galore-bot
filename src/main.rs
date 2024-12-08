use color_eyre::Result;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::{fmt::time::ChronoLocal, layer::SubscriberExt, util::SubscriberInitExt};

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

    Ok(())
}
