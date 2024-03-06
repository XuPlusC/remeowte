use axum::{
    routing::{get, post},
    Router,
};
use std::{collections::HashMap, path::PathBuf};
use tracing_subscriber::{filter::EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};
use tracing_subscriber::fmt::format::FmtSpan;

mod auth;
mod command;
mod config;
mod cli;
use cli::Cli;

fn init() {
    let rt_param = Cli::init();
    match config::read_config_from_file(rt_param.config_path.to_str().unwrap()) {
        Ok(config) => {
            let mut keys = HashMap::new();
            for key_pair in config.key_pairs.into_iter() {
                let _ = keys.insert(key_pair.ak, key_pair.sk);
            }
            auth::init(keys);
        },
        Err(e) => {
            println!("[Boot] read config fail: {}", e);
        }
    }
    
    init_tracing(rt_param.verbose, rt_param.log_dir);
    tracing::info!("weee");
}

fn init_tracing(verbose: bool, file: PathBuf) {
    let env_filter = EnvFilter::new("debug");

    // Configure the formatter for stdout
    let fmt_stdout = if verbose {
        Some(
            fmt::layer()
                .with_writer(std::io::stdout)
                .with_span_events(FmtSpan::CLOSE)
        )
    } else {
        None
    };
    
    // Configure the logger to write to a file
    let file_appender = tracing_appender::rolling::daily(file, "remeowte-server.log");
    let fmt_file = fmt::layer()
        .with_writer(file_appender)
        .with_span_events(FmtSpan::CLOSE);

    // Combine both layers into a `Registry`
    let registry = tracing_subscriber::registry()
        .with(env_filter)
        .with(fmt_stdout)
        .with(fmt_file);

    // Set as the global default
    registry.init();
}

#[tokio::main]
async fn main() {
    init();

    let app = Router::new()
        // .route("/:cmd", get(command::execute(Box::new(command::CommandExec::new())).await))
        .route("/cmd", post(command::cmd_exec));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
