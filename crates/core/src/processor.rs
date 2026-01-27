use {
    crate::{error::CarbonResult, metrics::MetricsCollection},
    async_trait::async_trait,
    std::sync::Arc,
};

#[async_trait]
pub trait Processor {
    type InputType;

    async fn process(
        &mut self,
        data: Self::InputType,
        metrics: Arc<MetricsCollection>,
    ) -> CarbonResult<()>;
}
