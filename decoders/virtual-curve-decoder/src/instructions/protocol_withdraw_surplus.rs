use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x3688e18aacb6d6a7")]
pub struct ProtocolWithdrawSurplus {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct ProtocolWithdrawSurplusInstructionAccounts {
    pub pool_authority: solana_pubkey::Pubkey,
    pub config: solana_pubkey::Pubkey,
    pub virtual_pool: solana_pubkey::Pubkey,
    pub token_quote_account: solana_pubkey::Pubkey,
    pub quote_vault: solana_pubkey::Pubkey,
    pub quote_mint: solana_pubkey::Pubkey,
    pub token_quote_program: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ProtocolWithdrawSurplus {
    type ArrangedAccounts = ProtocolWithdrawSurplusInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [pool_authority, config, virtual_pool, token_quote_account, quote_vault, quote_mint, token_quote_program, event_authority, program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(ProtocolWithdrawSurplusInstructionAccounts {
            pool_authority: pool_authority.pubkey,
            config: config.pubkey,
            virtual_pool: virtual_pool.pubkey,
            token_quote_account: token_quote_account.pubkey,
            quote_vault: quote_vault.pubkey,
            quote_mint: quote_mint.pubkey,
            token_quote_program: token_quote_program.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
