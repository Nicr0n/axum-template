use clap::Parser;
use infra::server::log;
use tracing::info;

use crate::cmd::{Args, Commands};

mod cmd;
mod server;
mod utils;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if let Err(e) = dotenvy::dotenv() {
        info!("Cannot open .env file Reason:{},Use system environment.", e)
    }

    if let Some(url) = dotenvy::var("SENRTY").ok() {
        info!("Detect sentry config.Url:{}", url);
    }

    log::init_tracing();

    let args = Args::parse();

    match args.commands {
        Commands::Start(start_args) => {
            server::app::run_server(start_args)?;
        }
    }

    Ok(())
}
