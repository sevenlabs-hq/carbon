use {
    carbon_core::{error::CarbonResult, metrics::MetricsExporter, metrics::MetricsSnapshot},
    std::sync::{Arc, Mutex},
    std::time::Instant,
};

pub struct LogMetricsExporter {
    start: Arc<Mutex<Instant>>,
    last_flush: Arc<Mutex<Instant>>,
}

impl LogMetricsExporter {
    pub fn new() -> Self {
        Self {
            start: Arc::new(Mutex::new(Instant::now())),
            last_flush: Arc::new(Mutex::new(Instant::now())),
        }
    }
}

impl Default for LogMetricsExporter {
    fn default() -> Self {
        Self::new()
    }
}

impl MetricsExporter for LogMetricsExporter {
    fn initialize(&self) -> CarbonResult<()> {
        let mut start = self.start.lock().unwrap();
        *start = Instant::now();

        let mut last_flush = self.last_flush.lock().unwrap();
        *last_flush = *start;

        Ok(())
    }

    fn export(&self, snapshot: &MetricsSnapshot) -> CarbonResult<()> {
        let start = self.start.lock().unwrap();
        let mut last_flush = self.last_flush.lock().unwrap();

        let elapsed = start.elapsed();
        let time_since_flush = last_flush.elapsed();

        let updates_received = snapshot
            .counters
            .iter()
            .find(|(name, _, _)| *name == "carbon_updates_received")
            .map(|(_, _, value)| *value)
            .unwrap_or(0);

        let updates_processed = snapshot
            .counters
            .iter()
            .find(|(name, _, _)| *name == "carbon_updates_processed")
            .map(|(_, _, value)| *value)
            .unwrap_or(0);

        let updates_successful = snapshot
            .counters
            .iter()
            .find(|(name, _, _)| *name == "carbon_updates_successful")
            .map(|(_, _, value)| *value)
            .unwrap_or(0);

        let updates_failed = snapshot
            .counters
            .iter()
            .find(|(name, _, _)| *name == "carbon_updates_failed")
            .map(|(_, _, value)| *value)
            .unwrap_or(0);

        let updates_queued = snapshot
            .gauges
            .iter()
            .find(|(name, _, _)| *name == "carbon_updates_queued")
            .map(|(_, _, value)| *value)
            .unwrap_or(0.0) as u64;

        let total_updates_received = updates_received + updates_queued;

        let processing_time_hist = snapshot
            .histograms
            .iter()
            .find(|(name, _, _)| *name == "carbon_updates_process_time_milliseconds");

        let (processing_time_avg, processing_time_min, processing_time_max) =
            if let Some((_, _, hist)) = processing_time_hist {
                let total_count: u64 = hist.counts.iter().sum();
                if total_count > 0 {
                    let sum: f64 = hist
                        .boundaries
                        .iter()
                        .enumerate()
                        .map(|(i, boundary)| {
                            let count = hist.counts.get(i).copied().unwrap_or(0);
                            let midpoint = if i == 0 {
                                0.0
                            } else if i < hist.boundaries.len() {
                                (hist.boundaries[i - 1] + *boundary) / 2.0
                            } else {
                                hist.boundaries.last().copied().unwrap_or(0.0) * 2.0
                            };
                            count as f64 * midpoint
                        })
                        .sum();

                    let avg = sum / total_count as f64;
                    let min = hist
                        .boundaries
                        .iter()
                        .enumerate()
                        .find(|(idx, _)| hist.counts.get(*idx).copied().unwrap_or(0) > 0)
                        .map(|(idx, _)| {
                            if idx == 0 {
                                0.0
                            } else {
                                hist.boundaries[idx - 1]
                            }
                        })
                        .unwrap_or(0.0);
                    let max = hist
                        .boundaries
                        .iter()
                        .enumerate()
                        .rev()
                        .find(|(idx, _)| hist.counts.get(*idx).copied().unwrap_or(0) > 0)
                        .map(|(_, boundary)| *boundary)
                        .unwrap_or(0.0);

                    (avg as u64, min as u64, max as u64)
                } else {
                    (0, 0, 0)
                }
            } else {
                (0, 0, 0)
            };

        log::info!(
            "{:02}:{:02}:{:02} (+{:?}) | {} processed ({}%), {} successful, {} failed ({}%), {} in queue, avg: {}ms, min: {}ms, max: {}ms",
            elapsed.as_secs() / 3600,
            (elapsed.as_secs() % 3600) / 60,
            elapsed.as_secs() % 60,
            time_since_flush,
            updates_processed,
            if total_updates_received > 0 {
                (updates_processed * 100) / total_updates_received
            } else {
                0
            },
            updates_successful,
            updates_failed,
            if updates_processed > 0 {
                (updates_failed * 100) / updates_processed
            } else {
                0
            },
            updates_queued,
            processing_time_avg,
            processing_time_min,
            processing_time_max
        );

        for (name, _help, value) in &snapshot.counters {
            if !name.starts_with("carbon_") {
                log::info!("{}: {}", name, value);
            }
        }

        for (name, _help, value) in &snapshot.gauges {
            if !name.starts_with("carbon_") {
                log::info!("{}: {}", name, value);
            }
        }

        for (name, _help, hist) in &snapshot.histograms {
            if !name.starts_with("carbon_") {
                let total_count: u64 = hist.counts.iter().sum();
                if total_count > 0 {
                    let sum: f64 = hist
                        .boundaries
                        .iter()
                        .enumerate()
                        .map(|(i, boundary)| {
                            let count = hist.counts.get(i).copied().unwrap_or(0);
                            let midpoint = if i == 0 {
                                0.0
                            } else if i < hist.boundaries.len() {
                                (hist.boundaries[i - 1] + *boundary) / 2.0
                            } else {
                                hist.boundaries.last().copied().unwrap_or(0.0) * 2.0
                            };
                            count as f64 * midpoint
                        })
                        .sum();

                    let avg = sum / total_count as f64;
                    let min = hist
                        .boundaries
                        .iter()
                        .enumerate()
                        .find(|(idx, _)| hist.counts.get(*idx).copied().unwrap_or(0) > 0)
                        .map(|(idx, _)| {
                            if idx == 0 {
                                0.0
                            } else {
                                hist.boundaries[idx - 1]
                            }
                        })
                        .unwrap_or(0.0);
                    let max = hist
                        .boundaries
                        .iter()
                        .enumerate()
                        .rev()
                        .find(|(idx, _)| hist.counts.get(*idx).copied().unwrap_or(0) > 0)
                        .map(|(_, boundary)| *boundary)
                        .unwrap_or(0.0);

                    log::info!(
                        "{} -> avg: {:.2}, min: {:.2}, max: {:.2}",
                        name,
                        avg,
                        min,
                        max
                    );
                }
            }
        }

        *last_flush = Instant::now();
        Ok(())
    }
}
