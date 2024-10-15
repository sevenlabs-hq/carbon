use crate::error::CarbonResult;
use async_trait::async_trait;

#[async_trait]
pub trait Metrics: Send + Sync {
    async fn initialize(&self) -> CarbonResult<()>;
    async fn flush(&self) -> CarbonResult<()>;
    async fn shutdown(&self) -> CarbonResult<()>;

    async fn update_gauge(&self, name: &str, value: f64) -> CarbonResult<()>;
    async fn increment_counter(&self, name: &str, value: u64) -> CarbonResult<()>;
    async fn record_histogram(&self, name: &str, value: f64) -> CarbonResult<()>;
}
