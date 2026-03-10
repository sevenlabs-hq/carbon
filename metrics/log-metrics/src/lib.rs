use {
    carbon_core::{
        error::CarbonResult,
        metrics::{MetricsExporter, MetricsRegistry, MetricsSnapshot},
    },
    std::{sync::Mutex, time::Instant},
    tokio::time,
    tokio_util::sync::CancellationToken,
};

pub struct LogMetrics {
    start: Mutex<Instant>,
    last_flush: Mutex<Instant>,
    flush_interval_secs: u64,
    cancellation_token: Mutex<Option<CancellationToken>>,
}

impl Default for LogMetrics {
    fn default() -> Self {
        let now = Instant::now();
        Self {
            start: Mutex::new(now),
            last_flush: Mutex::new(now),
            flush_interval_secs: 5,
            cancellation_token: Mutex::new(None),
        }
    }
}

impl LogMetrics {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn new_with_flush_interval(interval_secs: u64) -> Self {
        Self {
            flush_interval_secs: interval_secs,
            ..Self::default()
        }
    }
}

impl MetricsExporter for LogMetrics {
    fn initialize(self: std::sync::Arc<Self>) -> CarbonResult<()> {
        let now = Instant::now();
        *self.start.lock().unwrap() = now;
        *self.last_flush.lock().unwrap() = now;

        let cancellation_token = CancellationToken::new();
        let token_for_task = cancellation_token.clone();
        let interval_secs = self.flush_interval_secs;
        let this = std::sync::Arc::clone(&self);

        tokio::spawn(async move {
            let mut interval = time::interval(time::Duration::from_secs(interval_secs));
            loop {
                tokio::select! {
                    _ = interval.tick() => {
                        let snapshot = MetricsRegistry::global().snapshot();
                        if let Err(e) = this.export(&snapshot) {
                            log::error!("Error exporting log metrics: {e:?}");
                        }
                    }
                    _ = token_for_task.cancelled() => break,
                }
            }
        });
        *self.cancellation_token.lock().unwrap() = Some(cancellation_token);
        Ok(())
    }

    fn export(&self, snapshot: &MetricsSnapshot) -> CarbonResult<()> {
        let start = *self.start.lock().unwrap();
        let mut last_flush = self.last_flush.lock().unwrap();
        let elapsed = start.elapsed();
        let flush_elapsed = last_flush.elapsed();

        let updates_received = snapshot
            .counters
            .iter()
            .find(|(name, _, _)| *name == "carbon_updates_received_total")
            .map(|(_, _, v)| *v)
            .unwrap_or(0);
        let updates_processed = snapshot
            .counters
            .iter()
            .find(|(name, _, _)| *name == "carbon_updates_processed_total")
            .map(|(_, _, v)| *v)
            .unwrap_or(0);
        let updates_successful = snapshot
            .counters
            .iter()
            .find(|(name, _, _)| *name == "carbon_updates_successful_total")
            .map(|(_, _, v)| *v)
            .unwrap_or(0);
        let updates_failed = snapshot
            .counters
            .iter()
            .find(|(name, _, _)| *name == "carbon_updates_failed_total")
            .map(|(_, _, v)| *v)
            .unwrap_or(0);
        let updates_queued = snapshot
            .gauges
            .iter()
            .find(|(name, _, _)| *name == "carbon_updates_queued")
            .map(|(_, _, v)| *v)
            .unwrap_or(0.0);

        let total_received = updates_received + updates_queued as u64;
        let pct_processed = if total_received > 0 {
            (updates_processed * 100) / total_received
        } else {
            0
        };
        let pct_failed = if updates_processed > 0 {
            (updates_failed * 100) / updates_processed
        } else {
            0
        };

        log::info!(
            "{:02}:{:02}:{:02} (+{:?}) | {} processed ({}%), {} successful, {} failed ({}%), {:.0} in queue",
            elapsed.as_secs() / 3600,
            (elapsed.as_secs() % 3600) / 60,
            elapsed.as_secs() % 60,
            flush_elapsed,
            updates_processed,
            pct_processed,
            updates_successful,
            updates_failed,
            pct_failed,
            updates_queued
        );

        for (name, _help, value) in &snapshot.counters {
            log::info!("  {name}: {value}");
        }
        for (name, _help, value) in &snapshot.gauges {
            log::info!("  {name}: {value}");
        }
        for (name, _help, hist) in &snapshot.histograms {
            let total: u64 = hist.counts.iter().sum();
            log::info!("  {name}: count={total}");
        }

        *last_flush = Instant::now();
        Ok(())
    }

    fn shutdown(&self) -> CarbonResult<()> {
        if let Some(token) = self.cancellation_token.lock().unwrap().take() {
            token.cancel();
        }
        Ok(())
    }
}
