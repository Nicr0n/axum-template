use crate::{cmd::StartArgs, utils::config::get_socket_address};
use anyhow::Result;
use axum::{Router, routing::get};
use infra::server::config::ServerConfig;
use sea_orm::{Database, DatabaseConnection};
use tokio::signal;
use tower_http::trace::TraceLayer;
use tracing::info;

#[tokio::main]
pub async fn run_server(start_args: StartArgs) -> Result<()> {
    let server_config = ServerConfig::from_env_path(&start_args.env_path);

    info!("Server configuration loaded");

    info!(
        "Connecting to database {}:{} ...",
        server_config.database_config.db_host, server_config.database_config.db_port
    );
    let conn = Database::connect(server_config.database_config.into_sql_url()).await?;
    info!("Database connection established");

    let state = AppState { conn };

    // parse server address into SocketAddr
    let socket_addr = get_socket_address(server_config).await?;
    // create app
    let app = Router::new()
        .route("/healthy", get(|| async { "I'm good" }))
        // add tower http tracing layer
        .layer(TraceLayer::new_for_http())
        .with_state(state);
    let listener = tokio::net::TcpListener::bind(&socket_addr).await?;

    info!("🚀　Server has been start at {}", &socket_addr);
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

#[derive(Clone)]
struct AppState {
    conn: DatabaseConnection,
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("Failed to install SIGTERM handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            println!("\n🚨 Received Ctrl+C (SIGINT). Start exit");
        },
        _ = terminate => {
            println!("\n🚨 Received terminate (SIGTERM)，Start exit...");
        },
    }
}
