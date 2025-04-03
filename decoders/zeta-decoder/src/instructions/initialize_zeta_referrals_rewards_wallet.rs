use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xf5e5df780786f7f8")]
pub struct InitializeZetaReferralsRewardsWallet {}

pub struct InitializeZetaReferralsRewardsWalletInstructionAccounts {
    pub state: solana_pubkey::Pubkey,
    pub referrals_rewards_wallet: solana_pubkey::Pubkey,
    pub rent: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub usdc_mint: solana_pubkey::Pubkey,
    pub admin: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializeZetaReferralsRewardsWallet {
    type ArrangedAccounts = InitializeZetaReferralsRewardsWalletInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, referrals_rewards_wallet, rent, system_program, token_program, usdc_mint, admin, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(InitializeZetaReferralsRewardsWalletInstructionAccounts {
            state: state.pubkey,
            referrals_rewards_wallet: referrals_rewards_wallet.pubkey,
            rent: rent.pubkey,
            system_program: system_program.pubkey,
            token_program: token_program.pubkey,
            usdc_mint: usdc_mint.pubkey,
            admin: admin.pubkey,
        })
    }
}
