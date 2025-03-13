use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct AccountEventHeader {
    pub signer: Option<solana_sdk::pubkey::Pubkey>,
    pub marginfi_account: solana_sdk::pubkey::Pubkey,
    pub marginfi_account_authority: solana_sdk::pubkey::Pubkey,
    pub marginfi_group: solana_sdk::pubkey::Pubkey,
}
