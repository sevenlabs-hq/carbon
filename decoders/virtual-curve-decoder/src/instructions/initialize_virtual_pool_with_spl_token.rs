use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x8c55d7b06636684f")]
pub struct InitializeVirtualPoolWithSplToken {
    pub params: InitializePoolParameters,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct InitializeVirtualPoolWithSplTokenInstructionAccounts {
    pub config: solana_pubkey::Pubkey,
    pub pool_authority: solana_pubkey::Pubkey,
    pub creator: solana_pubkey::Pubkey,
    pub base_mint: solana_pubkey::Pubkey,
    pub quote_mint: solana_pubkey::Pubkey,
    pub pool: solana_pubkey::Pubkey,
    pub base_vault: solana_pubkey::Pubkey,
    pub quote_vault: solana_pubkey::Pubkey,
    pub mint_metadata: solana_pubkey::Pubkey,
    pub metadata_program: solana_pubkey::Pubkey,
    pub payer: solana_pubkey::Pubkey,
    pub token_quote_program: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializeVirtualPoolWithSplToken {
    type ArrangedAccounts = InitializeVirtualPoolWithSplTokenInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [config, pool_authority, creator, base_mint, quote_mint, pool, base_vault, quote_vault, mint_metadata, metadata_program, payer, token_quote_program, token_program, system_program, event_authority, program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(InitializeVirtualPoolWithSplTokenInstructionAccounts {
            config: config.pubkey,
            pool_authority: pool_authority.pubkey,
            creator: creator.pubkey,
            base_mint: base_mint.pubkey,
            quote_mint: quote_mint.pubkey,
            pool: pool.pubkey,
            base_vault: base_vault.pubkey,
            quote_vault: quote_vault.pubkey,
            mint_metadata: mint_metadata.pubkey,
            metadata_program: metadata_program.pubkey,
            payer: payer.pubkey,
            token_quote_program: token_quote_program.pubkey,
            token_program: token_program.pubkey,
            system_program: system_program.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
