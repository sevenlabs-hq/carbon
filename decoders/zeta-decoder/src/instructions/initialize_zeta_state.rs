use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x44274b8ebf925ede")]
pub struct InitializeZetaState {
    pub args: InitializeStateArgs,
}

pub struct InitializeZetaStateInstructionAccounts {
    pub state: solana_pubkey::Pubkey,
    pub mint_authority: solana_pubkey::Pubkey,
    pub serum_authority: solana_pubkey::Pubkey,
    pub treasury_wallet: solana_pubkey::Pubkey,
    pub referrals_admin: solana_pubkey::Pubkey,
    pub referrals_rewards_wallet: solana_pubkey::Pubkey,
    pub rent: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub usdc_mint: solana_pubkey::Pubkey,
    pub admin: solana_pubkey::Pubkey,
    pub secondary_admin: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializeZetaState {
    type ArrangedAccounts = InitializeZetaStateInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, mint_authority, serum_authority, treasury_wallet, referrals_admin, referrals_rewards_wallet, rent, system_program, token_program, usdc_mint, admin, secondary_admin, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(InitializeZetaStateInstructionAccounts {
            state: state.pubkey,
            mint_authority: mint_authority.pubkey,
            serum_authority: serum_authority.pubkey,
            treasury_wallet: treasury_wallet.pubkey,
            referrals_admin: referrals_admin.pubkey,
            referrals_rewards_wallet: referrals_rewards_wallet.pubkey,
            rent: rent.pubkey,
            system_program: system_program.pubkey,
            token_program: token_program.pubkey,
            usdc_mint: usdc_mint.pubkey,
            admin: admin.pubkey,
            secondary_admin: secondary_admin.pubkey,
        })
    }
}
