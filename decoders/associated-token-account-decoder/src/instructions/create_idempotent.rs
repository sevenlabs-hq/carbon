use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x01")]
pub struct CreateIdempotent {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct CreateIdempotentInstructionAccounts {
    pub funding_address: solana_pubkey::Pubkey,
    pub associated_account_address: solana_pubkey::Pubkey,
    pub wallet_address: solana_pubkey::Pubkey,
    pub token_mint_address: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CreateIdempotent {
    type ArrangedAccounts = CreateIdempotentInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [
            funding_address,
            associated_account_address,
            wallet_address,
            token_mint_address,
            system_program,
            token_program,
            _remaining @ ..,
        ] = accounts
        else {
            return None;
        };

        Some(CreateIdempotentInstructionAccounts {
            funding_address: funding_address.pubkey,
            associated_account_address: associated_account_address.pubkey,
            wallet_address: wallet_address.pubkey,
            token_mint_address: token_mint_address.pubkey,
            system_program: system_program.pubkey,
            token_program: token_program.pubkey,
        })
    }
}
