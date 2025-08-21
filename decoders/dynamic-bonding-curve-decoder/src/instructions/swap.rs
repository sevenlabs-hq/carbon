use super::super::types::*;

use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xf8c69e91e17587c8")]
pub struct Swap {
    pub params: SwapParameters,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct SwapInstructionAccounts {
    pub pool_authority: solana_pubkey::Pubkey,
    pub config: solana_pubkey::Pubkey,
    pub pool: solana_pubkey::Pubkey,
    pub input_token_account: solana_pubkey::Pubkey,
    pub output_token_account: solana_pubkey::Pubkey,
    pub base_vault: solana_pubkey::Pubkey,
    pub quote_vault: solana_pubkey::Pubkey,
    pub base_mint: solana_pubkey::Pubkey,
    pub quote_mint: solana_pubkey::Pubkey,
    pub payer: solana_pubkey::Pubkey,
    pub token_base_program: solana_pubkey::Pubkey,
    pub token_quote_program: solana_pubkey::Pubkey,
    pub referral_token_account: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Swap {
    type ArrangedAccounts = SwapInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let pool_authority = next_account(&mut iter)?;
        let config = next_account(&mut iter)?;
        let pool = next_account(&mut iter)?;
        let input_token_account = next_account(&mut iter)?;
        let output_token_account = next_account(&mut iter)?;
        let base_vault = next_account(&mut iter)?;
        let quote_vault = next_account(&mut iter)?;
        let base_mint = next_account(&mut iter)?;
        let quote_mint = next_account(&mut iter)?;
        let payer = next_account(&mut iter)?;
        let token_base_program = next_account(&mut iter)?;
        let token_quote_program = next_account(&mut iter)?;
        let referral_token_account = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        Some(SwapInstructionAccounts {
            pool_authority,
            config,
            pool,
            input_token_account,
            output_token_account,
            base_vault,
            quote_vault,
            base_mint,
            quote_mint,
            payer,
            token_base_program,
            token_quote_program,
            referral_token_account,
            event_authority,
            program,
        })
    }
}
