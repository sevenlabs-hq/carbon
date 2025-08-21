use super::super::types::*;

use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

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
        let mut iter = accounts.iter();
        let config = next_account(&mut iter)?;
        let pool_authority = next_account(&mut iter)?;
        let creator = next_account(&mut iter)?;
        let base_mint = next_account(&mut iter)?;
        let quote_mint = next_account(&mut iter)?;
        let pool = next_account(&mut iter)?;
        let base_vault = next_account(&mut iter)?;
        let quote_vault = next_account(&mut iter)?;
        let mint_metadata = next_account(&mut iter)?;
        let metadata_program = next_account(&mut iter)?;
        let payer = next_account(&mut iter)?;
        let token_quote_program = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        Some(InitializeVirtualPoolWithSplTokenInstructionAccounts {
            config,
            pool_authority,
            creator,
            base_mint,
            quote_mint,
            pool,
            base_vault,
            quote_vault,
            mint_metadata,
            metadata_program,
            payer,
            token_quote_program,
            token_program,
            system_program,
            event_authority,
            program,
        })
    }
}
