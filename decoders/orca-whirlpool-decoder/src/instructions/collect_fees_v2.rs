use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xcf755fbfe5b4e20f")]
pub struct CollectFeesV2 {
    pub remaining_accounts_info: Option<RemainingAccountsInfo>,
}

pub struct CollectFeesV2InstructionAccounts {
    pub whirlpool: solana_sdk::pubkey::Pubkey,
    pub position_authority: solana_sdk::pubkey::Pubkey,
    pub position: solana_sdk::pubkey::Pubkey,
    pub position_token_account: solana_sdk::pubkey::Pubkey,
    pub token_mint_a: solana_sdk::pubkey::Pubkey,
    pub token_mint_b: solana_sdk::pubkey::Pubkey,
    pub token_owner_account_a: solana_sdk::pubkey::Pubkey,
    pub token_vault_a: solana_sdk::pubkey::Pubkey,
    pub token_owner_account_b: solana_sdk::pubkey::Pubkey,
    pub token_vault_b: solana_sdk::pubkey::Pubkey,
    pub token_program_a: solana_sdk::pubkey::Pubkey,
    pub token_program_b: solana_sdk::pubkey::Pubkey,
    pub memo_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CollectFeesV2 {
    type ArrangedAccounts = CollectFeesV2InstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [whirlpool, position_authority, position, position_token_account, token_mint_a, token_mint_b, token_owner_account_a, token_vault_a, token_owner_account_b, token_vault_b, token_program_a, token_program_b, memo_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(CollectFeesV2InstructionAccounts {
            whirlpool: whirlpool.pubkey,
            position_authority: position_authority.pubkey,
            position: position.pubkey,
            position_token_account: position_token_account.pubkey,
            token_mint_a: token_mint_a.pubkey,
            token_mint_b: token_mint_b.pubkey,
            token_owner_account_a: token_owner_account_a.pubkey,
            token_vault_a: token_vault_a.pubkey,
            token_owner_account_b: token_owner_account_b.pubkey,
            token_vault_b: token_vault_b.pubkey,
            token_program_a: token_program_a.pubkey,
            token_program_b: token_program_b.pubkey,
            memo_program: memo_program.pubkey,
        })
    }
}
