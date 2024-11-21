use carbon_core::{borsh, CarbonDeserialize};
#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x72e44820c130a066")]
pub struct SetRewardEmissionsV2 {
    pub reward_index: u8,
    pub emissions_per_second_x64: u128,
}

pub struct SetRewardEmissionsV2InstructionAccounts {
    pub whirlpool: solana_sdk::pubkey::Pubkey,
    pub reward_authority: solana_sdk::pubkey::Pubkey,
    pub reward_vault: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SetRewardEmissionsV2 {
    type ArrangedAccounts = SetRewardEmissionsV2InstructionAccounts;

fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let whirlpool = accounts.get(0)?;
        let reward_authority = accounts.get(1)?;
        let reward_vault = accounts.get(2)?;

        Some(SetRewardEmissionsV2InstructionAccounts {
            whirlpool: whirlpool.pubkey,
            reward_authority: reward_authority.pubkey,
            reward_vault: reward_vault.pubkey,
        })
    }
}
