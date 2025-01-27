#![warn(
    missing_debug_implementations,
    missing_docs,
    clippy::missing_docs_in_private_items,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::indexing_slicing,
    clippy::panic,
    clippy::todo,
    clippy::unimplemented,
    clippy::unreachable
)]
#![feature(let_chains)]

//! Rune Server

use std::{
    env,
    net::{Ipv4Addr, SocketAddr}
};

use axum::{Router, extract::State, http::StatusCode, routing::get};
use clap::Parser;
use color_eyre::eyre::Result;
use logging::SpanTimingsLayer;
use sqlx::MySqlPool;
use tokio::net::TcpListener;
use tower_http::services::{ServeDir, ServeFile};
use tracing::*;
use tracing_subscriber::{
    EnvFilter,
    Layer,
    fmt::{self, format::FmtSpan},
    layer::SubscriberExt,
    util::SubscriberInitExt
};

mod db;
mod logging;

/// Rune Server
#[derive(Debug, Parser)]
struct Args {
    /// Bind address
    #[clap(short = 'a', long, default_value = "127.0.0.1")]
    bind_address: Ipv4Addr,
    /// Port
    #[clap(short = 'p', long, default_value_t = 8080)]
    port: u16,
    /// Max instrumenting datapoints to keep per span.
    /// Set to 0 to not keep any
    #[clap(long, default_value_t = 1000)]
    num_max_instrument_points: usize
}

#[tokio::main]
async fn main() -> Result<()> {
    // parse args
    let args = Args::parse();

    // read .env
    dotenvy::dotenv().ok();

    // setup panic handler
    color_eyre::install()?;

    // setup logging
    let env_filter =
        EnvFilter::new(env::var("RUST_LOG").unwrap_or_else(|_| {
            format!("warn,{}=info", env!("CARGO_PKG_NAME"))
        }));
    let fmt_layer = fmt::layer().with_span_events(FmtSpan::CLOSE);
    let (timings_layer, span_timings_ptr) =
        if args.num_max_instrument_points != 0 {
            let (timings_layer, span_timings_ptr) =
                SpanTimingsLayer::new(args.num_max_instrument_points);
            (Some(timings_layer), Some(span_timings_ptr))
        } else {
            (None, None)
        };
    tracing_subscriber::registry()
        .with(fmt_layer.with_filter(env_filter))
        .with(timings_layer.with_filter(EnvFilter::new(format!(
            "{}=info",
            env!("CARGO_PKG_NAME")
        ))))
        .init();

    // setup database
    let pool = db::init(&env::var("DATABASE_URL")?).await?;

    let default_serve_dir = "frontend/dist";

    // get dist file from env
    let env_serve_dir =
        env::var("SERVE_DIR").unwrap_or(default_serve_dir.to_owned());

    let path = std::path::Path::new(&env_serve_dir);

    let serve_dir = if path.exists() && path.is_dir() {
        path.to_str().unwrap_or(default_serve_dir)
    } else {
        default_serve_dir
    };

    // serve frontend/dist and fallback to index.html
    let serve_dir = ServeDir::new(serve_dir).not_found_service(ServeFile::new(
        serve_dir.to_owned() + "/index.html"
    ));

    // build server with a route
    let server = Router::new()
        .fallback_service(serve_dir)
        .route(
            "/test/timings",
            get(move || {
                async move {
                    if let Some(span_timings_ptr) = span_timings_ptr {
                        format!(
                            "{:#?}",
                            span_timings_ptr
                                .get_statistics(&[50.0, 90.0, 99.0, 99.9])
                        )
                    } else {
                        "Span timings not enabled".to_string()
                    }
                }
                .instrument(info_span!("/debug"))
            })
        )
        .route(
            "/test/db_connection",
            get(move |State(pool): State<MySqlPool>| async move {
                let socket_info = sqlx::query!("SELECT * FROM information_schema.processlist WHERE ID=connection_id()")
                    .fetch_one(&pool)
                    .instrument(info_span!("query_processlist"))
                    .await
                    .inspect_err(|e| error!(error = %e, debug = ?e))
                    .map_err(|_| {
                        (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            "Internal Server Error"
                        )
                    })?;

                Ok::<_, (StatusCode, &'static str)>(
                    format!("{:#?}", socket_info)
                )
            })
        )
        .with_state(pool);

    let listener =
        TcpListener::bind(SocketAddr::new(args.bind_address.into(), args.port))
            .await?;
    info!("Listening on: http://{}", listener.local_addr()?);

    axum::serve(listener, server).await?;

    Ok(())
}
