//! Transaction metadata.

pub mod rpc;

use {solana_hash::Hash, solana_pubkey::Pubkey, solana_signature::Signature};

/// Per-transaction context shared across all of its instructions.
///
/// Built from a `TransactionUpdate` via `TryFrom`. Wrapped in `Arc`
/// inside the pipeline so child instructions cheaply reference it.
#[derive(Debug, Clone, Default)]
pub struct TransactionMetadata {
    pub slot: u64,
    pub signature: Signature,
    pub fee_payer: Pubkey,
    pub meta: solana_transaction_status::TransactionStatusMeta,
    pub message: solana_message::VersionedMessage,
    pub index: Option<u64>,
    pub block_time: Option<i64>,
    pub block_hash: Option<Hash>,
}

impl TryFrom<crate::update::TransactionUpdate> for TransactionMetadata {
    type Error = crate::error::Error;

    fn try_from(value: crate::update::TransactionUpdate) -> Result<Self, Self::Error> {
        let accounts = value.transaction().message.static_account_keys();

        Ok(TransactionMetadata {
            slot: value.slot(),
            signature: *value.signature(),
            fee_payer: *accounts
                .first()
                .ok_or(crate::error::Error::MissingFeePayer)?,
            meta: value.meta().clone(),
            message: value.transaction().message.clone(),
            index: value.index(),
            block_time: value.block_time(),
            block_hash: value.block_hash().copied(),
        })
    }
}
