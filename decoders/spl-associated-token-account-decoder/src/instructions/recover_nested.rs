use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x02")]
pub struct RecoverNested {}

pub struct RecoverNestedInstructionAccounts {
    pub nested_associated_account_address: solana_sdk::pubkey::Pubkey,
    pub nested_token_mint_address: solana_sdk::pubkey::Pubkey,
    pub destination_associated_account_address: solana_sdk::pubkey::Pubkey,
    pub owner_associated_account_address: solana_sdk::pubkey::Pubkey,
    pub owner_token_mint_address: solana_sdk::pubkey::Pubkey,
    pub wallet_address: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for RecoverNested {
    type ArrangedAccounts = RecoverNestedInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [nested_associated_account_address, nested_token_mint_address, destination_associated_account_address, owner_associated_account_address, owner_token_mint_address, wallet_address, token_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(RecoverNestedInstructionAccounts {
            nested_associated_account_address: nested_associated_account_address.pubkey,
            nested_token_mint_address: nested_token_mint_address.pubkey,
            destination_associated_account_address: destination_associated_account_address.pubkey,
            owner_associated_account_address: owner_associated_account_address.pubkey,
            owner_token_mint_address: owner_token_mint_address.pubkey,
            wallet_address: wallet_address.pubkey,
            token_program: token_program.pubkey,
        })
    }
}
