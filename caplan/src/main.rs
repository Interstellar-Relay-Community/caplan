// SPDX-FileCopyrightText: 2024 Project Caplan contributors
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use axum::Router;
use clap::Parser;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{EnvFilter, Registry};

pub(crate) mod types;

#[derive(Debug, Parser)]
#[clap(about, version, author)]
struct Args {
    #[clap(long, short = 'c', value_name = "CONFIG_FILE")]
    config: String,

    #[clap(long, value_name = "LOG_FORMAT", default_value = "normal")]
    log_format: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    // Initialize tracing
    match args.log_format.as_str() {
        "gcp" => {
            let stackdriver = tracing_stackdriver::layer();
            let subscriber = Registry::default()
                .with(stackdriver)
                .with(EnvFilter::from_env("RUST_LOG"));

            tracing::subscriber::set_global_default(subscriber)
                .expect("Could not set up global logger.");
        }
        _ => {
            tracing_subscriber::fmt::init();
        }
    }

    // TODO: Read config

    let app = Router::new();

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;

    tracing::info!("listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await?;

    Ok(())
}
