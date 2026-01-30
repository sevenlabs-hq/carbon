use {
    carbon_core::{error::CarbonResult, metrics::MetricsExporter, metrics::MetricsSnapshot},
    std::{sync::Mutex, time::Instant},
};

pub struct LogMetricsExporter {
    start: Mutex<Instant>,
    last_flush: Mutex<Instant>,
}

impl Default for LogMetricsExporter {
    fn default() -> Self {
        let now = Instant::now();
        Self {
            start: Mutex::new(now),
            last_flush: Mutex::new(now),
        }
    }
}

impl LogMetricsExporter {
    pub fn new() -> Self {
        Self::default()
    }
}

impl MetricsExporter for LogMetricsExporter {
    fn initialize(&self) -> CarbonResult<()> {
        let now = Instant::now();
        *self.start.lock().unwrap() = now;
        *self.last_flush.lock().unwrap() = now;
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
            .find(|(name, _, _)| *name == "carbon_updates_received")
            .map(|(_, _, v)| *v)
            .unwrap_or(0);
        let updates_processed = snapshot
            .counters
            .iter()
            .find(|(name, _, _)| *name == "carbon_updates_processed")
            .map(|(_, _, v)| *v)
            .unwrap_or(0);
        let updates_successful = snapshot
            .counters
            .iter()
            .find(|(name, _, _)| *name == "carbon_updates_successful")
            .map(|(_, _, v)| *v)
            .unwrap_or(0);
        let updates_failed = snapshot
            .counters
            .iter()
            .find(|(name, _, _)| *name == "carbon_updates_failed")
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
        Ok(())
    }
}
