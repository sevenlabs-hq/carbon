use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x2227b7fc531c557f")]
pub struct SetRewardAuthority {
    pub reward_index: u8,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct SetRewardAuthorityInstructionAccounts {
    pub whirlpool: solana_pubkey::Pubkey,
    pub reward_authority: solana_pubkey::Pubkey,
    pub new_reward_authority: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SetRewardAuthority {
    type ArrangedAccounts = SetRewardAuthorityInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let whirlpool = next_account(&mut iter)?;
        let reward_authority = next_account(&mut iter)?;
        let new_reward_authority = next_account(&mut iter)?;

        Some(SetRewardAuthorityInstructionAccounts {
            whirlpool,
            reward_authority,
            new_reward_authority,
        })
    }
}
