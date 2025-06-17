use std::net::SocketAddr;
use {
    async_trait::async_trait,
    carbon_core::{
        error::{CarbonResult, Error},
        metrics::Metrics,
    },
    metrics::{counter, gauge, histogram},
    metrics_exporter_prometheus::PrometheusBuilder,
    std::{collections::HashMap, sync::Once},
    tokio::sync::RwLock,
};

pub struct PrometheusMetrics {
    pub counters: RwLock<HashMap<String, metrics::Counter>>,
    pub gauges: RwLock<HashMap<String, metrics::Gauge>>,
    pub histograms: RwLock<HashMap<String, metrics::Histogram>>,
    pub listen_port: u16,
}

impl Default for PrometheusMetrics {
    fn default() -> Self {
        Self {
            counters: RwLock::new(HashMap::new()),
            gauges: RwLock::new(HashMap::new()),
            histograms: RwLock::new(HashMap::new()),
            listen_port: 9100,
        }
    }
}
impl PrometheusMetrics {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn new_with_port(listen_port: u16) -> Self {
        Self {
            gauges: RwLock::new(HashMap::new()),
            counters: RwLock::new(HashMap::new()),
            histograms: RwLock::new(HashMap::new()),
            listen_port,
        }
    }
}

#[async_trait]
impl Metrics for PrometheusMetrics {
    async fn initialize(&self) -> CarbonResult<()> {
        static INIT: Once = Once::new();

        let mut result = Ok(());
        INIT.call_once(|| {
            let addr = format!("127.0.0.1:{}", self.listen_port)
                .parse::<SocketAddr>()
                .expect("Failed to parse address");

            let builder = PrometheusBuilder::new().with_http_listener(addr);

            match builder.install() {
                Ok(_handle) => {
                    log::info!("Prometheus exporter installed and listening on {}", addr);
                }
                Err(e) => {
                    result = Err(Error::Custom(format!(
                        "Failed to install Prometheus exporter: {}",
                        e
                    )));
                }
            }
        });
        result
    }

    async fn flush(&self) -> CarbonResult<()> {
        Ok(())
    }

    async fn shutdown(&self) -> CarbonResult<()> {
        Ok(())
    }

    async fn update_gauge(&self, name: &str, value: f64) -> CarbonResult<()> {
        let mut gauge = self.gauges.write().await;

        if let Some(gauge) = gauge.get(name) {
            gauge.set(value);
        } else {
            let new_gauge = gauge!(name.to_string());
            new_gauge.set(value);
            gauge.insert(name.to_string(), new_gauge);
        }

        Ok(())
    }

    async fn increment_counter(&self, name: &str, value: u64) -> CarbonResult<()> {
        let mut counter = self.counters.write().await;

        if let Some(counter) = counter.get(name) {
            counter.increment(value);
        } else {
            let new_counter = counter!(name.to_string());
            new_counter.increment(value);
            counter.insert(name.to_string(), new_counter);
        }

        Ok(())
    }

    async fn record_histogram(&self, name: &str, value: f64) -> CarbonResult<()> {
        let mut histogram = self.histograms.write().await;

        if let Some(histogram) = histogram.get(name) {
            histogram.record(value);
        } else {
            let new_histogram = histogram!(name.to_string());
            new_histogram.record(value);
            histogram.insert(name.to_string(), new_histogram);
        }

        Ok(())
    }
}
