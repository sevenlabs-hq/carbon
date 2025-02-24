use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x00")]
pub struct Create {}

pub struct CreateInstructionAccounts {
    pub funding_address: solana_sdk::pubkey::Pubkey,
    pub associated_account_address: solana_sdk::pubkey::Pubkey,
    pub wallet_address: solana_sdk::pubkey::Pubkey,
    pub token_mint_address: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Create {
    type ArrangedAccounts = CreateInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [funding_address, associated_account_address, wallet_address, token_mint_address, system_program, token_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(CreateInstructionAccounts {
            funding_address: funding_address.pubkey,
            associated_account_address: associated_account_address.pubkey,
            wallet_address: wallet_address.pubkey,
            token_mint_address: token_mint_address.pubkey,
            system_program: system_program.pubkey,
            token_program: token_program.pubkey,
        })
    }
}
