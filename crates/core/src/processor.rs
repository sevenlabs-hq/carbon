use std::sync::Arc;

use crate::{error::CarbonResult, metrics::Metrics};
use async_trait::async_trait;

#[async_trait]
pub trait Processor {
    type InputType;

    async fn process(
        &mut self,
        data: Self::InputType,
        metrics: Vec<Arc<dyn Metrics>>,
    ) -> CarbonResult<()>;
}
