use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x5b014d32ebe58531")]
pub struct InitializeRewardV2 {
    pub reward_index: u8,
}

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
        let [reward_authority, funder, whirlpool, reward_mint, reward_token_badge, reward_vault, reward_token_program, system_program, rent, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(InitializeRewardV2InstructionAccounts {
            reward_authority: reward_authority.pubkey,
            funder: funder.pubkey,
            whirlpool: whirlpool.pubkey,
            reward_mint: reward_mint.pubkey,
            reward_token_badge: reward_token_badge.pubkey,
            reward_vault: reward_vault.pubkey,
            reward_token_program: reward_token_program.pubkey,
            system_program: system_program.pubkey,
            rent: rent.pubkey,
        })
    }
}
