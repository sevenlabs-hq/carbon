use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x9ae6fa0decd14bdf")]
pub struct UpdateFeesAndRewards {}

pub struct UpdateFeesAndRewardsInstructionAccounts {
    pub whirlpool: solana_pubkey::Pubkey,
    pub position: solana_pubkey::Pubkey,
    pub tick_array_lower: solana_pubkey::Pubkey,
    pub tick_array_upper: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateFeesAndRewards {
    type ArrangedAccounts = UpdateFeesAndRewardsInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [whirlpool, position, tick_array_lower, tick_array_upper, _remaining @ ..] = accounts
        else {
            return None;
        };

        Some(UpdateFeesAndRewardsInstructionAccounts {
            whirlpool: whirlpool.pubkey,
            position: position.pubkey,
            tick_array_lower: tick_array_lower.pubkey,
            tick_array_upper: tick_array_upper.pubkey,
        })
    }
}
