use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Clone)]
#[carbon(discriminator = "0x181ec828051c0777")]
pub struct Create {
    pub params: CreateParams,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct CreateInstructionAccounts {
    pub payer: solana_pubkey::Pubkey,
    pub owner: solana_pubkey::Pubkey,
    pub token_wallet_authority: solana_pubkey::Pubkey,
    pub mint_a: solana_pubkey::Pubkey,
    pub mint_b: solana_pubkey::Pubkey,
    pub token_wallet_b: solana_pubkey::Pubkey,
    pub pool: solana_pubkey::Pubkey,
    pub vault_a: solana_pubkey::Pubkey,
    pub vault_b: solana_pubkey::Pubkey,
    pub token_program_a: solana_pubkey::Pubkey,
    pub token_program_b: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub rent: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Create {
    type ArrangedAccounts = CreateInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [payer, owner, token_wallet_authority, mint_a, mint_b, token_wallet_b, pool, vault_a, vault_b, token_program_a, token_program_b, system_program, rent, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(CreateInstructionAccounts {
            payer: payer.pubkey,
            owner: owner.pubkey,
            token_wallet_authority: token_wallet_authority.pubkey,
            mint_a: mint_a.pubkey,
            mint_b: mint_b.pubkey,
            token_wallet_b: token_wallet_b.pubkey,
            pool: pool.pubkey,
            vault_a: vault_a.pubkey,
            vault_b: vault_b.pubkey,
            token_program_a: token_program_a.pubkey,
            token_program_b: token_program_b.pubkey,
            system_program: system_program.pubkey,
            rent: rent.pubkey,
        })
    }
}
