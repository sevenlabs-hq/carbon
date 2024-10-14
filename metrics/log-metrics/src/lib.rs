use std::{collections::HashMap, time::Instant};

use async_trait::async_trait;
use carbon_core::{error::CarbonResult, metrics::Metrics};
use tokio::sync::RwLock;

pub struct LogMetrics {
    pub updates_received: RwLock<u64>,
    pub updates_processed: RwLock<u64>,
    pub updates_successful: RwLock<u64>,
    pub updates_failed: RwLock<u64>,
    pub updates_queued: RwLock<u64>,
    pub updates_processing_times: RwLock<Vec<u64>>,

    pub counters: RwLock<HashMap<String, u64>>,
    pub gauges: RwLock<HashMap<String, f64>>,
    pub histograms: RwLock<HashMap<String, Vec<f64>>>,

    pub start: RwLock<Instant>,
    pub last_flush: RwLock<Instant>,
}

#[async_trait]
impl Metrics for LogMetrics {
    async fn initialize(&self) -> CarbonResult<()> {
        let mut start = self.start.write().await;
        *start = Instant::now();

        let mut last_flush = self.last_flush.write().await;
        *last_flush = *start;

        Ok(())
    }

    async fn flush(&self) -> CarbonResult<()> {
        let mut updates_processing_times = self.updates_processing_times.write().await;

        let updates_processing_times_avg = if !updates_processing_times.is_empty() {
            updates_processing_times.iter().sum::<u64>() / updates_processing_times.len() as u64
        } else {
            0
        };
        let updates_processing_times_min = *updates_processing_times.iter().min().unwrap_or(&0);
        let updates_processing_times_max = *updates_processing_times.iter().max().unwrap_or(&0);

        let updates_received = self.updates_received.read().await;
        let updates_queued = self.updates_queued.read().await;

        let total_updates_received = *updates_received + *updates_queued;

        let updates_successful = self.updates_successful.read().await;
        let updates_failed = self.updates_failed.read().await;
        let updates_processed = self.updates_processed.read().await;

        let start = self.start.read().await;
        let mut last_flush = self.last_flush.write().await;

        log::info!(
            "{:02}:{:02}:{:02} (+{:?}) | {} processed ({}%), {} successful, {} failed ({}%), {} in queue, avg: {}ms, min: {}ms, max: {}ms",
            start.elapsed().as_secs() / 3600,
            (start.elapsed().as_secs() % 3600) / 60,
            start.elapsed().as_secs() % 60,
            last_flush.elapsed(),
            updates_processed,
            if total_updates_received > 0 {(*updates_processed * 100) / total_updates_received} else {0},
            updates_successful,
            updates_failed,
            if *updates_processed > 0 {(*updates_failed * 100) / *updates_processed} else {0},
            updates_queued,
            updates_processing_times_avg,
            updates_processing_times_min,
            updates_processing_times_max
        );

        *last_flush = Instant::now();

        updates_processing_times.clear();

        Ok(())
    }

    async fn shutdown(&self) -> CarbonResult<()> {
        Ok(())
    }

    async fn increment_counter(&self, name: &str, value: u64) -> CarbonResult<()> {
        match name {
            "updates_queued" => {
                let mut updates_queued = self.updates_queued.write().await;
                *updates_queued += value;
            }
            "updates_processed" => {
                let mut updates_processed = self.updates_processed.write().await;
                *updates_processed += value;
            }
            "updates_successful" => {
                let mut updates_successful = self.updates_successful.write().await;
                *updates_successful += value;
            }
            "updates_failed" => {
                let mut updates_failed = self.updates_failed.write().await;
                *updates_failed += value;
            }
            _ => {
                let mut counters = self.counters.write().await;
                let counter = counters.entry(name.to_string()).or_insert(0);
                *counter += value;
            }
        };

        Ok(())
    }

    async fn update_gauge(&self, name: &str, value: f64) -> CarbonResult<()> {
        match name {
            "updates_queued" => {
                let mut updates_queued = self.updates_queued.write().await;
                *updates_queued = value as u64;
            }
            _ => {
                let mut gauges = self.gauges.write().await;
                let gauge = gauges.entry(name.to_string()).or_insert(0.0);
                *gauge = value;
            }
        };

        Ok(())
    }

    async fn record_histogram(&self, name: &str, value: f64) -> CarbonResult<()> {
        match name {
            "updates_processing_times" => {
                let mut updates_processing_times = self.updates_processing_times.write().await;
                updates_processing_times.push(value as u64);
            }
            _ => {
                let mut histograms = self.histograms.write().await;
                let histogram = histograms.entry(name.to_string()).or_insert(Vec::new());
                histogram.push(value);
            }
        };

        Ok(())
    }
}
