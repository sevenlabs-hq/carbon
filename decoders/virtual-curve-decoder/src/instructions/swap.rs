use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

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
        let [pool_authority, config, pool, input_token_account, output_token_account, base_vault, quote_vault, base_mint, quote_mint, payer, token_base_program, token_quote_program, referral_token_account, event_authority, program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(SwapInstructionAccounts {
            pool_authority: pool_authority.pubkey,
            config: config.pubkey,
            pool: pool.pubkey,
            input_token_account: input_token_account.pubkey,
            output_token_account: output_token_account.pubkey,
            base_vault: base_vault.pubkey,
            quote_vault: quote_vault.pubkey,
            base_mint: base_mint.pubkey,
            quote_mint: quote_mint.pubkey,
            payer: payer.pubkey,
            token_base_program: token_base_program.pubkey,
            token_quote_program: token_quote_program.pubkey,
            referral_token_account: referral_token_account.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
