use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d3582975c6d399170")]
pub struct SetDelegateLogEvent {
    pub open_orders_account: solana_sdk::pubkey::Pubkey,
    pub delegate: Option<solana_sdk::pubkey::Pubkey>,
}
