use axum::{
    routing::{get, post},
    Router,
};
use std::{env, io::Error, result::Result, path::PathBuf};
use tracing::{debug, error, info, trace};
use tracing_subscriber::{FmtSubscriber, filter::EnvFilter};

mod auth;
mod command;
mod config;
mod cli;
use cli::{Cli, RuntimeParam};

fn init_tracing() {
    let env_filter = EnvFilter::new("debug");
    let subscriber = FmtSubscriber::builder()
        .with_env_filter(env_filter)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("Failed to set tracing subscriber");
}

// TODO: read json, make it configurable
#[tokio::main]
async fn main() {
    // Create and initialize the tracing subscriber
    init_tracing();

    let rt_param = Cli::init();

    tracing::info!("weee");

    let app = Router::new()
        // .route("/:cmd", get(command::execute(Box::new(command::CommandExec::new())).await))
        .route("/cmd", post(command::cmd_exec));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
