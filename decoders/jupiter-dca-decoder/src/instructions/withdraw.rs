use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1dc0f1c9d946965af7")]
pub struct Withdraw {
    pub dca_key: solana_sdk::pubkey::Pubkey,
    pub in_amount: u64,
    pub out_amount: u64,
    pub user_withdraw: bool,
}
