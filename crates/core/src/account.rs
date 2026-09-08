//! Account decoder and metadata.

use {
    crate::{error::BoxError, update::AccountUpdate},
    solana_pubkey::Pubkey,
    solana_signature::Signature,
};

/// Slot + identity context attached to every decoded account.
#[derive(Debug, Clone)]
pub struct AccountMetadata {
    pub slot: u64,
    pub pubkey: Pubkey,
    pub transaction_signature: Option<Signature>,
}

/// Decodes account data. `Ok(None)` means the account is outside this decoder's scope.
pub trait AccountDecoder {
    type AccountType;

    fn decode_account(
        &self,
        pubkey: &Pubkey,
        account: &solana_account::Account,
    ) -> Result<Option<Self::AccountType>, BoxError>;
}

impl From<&AccountUpdate> for AccountMetadata {
    fn from(update: &AccountUpdate) -> Self {
        Self {
            slot: update.slot(),
            pubkey: *update.pubkey(),
            transaction_signature: update.transaction_signature().copied(),
        }
    }
}
