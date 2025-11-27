use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x03")]
pub struct WithdrawAllTokenTypes {
    pub pool_token_amount: u64,
    pub minimum_token_a_amount: u64,
    pub minimum_token_b_amount: u64,
}

#[derive(Debug, PartialEq)]
pub struct WithdrawAllTokenTypesInstructionAccounts {
    pub swap: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
    pub user_transfer_authority: solana_pubkey::Pubkey,
    pub pool_mint: solana_pubkey::Pubkey,
    pub source: solana_pubkey::Pubkey,
    pub swap_token_a: solana_pubkey::Pubkey,
    pub swap_token_b: solana_pubkey::Pubkey,
    pub destination_token_a: solana_pubkey::Pubkey,
    pub destination_token_b: solana_pubkey::Pubkey,
    pub fee_account: solana_pubkey::Pubkey,
    pub token_a_mint: solana_pubkey::Pubkey,
    pub token_b_mint: solana_pubkey::Pubkey,
    pub pool_token_program: solana_pubkey::Pubkey,
    pub token_a_program: solana_pubkey::Pubkey,
    pub token_b_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for WithdrawAllTokenTypes {
    type ArrangedAccounts = WithdrawAllTokenTypesInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [
            swap,
            authority,
            user_transfer_authority,
            pool_mint,
            source,
            swap_token_a,
            swap_token_b,
            destination_token_a,
            destination_token_b,
            fee_account,
            token_a_mint,
            token_b_mint,
            pool_token_program,
            token_a_program,
            token_b_program,
            _remaining @ ..,
        ] = accounts
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
            pool_token_program: pool_token_program.pubkey,
            token_a_program: token_a_program.pubkey,
            token_b_program: token_b_program.pubkey,
        })
    }
}
