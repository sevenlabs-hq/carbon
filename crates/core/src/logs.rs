use crate::{
    datasource::LogsUpdate, error::CarbonResult, metrics::MetricsCollection, processor::Processor,
};
use async_trait::async_trait;
use std::sync::Arc;

pub struct LogsPipe {
    pub processor: Box<dyn Processor<InputType = LogsUpdate> + Send + Sync>,
}

#[async_trait]
pub trait LogsPipes: Send + Sync {
    async fn run(
        &mut self,
        logs_update: LogsUpdate,
        metrics: Arc<MetricsCollection>,
    ) -> CarbonResult<()>;
}

#[async_trait]
impl LogsPipes for LogsPipe {
    async fn run(
        &mut self,
        logs_update: LogsUpdate,
        metrics: Arc<MetricsCollection>,
    ) -> CarbonResult<()> {
        log::trace!("LogsPipe::run(logs_update: {:?}, metrics)", logs_update,);

        self.processor.process(logs_update, metrics).await?;

        Ok(())
    }
}
