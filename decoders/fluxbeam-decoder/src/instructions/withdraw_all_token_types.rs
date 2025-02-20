use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xbdfe9caed209a4d8")]
pub struct WithdrawAllTokenTypes {
    pub pool_token_amount: u64,
    pub minimum_token_a_amount: u64,
    pub minimum_token_b_amount: u64,
}

pub struct WithdrawAllTokenTypesInstructionAccounts {
    pub swap: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
    pub user_transfer_authority: solana_sdk::pubkey::Pubkey,
    pub pool_mint: solana_sdk::pubkey::Pubkey,
    pub source: solana_sdk::pubkey::Pubkey,
    pub swap_token_a: solana_sdk::pubkey::Pubkey,
    pub swap_token_b: solana_sdk::pubkey::Pubkey,
    pub destination_token_a: solana_sdk::pubkey::Pubkey,
    pub destination_token_b: solana_sdk::pubkey::Pubkey,
    pub fee_account: solana_sdk::pubkey::Pubkey,
    pub token_a_mint: solana_sdk::pubkey::Pubkey,
    pub token_b_mint: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub token_a_program: solana_sdk::pubkey::Pubkey,
    pub token_b_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for WithdrawAllTokenTypes {
    type ArrangedAccounts = WithdrawAllTokenTypesInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [swap, authority, user_transfer_authority, pool_mint, source, swap_token_a, swap_token_b, destination_token_a, destination_token_b, fee_account, token_a_mint, token_b_mint, token_program, token_a_program, token_b_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(WithdrawAllTokenTypesInstructionAccounts {
            swap: swap.pubkey,
            authority: authority.pubkey,
            user_transfer_authority: user_transfer_authority.pubkey,
            pool_mint: pool_mint.pubkey,
            source: source.pubkey,
            swap_token_a: swap_token_a.pubkey,
            swap_token_b: swap_token_b.pubkey,
            destination_token_a: destination_token_a.pubkey,
            destination_token_b: destination_token_b.pubkey,
            fee_account: fee_account.pubkey,
            token_a_mint: token_a_mint.pubkey,
            token_b_mint: token_b_mint.pubkey,
            token_program: token_program.pubkey,
            token_a_program: token_a_program.pubkey,
            token_b_program: token_b_program.pubkey,
        })
    }
}
