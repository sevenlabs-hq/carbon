use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x00")]
pub struct Swap {
    pub params: SwapIxParams,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct SwapInstructionAccounts {
    pub plasma_program: solana_pubkey::Pubkey,
    pub log_authority: solana_pubkey::Pubkey,
    pub pool: solana_pubkey::Pubkey,
    pub trader: solana_pubkey::Pubkey,
    pub base_account: solana_pubkey::Pubkey,
    pub quote_account: solana_pubkey::Pubkey,
    pub base_vault: solana_pubkey::Pubkey,
    pub quote_vault: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Swap {
    type ArrangedAccounts = SwapInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [plasma_program, log_authority, pool, trader, base_account, quote_account, base_vault, quote_vault, token_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(SwapInstructionAccounts {
            plasma_program: plasma_program.pubkey,
            log_authority: log_authority.pubkey,
            pool: pool.pubkey,
            trader: trader.pubkey,
            base_account: base_account.pubkey,
            quote_account: quote_account.pubkey,
            base_vault: base_vault.pubkey,
            quote_vault: quote_vault.pubkey,
            token_program: token_program.pubkey,
        })
    }
}
