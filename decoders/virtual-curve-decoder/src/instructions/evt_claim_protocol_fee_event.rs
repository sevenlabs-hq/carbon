use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1dbaf44bfbbc0d1921")]
pub struct EvtClaimProtocolFeeEvent {
    pub pool: solana_pubkey::Pubkey,
    pub token_base_amount: u64,
    pub token_quote_amount: u64,
}
