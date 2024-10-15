use async_trait::async_trait;
use carbon_core::error::{CarbonResult, Error};
use carbon_core::metrics::Metrics;
use metrics::{counter, gauge, histogram};
use metrics_exporter_prometheus::PrometheusBuilder;
use std::net::SocketAddrV4;
use std::sync::Once;

pub struct PrometheusMetrics;

impl PrometheusMetrics {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Metrics for PrometheusMetrics {
    async fn initialize_metrics(&self) -> CarbonResult<()> {
        static INIT: Once = Once::new();

        let mut result = Ok(());
        INIT.call_once(|| {
            let builder = PrometheusBuilder::new()
                .with_http_listener("127.0.0.1:9100".parse::<SocketAddrV4>().unwrap());

            match builder.install() {
                Ok(_handle) => {
                    log::info!("Prometheus exporter installed and listening on 127.0.0.1:9100");
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

    async fn flush_metrics(&self) -> CarbonResult<()> {
        Ok(())
    }

    async fn shutdown_metrics(&self) -> CarbonResult<()> {
        Ok(())
    }

    async fn update_gauge(&self, name: &str, value: f64) -> CarbonResult<()> {
        gauge!(name.to_string(), value);
        Ok(())
    }

    async fn increment_counter(&self, name: &str, value: u64) -> CarbonResult<()> {
        counter!(name.to_string(), value);
        Ok(())
    }

    async fn record_histogram(&self, name: &str, value: f64) -> CarbonResult<()> {
        histogram!(name.to_string(), value);
        Ok(())
    }
}
