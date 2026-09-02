mod error;
{% if use_async %}
use tracing_subscriber::EnvFilter;

use crate::error::Result;

#[tokio::main]
async fn main() -> Result<()> {
    init_tracing();
    run().await
}

async fn run() -> Result<()> {
    tracing::info!(version = env!("CARGO_PKG_VERSION"), "starting {{project-name}}");
    tokio::task::yield_now().await;
    tracing::info!("done");
    Ok(())
}

fn init_tracing() {
    let filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    tracing_subscriber::fmt().with_env_filter(filter).init();
}{% else %}
use std::io::{self, Write};

use crate::error::Result;

fn main() -> Result<()> {
    let mut out = io::stdout().lock();
    writeln!(out, "hello from {{project-name}} v{}", env!("CARGO_PKG_VERSION"))?;
    Ok(())
}{% endif %}
