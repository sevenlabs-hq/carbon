use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct WithdrawProtocolFeesEvent {
    pub protocol_fee_recipient: solana_pubkey::Pubkey,
    pub fees_withdrawn: u64,
}
