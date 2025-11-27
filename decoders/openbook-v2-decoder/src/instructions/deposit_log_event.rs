use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d8dbaa8fc6c8d485e")]
pub struct DepositLogEvent {
    pub open_orders_account: solana_pubkey::Pubkey,
    pub signer: solana_pubkey::Pubkey,
    pub base_amount: u64,
    pub quote_amount: u64,
}
