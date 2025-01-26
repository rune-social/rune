//! # Logging Module

use std::{
    collections::{BTreeMap, HashMap},
    sync::Arc,
    time::{Duration, Instant}
};

use dashmap::DashMap;
use derive_more::derive::Display;
use ordered_float::NotNan;
use rand::{Rng, thread_rng};
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
    /// map of span names to their execution durations
    span_timings: Arc<DashMap<String, BTreeMap<Duration, usize>>>,
    /// max number of timing datapoints to store per span
    max_timing_datapoints_per_span: usize
}

/// Arc pointer to the span timings map
///
/// `.clone()` is essentially an [`Arc::clone()`] thus cheap
#[derive(Clone)]
pub struct SpanTimingsPtr(Arc<DashMap<String, BTreeMap<Duration, usize>>>);

impl SpanTimingsPtr {
    /// Get the map containing [`SpanTimingsLayerStatistics`] for each spans
    ///
    /// percentiles must be [0.0, 100.0) - otherwise they will be ignored
    pub fn get_statistics(
        &self,
        percentiles: &[f64]
    ) -> HashMap<String, SpanTimingsLayerStatistics> {
        self.0
            .iter()
            .filter_map(|kv| {
                let (name, timings) = kv.pair();

                let count = timings.iter().fold(0, |acc, (_, c)| acc + *c);
                let total = timings
                    .iter()
                    .fold(Duration::ZERO, |acc, (d, c)| acc + *d * *c as u32);

                Some((name.clone(), SpanTimingsLayerStatistics {
                    count,
                    total,
                    min: *timings.keys().min()?,
                    avg: total / count as u32,
                    max: *timings.keys().max()?,
                    percentiles: percentiles
                        .iter()
                        .filter_map(|&p| {
                            let p = NotNan::new(p).ok()?;
                            if *p < 0.0 || *p >= 100.0 {
                                return None;
                            }
                            let idx =
                                (count as f64 * *p / 100.0).floor() as usize;
                            timings
                                .iter()
                                .fold((None, 0), |(found, cur), (d, c)| {
                                    let cur = cur + *c;
                                    if found.is_none() && cur > idx {
                                        return (Some(*d), cur);
                                    }
                                    (found, cur)
                                })
                                .0
                                .map(|d| (p, d))
                        })
                        .collect()
                }))
            })
            .collect()
    }
}

impl SpanTimingsLayer {
    /// Create a new [`SpanTimingsLayer`]
    ///
    /// `max_timings_per_span` is the maximum number of timing datapoints to store per span
    ///
    /// if `max_timings_per_span` is 0, there will be no limit (not recommended for long-running programs)
    ///
    /// once the limit is reached, a random datapoint will be removed
    ///
    /// highly recommended to run with [`tracing_subscriber::EnvFilter`] to limit the number of spans,
    /// otherwise the memory usage can grow significantly
    /// ```rust
    /// let (layer, _) = SpanTimingsLayer::new(100);
    /// tracing_subscriber::registry().with(layer.with_filter(EnvFilter::new(format!("{}=info", env!("CARGO_PKG_NAME"))))).init();
    /// ```
    pub fn new(max_timings_per_span: usize) -> (Self, SpanTimingsPtr) {
        let span_timings: Arc<DashMap<String, BTreeMap<Duration, usize>>> =
            Arc::new(DashMap::new());

        (
            Self {
                span_timings: Arc::clone(&span_timings),
                max_timing_datapoints_per_span: max_timings_per_span
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

        span.extensions_mut().insert(StartedAt(Instant::now()));
    }

    fn on_close(&self, id: Id, ctx: Context<'_, S>) {
        let now = Instant::now();
        #[allow(
            clippy::unwrap_used,
            reason = "we are inside on_close so this always succeeds"
        )]
        let span = ctx.span(&id).unwrap();

        if let Some(started_at) = span.extensions().get::<StartedAt>() {
            let mut timings = self
                .span_timings
                .entry(span.metadata().name().to_string())
                .or_default();
            if self.max_timing_datapoints_per_span != 0
                && timings.iter().fold(0, |acc, (_, c)| acc + *c)
                    >= self.max_timing_datapoints_per_span
            {
                let to_remove = if 1 < timings.len() - 1 {
                    thread_rng().gen_range(1..(timings.len() - 1))
                } else {
                    0
                };
                #[allow(
                    clippy::unwrap_used,
                    reason = "to_remove is always in bounds"
                )]
                let target_key = *timings.keys().nth(to_remove).unwrap();
                #[allow(
                    clippy::unwrap_used,
                    reason = "target_key always exists"
                )]
                let target_value = timings.get_mut(&target_key).unwrap();
                if *target_value == 1 {
                    timings.remove(&target_key);
                } else {
                    *target_value -= 1;
                }
            }
            timings
                .entry(now - started_at.0)
                .and_modify(|c| *c += 1)
                .or_insert(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use tracing::*;
    use tracing_subscriber::{
        layer::SubscriberExt,
        reload::Layer,
        util::SubscriberInitExt
    };

    use super::*;

    async fn foo() {
        trace!("foo");
    }

    async fn bar() {
        trace!("bar");
    }

    #[tokio::test]
    async fn test_span_timings_layer() {
        let (layer, ptr) = SpanTimingsLayer::new(0);
        let (layer, handle) = Layer::new(layer);
        tracing_subscriber::registry().with(layer).init();

        for _ in 0..10 {
            foo().instrument(info_span!("foo_span")).await;
        }

        // -1.0 and 100.0 is invalid and should be skipped
        let stats = ptr.get_statistics(&[-1.0, 0.0, 50.0, 99.9, 100.0]);
        assert_eq!(stats.len(), 1);
        let stat = stats.get("foo_span").unwrap();
        assert_eq!(stat.count, 10);
        assert_eq!(stat.percentiles.keys().cloned().collect::<Vec<_>>(), vec![
            NotNan::new(0.0).unwrap(),
            NotNan::new(50.0).unwrap(),
            NotNan::new(99.9).unwrap()
        ]);

        // test bounded timings layer
        let (layer, ptr) = SpanTimingsLayer::new(1);
        handle.reload(layer).unwrap();

        for _ in 0..10 {
            foo().instrument(info_span!("foo_span")).await;
            bar().instrument(info_span!("bar_span")).await;
        }

        // -1.0 and 100.0 is invalid and should be skipped
        let stats = ptr.get_statistics(&[-1.0, 0.0, 50.0, 99.9, 100.0]);
        assert_eq!(stats.len(), 2);
        let stat = stats.get("foo_span").unwrap();
        assert_eq!(stat.count, 1);
        assert_eq!(stat.percentiles.keys().cloned().collect::<Vec<_>>(), vec![
            NotNan::new(0.0).unwrap(),
            NotNan::new(50.0).unwrap(),
            NotNan::new(99.9).unwrap()
        ]);
        let stat = stats.get("bar_span").unwrap();
        assert_eq!(stat.count, 1);

        // test bounded timings rand drop
        let (layer, ptr) = SpanTimingsLayer::new(10);
        handle.reload(layer).unwrap();

        for _ in 0..100 {
            foo().instrument(info_span!("foo_span")).await;
        }

        let stats = ptr.get_statistics(&[50.0]);
        assert_eq!(stats.len(), 1);
        let stat = stats.get("foo_span").unwrap();
        assert_eq!(stat.count, 10);
    }
}
