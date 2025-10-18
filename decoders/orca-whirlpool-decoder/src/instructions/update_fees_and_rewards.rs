use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x9ae6fa0decd14bdf")]
pub struct UpdateFeesAndRewards {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct UpdateFeesAndRewardsInstructionAccounts {
    pub whirlpool: solana_pubkey::Pubkey,
    pub position: solana_pubkey::Pubkey,
    pub tick_array_lower: solana_pubkey::Pubkey,
    pub tick_array_upper: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateFeesAndRewards {
    type ArrangedAccounts = UpdateFeesAndRewardsInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let whirlpool = next_account(&mut iter)?;
        let position = next_account(&mut iter)?;
        let tick_array_lower = next_account(&mut iter)?;
        let tick_array_upper = next_account(&mut iter)?;

        Some(UpdateFeesAndRewardsInstructionAccounts {
            whirlpool,
            position,
            tick_array_lower,
            tick_array_upper,
        })
    }
}
