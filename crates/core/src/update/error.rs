/// Invalid data supplied to an update constructor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum UpdateValidationError {
    #[error("transaction has no signature")]
    MissingTransactionSignature,
    #[error("account is not closed: {lamports} lamports remain")]
    AccountNotClosed { lamports: u64 },
}
