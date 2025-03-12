
use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xe445a52e51cb9a1d703d8c84fb5c5aca")]
pub struct MarginfiAccountTransferAccountAuthorityEvent{
    pub header: AccountEventHeader,
    pub old_account_authority: solana_sdk::pubkey::Pubkey,
    pub new_account_authority: solana_sdk::pubkey::Pubkey,
}
