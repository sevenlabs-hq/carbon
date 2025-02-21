use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x205f453c4b4fcdee")]
pub struct DepositAllTokenTypes {
    pub pool_token_amount: u64,
    pub maximum_token_a_amount: u64,
    pub maximum_token_b_amount: u64,
}

pub struct DepositAllTokenTypesInstructionAccounts {
    pub amm: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
    pub user_transfer_authority_info: solana_sdk::pubkey::Pubkey,
    pub source_a_info: solana_sdk::pubkey::Pubkey,
    pub source_b_info: solana_sdk::pubkey::Pubkey,
    pub token_a: solana_sdk::pubkey::Pubkey,
    pub token_b: solana_sdk::pubkey::Pubkey,
    pub pool_mint: solana_sdk::pubkey::Pubkey,
    pub destination: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for DepositAllTokenTypes {
    type ArrangedAccounts = DepositAllTokenTypesInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [amm, authority, user_transfer_authority_info, source_a_info, source_b_info, token_a, token_b, pool_mint, destination, token_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(DepositAllTokenTypesInstructionAccounts {
            amm: amm.pubkey,
            authority: authority.pubkey,
            user_transfer_authority_info: user_transfer_authority_info.pubkey,
            source_a_info: source_a_info.pubkey,
            source_b_info: source_b_info.pubkey,
            token_a: token_a.pubkey,
            token_b: token_b.pubkey,
            pool_mint: pool_mint.pubkey,
            destination: destination.pubkey,
            token_program: token_program.pubkey,
        })
    }
}
