use carbon_core::{borsh, CarbonDeserialize};
#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xd0d450a92494d123")]
pub struct OneIntroSwap {}

pub struct OneIntroSwapInstructionAccounts {
    pub swap_program: solana_sdk::pubkey::Pubkey,
    pub metadata_state: solana_sdk::pubkey::Pubkey,
    pub pool_state: solana_sdk::pubkey::Pubkey,
    pub pool_auth_pda: solana_sdk::pubkey::Pubkey,
    pub pool_token_in_account: solana_sdk::pubkey::Pubkey,
    pub pool_token_out_account: solana_sdk::pubkey::Pubkey,
    pub user: solana_sdk::pubkey::Pubkey,
    pub user_token_in_account: solana_sdk::pubkey::Pubkey,
    pub user_token_out_account: solana_sdk::pubkey::Pubkey,
    pub metadata_swap_fee_account: solana_sdk::pubkey::Pubkey,
    pub referral_token_account: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for OneIntroSwap {
    type ArrangedAccounts = OneIntroSwapInstructionAccounts;

fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let swap_program = accounts.get(0)?;
        let metadata_state = accounts.get(1)?;
        let pool_state = accounts.get(2)?;
        let pool_auth_pda = accounts.get(3)?;
        let pool_token_in_account = accounts.get(4)?;
        let pool_token_out_account = accounts.get(5)?;
        let user = accounts.get(6)?;
        let user_token_in_account = accounts.get(7)?;
        let user_token_out_account = accounts.get(8)?;
        let metadata_swap_fee_account = accounts.get(9)?;
        let referral_token_account = accounts.get(10)?;
        let token_program = accounts.get(11)?;

        Some(OneIntroSwapInstructionAccounts {
            swap_program: swap_program.pubkey,
            metadata_state: metadata_state.pubkey,
            pool_state: pool_state.pubkey,
            pool_auth_pda: pool_auth_pda.pubkey,
            pool_token_in_account: pool_token_in_account.pubkey,
            pool_token_out_account: pool_token_out_account.pubkey,
            user: user.pubkey,
            user_token_in_account: user_token_in_account.pubkey,
            user_token_out_account: user_token_out_account.pubkey,
            metadata_swap_fee_account: metadata_swap_fee_account.pubkey,
            referral_token_account: referral_token_account.pubkey,
            token_program: token_program.pubkey,
        })
    }
}
