use thiserror::Error;

use crate::datasource::UpdateType;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Missing datasource")]
    MissingDatasource,
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
    #[error("Missing account in instruction")]
    MissingAccountInInstruction,
    #[error("Missing instruction data")]
    MissingInstructionData,
    #[error("Invalid instruction data length")]
    InvalidDataLength,
    #[error("Invalid instruction discriminator")]
    InvalidInstructionDiscriminator,
}

pub type CarbonResult<T> = Result<T, Error>;
