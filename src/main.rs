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
    net::{Ipv4Addr, SocketAddr},
    path::Path,
    time::Duration
};

use axum::{
    Router,
    body::Body,
    extract::{Request, State},
    http::StatusCode,
    response::Response,
    routing::get
};
use clap::Parser;
use color_eyre::eyre::{Result, eyre};
use logging::SpanTimingsLayer;
use opentelemetry::trace::{Span, Tracer};
use opentelemetry_otlp::{WithExportConfig, WithHttpConfig};
use reqwest::Client;
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

/// default directory to serve compiled frontend code.
const DEFAULT_SERVE_DIR: &str = "frontend/dist";

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
    num_max_instrument_points: usize,
    /// Use reverse proxy instead of serving SERVE_DIR.
    #[clap(long, default_value_t = false)]
    reverse_proxy: bool
}

#[tokio::main]
async fn main() -> Result<()> {
    // parse args
    let args = Args::parse();

    // read .env
    dotenvy::dotenv().ok();

    // setup panic handler
    color_eyre::install()?;

    // setup opentelemetry
    let otlp_span_exporter = opentelemetry_otlp::SpanExporter::builder()
        .with_http()
        .with_timeout(Duration::from_secs(3))
        .with_http_client(Client::new())
        .build()?;
    let tracer_provider = opentelemetry_sdk::trace::TracerProvider::builder()
        .with_batch_exporter(
            otlp_span_exporter,
            opentelemetry_sdk::runtime::Tokio
        )
        .build();
    opentelemetry::global::set_tracer_provider(tracer_provider);

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

    // build server with a route
    let server = Router::new()
        .route(
            "/test/timings",
            get(move || {
                async move {
                    let tracer = opentelemetry::global::tracer("rune/server");
                    let mut span = tracer.start("/test/timings");
                    span.add_event("show timings", vec![]);
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
                .instrument(info_span!("/test/timings"))
            })
        )
        .route(
            "/test/db_connection",
            get(move |State(pool): State<MySqlPool>| async move {
                let socket_info = sqlx::query!(
                    "SELECT *
                    FROM information_schema.processlist
                    WHERE ID=connection_id()"
                )
                .fetch_one(&pool)
                .instrument(info_span!("query_processlist"))
                .await
                .inspect_err(|e| error!(error = %e, debug = ?e))
                .map_err(|_| {
                    (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error")
                })?;

                Ok::<_, (StatusCode, &'static str)>(format!(
                    "{:#?}",
                    socket_info
                ))
            })
        )
        .with_state(pool);

    // add route for serving frontend (reverse_proxy or static)
    let server = if args.reverse_proxy {
        let client = Client::new();
        server.fallback(move |req: Request| async move {
            let path = req.uri().path();
            let path_query =
                req.uri().path_and_query().map(|v| v.as_str()).unwrap_or(path);

            let uri = format!("http://127.0.0.1:8081{}", path_query);
            let reqwest_response =
                client.get(uri).send().await.map_err(|_| {
                    (
                        StatusCode::SERVICE_UNAVAILABLE,
                        "Expo Dev Server Error. Try restarting expo?"
                    )
                })?;

            let mut response_builder =
                Response::builder().status(reqwest_response.status());
            let headers = response_builder.headers_mut().ok_or((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error"
            ))?;
            *headers = reqwest_response.headers().clone();

            let body = response_builder
                .body(Body::from_stream(reqwest_response.bytes_stream()))
                .map_err(|_| {
                    (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error")
                })?;

            Ok::<_, (StatusCode, &'static str)>(body)
        })
    } else {
        // get dist file from env
        let env_serve_dir = env::var("SERVE_DIR")
            .unwrap_or_else(|_| DEFAULT_SERVE_DIR.to_string());
        let path = Path::new(&env_serve_dir);
        if !path.is_dir() {
            return Err(eyre!("Path {:#?} is not a valid directory.", &path));
        }
        server.fallback_service(
            ServeDir::new(path)
                .not_found_service(ServeFile::new(path.join("index.html")))
        )
    };

    let listener =
        TcpListener::bind(SocketAddr::new(args.bind_address.into(), args.port))
            .await?;
    info!("Listening on: http://{}", listener.local_addr()?);

    axum::serve(listener, server).await?;

    Ok(())
}
