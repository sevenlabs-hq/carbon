use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1de7bd419566d79af4")]
pub struct WithdrawIneligibleRewardEvent {
    pub lb_pair: solana_sdk::pubkey::Pubkey,
    pub reward_mint: solana_sdk::pubkey::Pubkey,
    pub amount: u64,
}
