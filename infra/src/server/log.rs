use tracing::Level;
use tracing_subscriber::EnvFilter;

pub fn init_tracing() {
    // build env filter set default level to info
    let filter = EnvFilter::builder()
        .with_default_directive(Level::INFO.into())
        .from_env_lossy();

    tracing_subscriber::fmt().with_env_filter(filter).init();
}
