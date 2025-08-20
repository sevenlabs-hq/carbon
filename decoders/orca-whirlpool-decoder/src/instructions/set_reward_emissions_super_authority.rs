use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xcf05c8d17a3852b7")]
pub struct SetRewardEmissionsSuperAuthority {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct SetRewardEmissionsSuperAuthorityInstructionAccounts {
    pub whirlpools_config: solana_pubkey::Pubkey,
    pub reward_emissions_super_authority: solana_pubkey::Pubkey,
    pub new_reward_emissions_super_authority: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SetRewardEmissionsSuperAuthority {
    type ArrangedAccounts = SetRewardEmissionsSuperAuthorityInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let whirlpools_config = next_account(&mut iter)?;
        let reward_emissions_super_authority = next_account(&mut iter)?;
        let new_reward_emissions_super_authority = next_account(&mut iter)?;

        Some(SetRewardEmissionsSuperAuthorityInstructionAccounts {
            whirlpools_config,
            reward_emissions_super_authority,
            new_reward_emissions_super_authority,
        })
    }
}
