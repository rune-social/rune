#![warn(missing_debug_implementations)]

use std::{
	env,
	net::{Ipv4Addr, SocketAddr}
};

use axum::{Router, routing::get};
use clap::Parser;
use color_eyre::eyre::Result;
use tokio::net::TcpListener;
use tracing::*;
use tracing_subscriber::{
	EnvFilter,
	Layer,
	fmt::{self, format::FmtSpan},
	layer::SubscriberExt,
	util::SubscriberInitExt
};

/// Rune Server
#[derive(Debug, Parser)]
struct Args {
	/// Bind address
	#[clap(short = 'a', long, default_value = "127.0.0.1")]
	bind_address: Ipv4Addr,
	/// Port
	#[clap(short = 'p', long, default_value = "8080")]
	port: u16
}

#[tokio::main]
async fn main() -> Result<()> {
	// parse args
	let args = Args::parse();

	// setup logging
	let env_filter = EnvFilter::new(
		env::var("RUST_LOG")
			.unwrap_or_else(|_| format!("{}=info", env!("CARGO_PKG_NAME")))
	);
	let fmt_layer = fmt::layer().with_span_events(FmtSpan::CLOSE);
	tracing_subscriber::registry()
		.with(fmt_layer.with_filter(env_filter))
		.init();

	// build server with a route
	let server = Router::new().route("/", get(|| async { "Hello, world!" }));

	let listener =
		TcpListener::bind(SocketAddr::new(args.bind_address.into(), args.port))
			.await?;
	info!("Listening on: http://{}", listener.local_addr()?);

	axum::serve(listener, server).await?;

	Ok(())
}
