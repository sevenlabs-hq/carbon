use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xf09ac9c6945d3819")]
pub struct SetRewardAuthorityBySuperAuthority {
    pub reward_index: u8,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct SetRewardAuthorityBySuperAuthorityInstructionAccounts {
    pub whirlpools_config: solana_pubkey::Pubkey,
    pub whirlpool: solana_pubkey::Pubkey,
    pub reward_emissions_super_authority: solana_pubkey::Pubkey,
    pub new_reward_authority: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SetRewardAuthorityBySuperAuthority {
    type ArrangedAccounts = SetRewardAuthorityBySuperAuthorityInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let whirlpools_config = next_account(&mut iter)?;
        let whirlpool = next_account(&mut iter)?;
        let reward_emissions_super_authority = next_account(&mut iter)?;
        let new_reward_authority = next_account(&mut iter)?;

        Some(SetRewardAuthorityBySuperAuthorityInstructionAccounts {
            whirlpools_config,
            whirlpool,
            reward_emissions_super_authority,
            new_reward_authority,
        })
    }
}
