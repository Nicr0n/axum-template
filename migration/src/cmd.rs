use clap::Parser;

#[derive(Parser, Debug)]
#[command(version,about,long_about=None)]
#[clap(group(
    clap::ArgGroup::new("action")
        .required(true)
        .args(["up", "down", "reset"]),
))]
pub struct Args {
    #[arg(short = 'u', long, help = "apply <number> of migrations")]
    pub up: Option<Option<u32>>,

    #[arg(
        short = 'd',
        long,
        help = "rollback <number> of migrations, default rollback 1 migration"
    )]
    pub down: Option<Option<u32>>,

    #[arg(short = 'r', long, help = "reset all migrations")]
    pub reset: bool,
}
