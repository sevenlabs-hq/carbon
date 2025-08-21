use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x4605845756ebb122")]
pub struct CollectReward {
    pub reward_index: u8,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct CollectRewardInstructionAccounts {
    pub whirlpool: solana_pubkey::Pubkey,
    pub position_authority: solana_pubkey::Pubkey,
    pub position: solana_pubkey::Pubkey,
    pub position_token_account: solana_pubkey::Pubkey,
    pub reward_owner_account: solana_pubkey::Pubkey,
    pub reward_vault: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CollectReward {
    type ArrangedAccounts = CollectRewardInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let whirlpool = next_account(&mut iter)?;
        let position_authority = next_account(&mut iter)?;
        let position = next_account(&mut iter)?;
        let position_token_account = next_account(&mut iter)?;
        let reward_owner_account = next_account(&mut iter)?;
        let reward_vault = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;

        Some(CollectRewardInstructionAccounts {
            whirlpool,
            position_authority,
            position,
            position_token_account,
            reward_owner_account,
            reward_vault,
            token_program,
        })
    }
}
