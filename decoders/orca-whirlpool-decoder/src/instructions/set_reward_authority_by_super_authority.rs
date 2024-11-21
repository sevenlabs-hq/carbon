use carbon_core::{borsh, CarbonDeserialize};
#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xf09ac9c6945d3819")]
pub struct SetRewardAuthorityBySuperAuthority {
    pub reward_index: u8,
}

pub struct SetRewardAuthorityBySuperAuthorityInstructionAccounts {
    pub whirlpools_config: solana_sdk::pubkey::Pubkey,
    pub whirlpool: solana_sdk::pubkey::Pubkey,
    pub reward_emissions_super_authority: solana_sdk::pubkey::Pubkey,
    pub new_reward_authority: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SetRewardAuthorityBySuperAuthority {
    type ArrangedAccounts = SetRewardAuthorityBySuperAuthorityInstructionAccounts;

fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let whirlpools_config = accounts.get(0)?;
        let whirlpool = accounts.get(1)?;
        let reward_emissions_super_authority = accounts.get(2)?;
        let new_reward_authority = accounts.get(3)?;

        Some(SetRewardAuthorityBySuperAuthorityInstructionAccounts {
            whirlpools_config: whirlpools_config.pubkey,
            whirlpool: whirlpool.pubkey,
            reward_emissions_super_authority: reward_emissions_super_authority.pubkey,
            new_reward_authority: new_reward_authority.pubkey,
        })
    }
}
