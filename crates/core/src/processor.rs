use async_trait::async_trait;

use crate::error::CarbonResult;

#[async_trait]
pub trait Processor {
    type InputType;

    async fn process(&self, data: Self::InputType) -> CarbonResult<()>;
}
