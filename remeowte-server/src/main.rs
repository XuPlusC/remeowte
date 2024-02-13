use axum::{
    routing::{get, post},
    Router,
};
use futures::{executor, future::BoxFuture};
use std::{
    result::Result,
    io::Error,
};
use tracing::{error, info, debug, trace};
use tracing_subscriber::FmtSubscriber;

mod command;
mod auth;

// TODO: read json, make it configurable 
#[tokio::main]
async fn main() {
    // Create and initialize the tracing subscriber
    let subscriber = FmtSubscriber::builder().finish();
    tracing::subscriber::set_global_default(subscriber).expect("Failed to set tracing subscriber");
    
    let app = Router::new()
        // .route("/:cmd", get(command::execute(Box::new(command::CommandExec::new())).await))
        .route("/cmd", post(command::cmd_exec));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
