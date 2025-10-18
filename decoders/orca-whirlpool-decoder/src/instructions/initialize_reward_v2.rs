use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x5b014d32ebe58531")]
pub struct InitializeRewardV2 {
    pub reward_index: u8,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct InitializeRewardV2InstructionAccounts {
    pub reward_authority: solana_pubkey::Pubkey,
    pub funder: solana_pubkey::Pubkey,
    pub whirlpool: solana_pubkey::Pubkey,
    pub reward_mint: solana_pubkey::Pubkey,
    pub reward_token_badge: solana_pubkey::Pubkey,
    pub reward_vault: solana_pubkey::Pubkey,
    pub reward_token_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub rent: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializeRewardV2 {
    type ArrangedAccounts = InitializeRewardV2InstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let reward_authority = next_account(&mut iter)?;
        let funder = next_account(&mut iter)?;
        let whirlpool = next_account(&mut iter)?;
        let reward_mint = next_account(&mut iter)?;
        let reward_token_badge = next_account(&mut iter)?;
        let reward_vault = next_account(&mut iter)?;
        let reward_token_program = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;
        let rent = next_account(&mut iter)?;

        Some(InitializeRewardV2InstructionAccounts {
            reward_authority,
            funder,
            whirlpool,
            reward_mint,
            reward_token_badge,
            reward_vault,
            reward_token_program,
            system_program,
            rent,
        })
    }
}
