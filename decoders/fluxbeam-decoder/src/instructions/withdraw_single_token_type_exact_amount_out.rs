use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xf3c0468bd19c088b")]
pub struct WithdrawSingleTokenTypeExactAmountOut {
    pub destination_token_amount: u64,
    pub maximum_pool_token_amount: u64,
}

pub struct WithdrawSingleTokenTypeExactAmountOutInstructionAccounts {
    pub swap: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
    pub user_transfer_authority: solana_sdk::pubkey::Pubkey,
    pub pool_mint: solana_sdk::pubkey::Pubkey,
    pub pool_token_source: solana_sdk::pubkey::Pubkey,
    pub swap_token_a: solana_sdk::pubkey::Pubkey,
    pub swap_token_b: solana_sdk::pubkey::Pubkey,
    pub destination: solana_sdk::pubkey::Pubkey,
    pub fee_account: solana_sdk::pubkey::Pubkey,
    pub destination_mint: solana_sdk::pubkey::Pubkey,
    pub token_a_program: solana_sdk::pubkey::Pubkey,
    pub token_b_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for WithdrawSingleTokenTypeExactAmountOut {
    type ArrangedAccounts = WithdrawSingleTokenTypeExactAmountOutInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [swap, authority, user_transfer_authority, pool_mint, pool_token_source, swap_token_a, swap_token_b, destination, fee_account, destination_mint, token_a_program, token_b_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(WithdrawSingleTokenTypeExactAmountOutInstructionAccounts {
            swap: swap.pubkey,
            authority: authority.pubkey,
            user_transfer_authority: user_transfer_authority.pubkey,
            pool_mint: pool_mint.pubkey,
            pool_token_source: pool_token_source.pubkey,
            swap_token_a: swap_token_a.pubkey,
            swap_token_b: swap_token_b.pubkey,
            destination: destination.pubkey,
            fee_account: fee_account.pubkey,
            destination_mint: destination_mint.pubkey,
            token_a_program: token_a_program.pubkey,
            token_b_program: token_b_program.pubkey,
        })
    }
}
