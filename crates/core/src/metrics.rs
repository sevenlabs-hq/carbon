use crate::error::CarbonResult;
use async_trait::async_trait;

#[async_trait]
pub trait Metrics: Send + Sync {
    async fn initialize_metrics(&self) -> CarbonResult<()>;
    async fn flush_metrics(&self) -> CarbonResult<()>;
    async fn shutdown_metrics(&self) -> CarbonResult<()>;

    async fn update_gauge(&self, name: &str, value: f64) -> CarbonResult<()>;
    async fn increment_counter(&self, name: &str, value: u64) -> CarbonResult<()>;
    async fn record_histogram(&self, name: &str, value: f64) -> CarbonResult<()>;
}
