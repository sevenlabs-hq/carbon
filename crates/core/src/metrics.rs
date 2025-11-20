//! Atomic-based metrics system for high-performance metric collection.
//!
//! This module provides lock-free, async-free metric primitives (Counter, Gauge, Histogram)
//! that use atomic operations for updates in the hot path. Metrics are defined as static
//! variables and registered in a global registry for export via MetricsExporter implementations.

use {
    crate::error::CarbonResult,
    std::sync::{
        atomic::{AtomicI64, AtomicU64, Ordering},
        OnceLock, RwLock,
    },
};

pub trait Metric: Send + Sync {
    fn name(&self) -> &'static str;
    fn help(&self) -> &'static str {
        ""
    }
}

pub struct Counter {
    name: &'static str,
    help: &'static str,
    value: AtomicU64,
}

impl Counter {
    pub const fn new(name: &'static str, help: &'static str) -> Self {
        Self {
            name,
            help,
            value: AtomicU64::new(0),
        }
    }

    #[inline]
    pub fn inc(&self) {
        self.inc_by(1);
    }

    #[inline]
    pub fn inc_by(&self, v: u64) {
        self.value.fetch_add(v, Ordering::Relaxed);
    }

    #[inline]
    pub fn get(&self) -> u64 {
        self.value.load(Ordering::Relaxed)
    }
}

impl Metric for Counter {
    fn name(&self) -> &'static str {
        self.name
    }

    fn help(&self) -> &'static str {
        self.help
    }
}

pub struct Gauge {
    name: &'static str,
    help: &'static str,
    value: AtomicI64,
}

impl Gauge {
    pub const fn new(name: &'static str, help: &'static str) -> Self {
        Self {
            name,
            help,
            value: AtomicI64::new(0),
        }
    }

    #[inline]
    pub fn set(&self, value: f64) {
        let scaled = (value * 1_000_000.0) as i64;
        self.value.store(scaled, Ordering::Relaxed);
    }

    #[inline]
    pub fn add(&self, delta: f64) {
        let scaled_delta = (delta * 1_000_000.0) as i64;
        self.value.fetch_add(scaled_delta, Ordering::Relaxed);
    }

    #[inline]
    pub fn get(&self) -> f64 {
        let scaled = self.value.load(Ordering::Relaxed);
        scaled as f64 / 1_000_000.0
    }
}

impl Metric for Gauge {
    fn name(&self) -> &'static str {
        self.name
    }

    fn help(&self) -> &'static str {
        self.help
    }
}

pub struct Histogram {
    name: &'static str,
    help: &'static str,
    buckets: Vec<AtomicU64>,
    boundaries: Vec<f64>,
}

impl Histogram {
    pub fn new(name: &'static str, help: &'static str, boundaries: Vec<f64>) -> Self {
        let bucket_count = boundaries.len() + 1;
        let mut buckets = Vec::with_capacity(bucket_count);
        for _ in 0..bucket_count {
            buckets.push(AtomicU64::new(0));
        }

        Self {
            name,
            help,
            buckets,
            boundaries,
        }
    }

    #[inline]
    pub fn record(&self, value: f64) {
        let bucket = self.find_bucket(value);
        self.buckets[bucket].fetch_add(1, Ordering::Relaxed);
    }

    fn find_bucket(&self, value: f64) -> usize {
        match self
            .boundaries
            .binary_search_by(|b| b.partial_cmp(&value).unwrap_or(std::cmp::Ordering::Equal))
        {
            Ok(i) => i + 1,
            Err(i) => i,
        }
    }

    pub fn get(&self) -> HistogramSnapshot {
        let mut counts = Vec::with_capacity(self.buckets.len());
        for bucket in &self.buckets {
            counts.push(bucket.load(Ordering::Relaxed));
        }
        HistogramSnapshot {
            counts,
            boundaries: self.boundaries.clone(),
        }
    }
}

impl Metric for Histogram {
    fn name(&self) -> &'static str {
        self.name
    }

    fn help(&self) -> &'static str {
        self.help
    }
}

pub struct HistogramSnapshot {
    pub counts: Vec<u64>,
    pub boundaries: Vec<f64>,
}

pub struct MetricsSnapshot {
    pub counters: Vec<(&'static str, &'static str, u64)>,
    pub gauges: Vec<(&'static str, &'static str, f64)>,
    pub histograms: Vec<(&'static str, &'static str, HistogramSnapshot)>,
}

pub struct MetricsRegistry {
    counters: RwLock<Vec<&'static Counter>>,
    gauges: RwLock<Vec<&'static Gauge>>,
    histograms: RwLock<Vec<&'static Histogram>>,
}

static REGISTRY: OnceLock<MetricsRegistry> = OnceLock::new();

impl MetricsRegistry {
    pub fn global() -> &'static MetricsRegistry {
        REGISTRY.get_or_init(|| MetricsRegistry {
            counters: RwLock::new(Vec::new()),
            gauges: RwLock::new(Vec::new()),
            histograms: RwLock::new(Vec::new()),
        })
    }

    pub fn register_counter(&self, counter: &'static Counter) {
        let mut counters = self.counters.write().unwrap();
        counters.push(counter);
    }

    pub fn register_gauge(&self, gauge: &'static Gauge) {
        let mut gauges = self.gauges.write().unwrap();
        gauges.push(gauge);
    }

    pub fn register_histogram(&self, histogram: &'static Histogram) {
        let mut histograms = self.histograms.write().unwrap();
        histograms.push(histogram);
    }

    pub fn snapshot(&self) -> MetricsSnapshot {
        let counters = self.counters.read().unwrap();
        let gauges = self.gauges.read().unwrap();
        let histograms = self.histograms.read().unwrap();

        let counter_data: Vec<(&'static str, &'static str, u64)> = counters
            .iter()
            .map(|c| (c.name(), c.help(), c.get()))
            .collect();

        let gauge_data: Vec<(&'static str, &'static str, f64)> = gauges
            .iter()
            .map(|g| (g.name(), g.help(), g.get()))
            .collect();

        let histogram_data: Vec<(&'static str, &'static str, HistogramSnapshot)> = histograms
            .iter()
            .map(|h| (h.name(), h.help(), h.get()))
            .collect();

        MetricsSnapshot {
            counters: counter_data,
            gauges: gauge_data,
            histograms: histogram_data,
        }
    }
}

pub trait MetricsExporter: Send + Sync {
    fn initialize(&self) -> CarbonResult<()> {
        Ok(())
    }

    fn export(&self, snapshot: &MetricsSnapshot) -> CarbonResult<()>;

    fn shutdown(&self) -> CarbonResult<()> {
        Ok(())
    }
}
