use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x63cf042a589d2d37")]
pub struct DepositSingleTokenTypeExactAmountIn {
    pub source_token_amount: u64,
    pub minimum_pool_token_amount: u64,
}

pub struct DepositSingleTokenTypeExactAmountInInstructionAccounts {
    pub swap: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
    pub user_transfer_authority: solana_sdk::pubkey::Pubkey,
    pub source_token: solana_sdk::pubkey::Pubkey,
    pub swap_token_a: solana_sdk::pubkey::Pubkey,
    pub swap_token_b: solana_sdk::pubkey::Pubkey,
    pub pool_mint: solana_sdk::pubkey::Pubkey,
    pub destination: solana_sdk::pubkey::Pubkey,
    pub source_mint: solana_sdk::pubkey::Pubkey,
    pub token_a_program: solana_sdk::pubkey::Pubkey,
    pub token_b_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for DepositSingleTokenTypeExactAmountIn {
    type ArrangedAccounts = DepositSingleTokenTypeExactAmountInInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [swap, authority, user_transfer_authority, source_token, swap_token_a, swap_token_b, pool_mint, destination, source_mint, token_a_program, token_b_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(DepositSingleTokenTypeExactAmountInInstructionAccounts {
            swap: swap.pubkey,
            authority: authority.pubkey,
            user_transfer_authority: user_transfer_authority.pubkey,
            source_token: source_token.pubkey,
            swap_token_a: swap_token_a.pubkey,
            swap_token_b: swap_token_b.pubkey,
            pool_mint: pool_mint.pubkey,
            destination: destination.pubkey,
            source_mint: source_mint.pubkey,
            token_a_program: token_a_program.pubkey,
            token_b_program: token_b_program.pubkey,
        })
    }
}
