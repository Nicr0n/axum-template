use clap::{Parser, Subcommand};
use infra::server::config::server::BasicServerConfig;

#[derive(Parser, Debug)]
#[command(version,about,long_about=None)]
pub struct Args {
    #[command(subcommand)]
    pub commands: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Start(StartArgs),
}
#[derive(Parser, Debug)]
#[command(about = "Start Web Server")]
pub struct StartArgs {
    #[arg(short('a'), long, env,default_value_t=String::from("0.0.0.0"))]
    pub addr: String,

    #[arg(short, long, env, default_value_t = 7777)]
    pub port: u16,

    #[arg(short,long("env"),help=String::from("use env file or environment variable as configuration"))]
    pub env_path: Option<String>,

    pub sentry_url: Option<String>,
}

impl From<StartArgs> for BasicServerConfig {
    fn from(value: StartArgs) -> Self {
        Self {
            addr: value.addr,
            port: value.port,
        }
    }
}
