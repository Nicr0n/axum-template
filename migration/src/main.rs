use anyhow::Result;
use clap::Parser;

use infra::server::{config::database::DatabaseConfig, log};
use migration::{Migrator, MigratorTrait, sea_orm::Database};
use tracing::info;

use crate::cmd::Args;

mod cmd;

#[tokio::main]
async fn main() -> Result<()> {
    if let Err(e) = dotenvy::dotenv() {
        info!("Cannot open .env file Reason:{},Use system environment.", e)
    }

    log::init_tracing();
    let args = Args::parse();

    let database_config = DatabaseConfig::load();
    let conn = Database::connect(database_config.into_sql_url()).await?;

    if let Some(up) = args.up {
        if let Some(number) = up {
            info!("applying {} migrations...", number);
            Migrator::up(&conn, up).await?;
            info!("applied {} migrations", number);
        } else {
            info!("applying all migrations...");
            Migrator::up(&conn, None).await?;
            info!("all migrations applied");
        }
    }

    if let Some(down) = args.down {
        if let Some(number) = down {
            info!("applying {} rollbacks...", number);
            Migrator::down(&conn, down).await?;
            info!("applied {} rollbacks", number);
        } else {
            info!("applying 1 rollback...");
            Migrator::down(&conn, Some(1)).await?;
            info!("applied 1 rollback");
        }
    }

    if args.reset {
        info!("resetting all migrations...");
        Migrator::reset(&conn).await?;
        info!("all migrations reset");
    }

    Ok(())
}
