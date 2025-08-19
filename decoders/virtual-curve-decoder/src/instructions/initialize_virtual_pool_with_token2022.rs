use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xa976334e916edc9b")]
pub struct InitializeVirtualPoolWithToken2022 {
    pub params: InitializePoolParameters,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct InitializeVirtualPoolWithToken2022InstructionAccounts {
    pub config: solana_pubkey::Pubkey,
    pub pool_authority: solana_pubkey::Pubkey,
    pub creator: solana_pubkey::Pubkey,
    pub base_mint: solana_pubkey::Pubkey,
    pub quote_mint: solana_pubkey::Pubkey,
    pub pool: solana_pubkey::Pubkey,
    pub base_vault: solana_pubkey::Pubkey,
    pub quote_vault: solana_pubkey::Pubkey,
    pub payer: solana_pubkey::Pubkey,
    pub token_quote_program: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializeVirtualPoolWithToken2022 {
    type ArrangedAccounts = InitializeVirtualPoolWithToken2022InstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [config, pool_authority, creator, base_mint, quote_mint, pool, base_vault, quote_vault, payer, token_quote_program, token_program, system_program, event_authority, program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(InitializeVirtualPoolWithToken2022InstructionAccounts {
            config: config.pubkey,
            pool_authority: pool_authority.pubkey,
            creator: creator.pubkey,
            base_mint: base_mint.pubkey,
            quote_mint: quote_mint.pubkey,
            pool: pool.pubkey,
            base_vault: base_vault.pubkey,
            quote_vault: quote_vault.pubkey,
            payer: payer.pubkey,
            token_quote_program: token_quote_program.pubkey,
            token_program: token_program.pubkey,
            system_program: system_program.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
