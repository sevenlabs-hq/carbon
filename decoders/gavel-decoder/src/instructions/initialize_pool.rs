use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x06")]
pub struct InitializePool {
    pub params: InitializePoolIxParams,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct InitializePoolInstructionAccounts {
    pub plasma_program: solana_pubkey::Pubkey,
    pub log_authority: solana_pubkey::Pubkey,
    pub pool: solana_pubkey::Pubkey,
    pub pool_creator: solana_pubkey::Pubkey,
    pub base_mint: solana_pubkey::Pubkey,
    pub quote_mint: solana_pubkey::Pubkey,
    pub base_vault: solana_pubkey::Pubkey,
    pub quote_vault: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializePool {
    type ArrangedAccounts = InitializePoolInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [plasma_program, log_authority, pool, pool_creator, base_mint, quote_mint, base_vault, quote_vault, system_program, token_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(InitializePoolInstructionAccounts {
            plasma_program: plasma_program.pubkey,
            log_authority: log_authority.pubkey,
            pool: pool.pubkey,
            pool_creator: pool_creator.pubkey,
            base_mint: base_mint.pubkey,
            quote_mint: quote_mint.pubkey,
            base_vault: base_vault.pubkey,
            quote_vault: quote_vault.pubkey,
            system_program: system_program.pubkey,
            token_program: token_program.pubkey,
        })
    }
}
