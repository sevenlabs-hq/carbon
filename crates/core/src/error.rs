use thiserror::Error;

use crate::datasource::UpdateType;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Missing update type in datasource")]
    MissingUpdateTypeInDatasource(UpdateType),
    #[error("Failed to receive updates({0})")]
    FailedToReceiveUpdates(String),
    #[error("Transaction missing fee payer")]
    MissingFeePayer,
    #[error("Missing inner instructions")]
    MissingInnerInstructions,
    #[error("Missing account in transaction")]
    MissingAccountInTransaction,
    #[error("Missing instruction data")]
    MissingInstructionData,
    #[error("Failed to consume datasource ({0})")]
    FailedToConsumeDatasource(String),
    #[error("Custom error: {0}")]
    Custom(String),
}

pub type CarbonResult<T> = Result<T, Error>;
