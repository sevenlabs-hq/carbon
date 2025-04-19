use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xa8ad4864c962265c")]
pub struct PartnerWithdrawSurplus {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct PartnerWithdrawSurplusInstructionAccounts {
    pub pool_authority: solana_pubkey::Pubkey,
    pub config: solana_pubkey::Pubkey,
    pub virtual_pool: solana_pubkey::Pubkey,
    pub token_quote_account: solana_pubkey::Pubkey,
    pub quote_vault: solana_pubkey::Pubkey,
    pub quote_mint: solana_pubkey::Pubkey,
    pub fee_claimer: solana_pubkey::Pubkey,
    pub token_quote_program: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for PartnerWithdrawSurplus {
    type ArrangedAccounts = PartnerWithdrawSurplusInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [pool_authority, config, virtual_pool, token_quote_account, quote_vault, quote_mint, fee_claimer, token_quote_program, event_authority, program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(PartnerWithdrawSurplusInstructionAccounts {
            pool_authority: pool_authority.pubkey,
            config: config.pubkey,
            virtual_pool: virtual_pool.pubkey,
            token_quote_account: token_quote_account.pubkey,
            quote_vault: quote_vault.pubkey,
            quote_mint: quote_mint.pubkey,
            fee_claimer: fee_claimer.pubkey,
            token_quote_program: token_quote_program.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
