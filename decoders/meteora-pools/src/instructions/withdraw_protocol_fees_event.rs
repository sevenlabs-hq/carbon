use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d1ef0cfc48bef4f1c")]
pub struct WithdrawProtocolFeesEvent {
    pub pool: solana_sdk::pubkey::Pubkey,
    pub protocol_a_fee: u64,
    pub protocol_b_fee: u64,
    pub protocol_a_fee_owner: solana_sdk::pubkey::Pubkey,
    pub protocol_b_fee_owner: solana_sdk::pubkey::Pubkey,
}
