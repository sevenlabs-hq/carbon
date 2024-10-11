use indicatif::{ProgressBar, ProgressStyle};
use std::{sync::Arc, time::Duration};
use tokio::{sync::Mutex, time::sleep};

use std::collections::HashMap;

#[derive(Default, Clone)]
pub struct Metrics {
    pub datasource_metrics: HashMap<String, Arc<Mutex<DatasourceMetrics>>>,
    pub pipe_metrics: HashMap<String, Arc<Mutex<PipeMetrics>>>,
}

#[derive(Default, Clone)]
pub struct DatasourceMetrics {
    pub total_updates_received: u64,
    pub total_updates_processed: u64,
    pub average_processing_time: Duration,
}

#[derive(Default, Clone)]
pub struct PipeMetrics {
    pub total_updates_processed: u64,
    pub average_processing_time: Duration,
}

#[derive(Default, Clone)]
pub struct MetricsConfiguration {
    pub interval: tokio::time::Duration,
}

pub struct MetricsData {
    pub successful_txs: Arc<Mutex<usize>>,
    pub failed_txs: Arc<Mutex<usize>>,
    pub pending_txs: Arc<Mutex<usize>>,
    pub pb: ProgressBar,
}

pub fn setup_progress_bars() -> MetricsData {
    let pb = ProgressBar::new(0);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{elapsed_precise} [{bar:40.cyan/blue}] {pos} tx processed ({eta}) {msg}")
            .expect("Invalid progress bar template")
            .progress_chars("##-"),
    );

    let successful_txs = Arc::new(Mutex::new(0usize));
    let failed_txs = Arc::new(Mutex::new(0usize));
    let pending_txs = Arc::new(Mutex::new(0usize));

    MetricsData {
        successful_txs,
        failed_txs,
        pending_txs,
        pb,
    }
}

pub async fn update_progress_bar_timer(metrics_config: Arc<MetricsData>) {
    loop {
        sleep(Duration::from_secs(1)).await;

        let success_count = *metrics_config.successful_txs.lock().await;
        let fail_count = *metrics_config.failed_txs.lock().await;

        metrics_config.pb.set_message(format!(
            "Success: {}, Failed: {}",
            success_count, fail_count
        ));
    }
}
