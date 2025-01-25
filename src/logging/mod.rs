//! # Logging Module

use std::{
	collections::{BTreeMap, HashMap, HashSet},
	sync::Arc,
	time::{Duration, Instant}
};

use dashmap::DashMap;
use derive_more::derive::Display;
use ordered_float::NotNan;
use tracing::{
	Subscriber,
	span::{Attributes, Id}
};
use tracing_subscriber::{Layer, layer::Context, registry::LookupSpan};

/// Execution time statistics for a span
#[derive(Display, Debug)]
#[display("{:#?}", self)]
pub struct SpanTimingsLayerStatistics {
	/// Number of times the span was entered-exited
	pub count: usize,
	/// Total time spent in the span
	pub total: Duration,
	/// Minimum of "time spent in single enter-exit of this span"
	pub min: Duration,
	/// Average of "time spent in single enter-exit of this span"
	pub avg: Duration,
	/// Maximum of "time spent in single enter-exit of this span"
	pub max: Duration,
	/// Percentiles of "time spent in single enter-exit of this span"
	pub percentiles: BTreeMap<NotNan<f64>, Duration>
}

/// Wrapper around [`Instant`] to store the time a span was started
struct StartedAt(Instant);

/// [`tracing`] layer to collect execution timings of spans
///
/// this layer stores a timestamp on every span enter event,
/// and calculates the duration once the span is exited
/// and stores it in a vec for every distinct span name
pub struct SpanTimingsLayer {
	/// list of names of top-level modules (crates) to collect timings for
	allowlisted_modules: HashSet<String>,
	/// map of span names to their execution durations
	span_timings: Arc<DashMap<String, Vec<Duration>>>
}

/// Arc pointer to the span timings map
///
/// `.clone()` is essentially an [`Arc::clone()`] thus cheap
#[derive(Clone)]
pub struct SpanTimingsPtr(Arc<DashMap<String, Vec<Duration>>>);

impl SpanTimingsPtr {
	/// Get the map containing [`SpanTimingsLayerStatistics`] for each spans
	///
	/// percentiles must be [0.0, 100.0) - otherwise they will be ignored
	pub fn get_statistics(
		&self,
		percentiles: &[f64]
	) -> HashMap<String, SpanTimingsLayerStatistics> {
		let mut stats = HashMap::new();
		for kv in self.0.iter() {
			let (name, timings) = kv.pair();
			if timings.is_empty() {
				continue;
			}
			let count = timings.len();
			let total = timings.iter().sum();
			stats.insert(name.clone(), SpanTimingsLayerStatistics {
				count,
				total,
				#[allow(
					clippy::unwrap_used,
					reason = "we check that timings is not empty above"
				)]
				min: *timings.iter().min().unwrap(),
				avg: total / count as u32,
				#[allow(
					clippy::unwrap_used,
					reason = "we check that timings is not empty above"
				)]
				max: *timings.iter().max().unwrap(),
				percentiles: percentiles
					.iter()
					.filter_map(|&p| {
						let p = NotNan::new(p).ok()?;
						let idx = (count as f64 * *p / 100.0).floor() as usize;
						timings.get(idx).map(|&d| (p, d))
					})
					.collect()
			});
		}

		stats
	}
}

impl SpanTimingsLayer {
	/// Create a new [`SpanTimingsLayer`]
	///
	/// `allowlisted_modules` is a list of top-level modules (crates)
	///
	/// if `allowlisted_modules` is empty, all spans will be collected
	pub fn new(allowlisted_modules: &[&'static str]) -> (Self, SpanTimingsPtr) {
		let allowlisted_modules =
			allowlisted_modules.iter().map(|&m| m.to_string()).collect();

		let span_timings: Arc<DashMap<String, Vec<Duration>>> =
			Arc::new(DashMap::new());

		(
			Self {
				allowlisted_modules,
				span_timings: Arc::clone(&span_timings)
			},
			SpanTimingsPtr(span_timings)
		)
	}
}

impl<S> Layer<S> for SpanTimingsLayer
where
	S: Subscriber,
	S: for<'lookup> LookupSpan<'lookup>
{
	fn on_new_span(
		&self,
		_attrs: &Attributes<'_>,
		id: &Id,
		ctx: Context<'_, S>
	) {
		#[allow(
			clippy::unwrap_used,
			reason = "we are inside on_new_span so this always succeeds"
		)]
		let span = ctx.span(id).unwrap();

		let mut should_collect = self.allowlisted_modules.is_empty();
		if let Some(module_name) = span
			.metadata()
			.module_path()
			.and_then(|p| p.split_once("::"))
			.map(|(m, _)| m)
			&& self.allowlisted_modules.contains(module_name)
		{
			should_collect = true;
		}

		// if span module is of our interest
		if should_collect {
			span.extensions_mut().insert(StartedAt(Instant::now()));
		}
	}

	fn on_close(&self, id: Id, ctx: Context<'_, S>) {
		let now = Instant::now();
		#[allow(
			clippy::unwrap_used,
			reason = "we are inside on_close so this always succeeds"
		)]
		let span = ctx.span(&id).unwrap();

		if let Some(started_at) = span.extensions().get::<StartedAt>() {
			self.span_timings
				.entry(span.metadata().name().to_string())
				.or_default()
				.push(now - started_at.0);
		}
	}
}
