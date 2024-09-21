use crate::error::CarbonResult;
use async_trait::async_trait;

#[async_trait]
pub trait Processor {
    type InputType;

    async fn process(&self, data: Self::InputType) -> CarbonResult<()>;
}
