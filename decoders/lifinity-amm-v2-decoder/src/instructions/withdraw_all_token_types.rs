use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xbdfe9caed209a4d8")]
pub struct WithdrawAllTokenTypes {
    pub pool_token_amount: u64,
    pub minimum_token_a_amount: u64,
    pub minimum_token_b_amount: u64,
}

#[derive(Debug, PartialEq)]
pub struct WithdrawAllTokenTypesInstructionAccounts {
    pub amm: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
    pub user_transfer_authority_info: solana_pubkey::Pubkey,
    pub source_info: solana_pubkey::Pubkey,
    pub token_a: solana_pubkey::Pubkey,
    pub token_b: solana_pubkey::Pubkey,
    pub pool_mint: solana_pubkey::Pubkey,
    pub dest_token_a_info: solana_pubkey::Pubkey,
    pub dest_token_b_info: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for WithdrawAllTokenTypes {
    type ArrangedAccounts = WithdrawAllTokenTypesInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [
            amm,
            authority,
            user_transfer_authority_info,
            source_info,
            token_a,
            token_b,
            pool_mint,
            dest_token_a_info,
            dest_token_b_info,
            token_program,
            _remaining @ ..,
        ] = accounts
        else {
            return None;
        };

        Some(WithdrawAllTokenTypesInstructionAccounts {
            amm: amm.pubkey,
            authority: authority.pubkey,
            user_transfer_authority_info: user_transfer_authority_info.pubkey,
            source_info: source_info.pubkey,
            token_a: token_a.pubkey,
            token_b: token_b.pubkey,
            pool_mint: pool_mint.pubkey,
            dest_token_a_info: dest_token_a_info.pubkey,
            dest_token_b_info: dest_token_b_info.pubkey,
            token_program: token_program.pubkey,
        })
    }
}
