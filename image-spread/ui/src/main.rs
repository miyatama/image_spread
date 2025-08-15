use tracing::error;
use tracing_subscriber::prelude::*;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "debug".into()),
        )
        // .with(tracing_subscriber::fmt::layer())
        // TODO jsonでの出力も試したい
        .with(tracing_subscriber::fmt::layer().json())
        .init();

    match ui::get_args() {
        Ok(config) => {
            if let Err(e) = ui::run(&config).await {
                error!("{:?}", e);
                std::process::exit(1);
            }
        }
        Err(e) => {
            error!("{:?}", e);
            std::process::exit(1);
        }
    }
}
