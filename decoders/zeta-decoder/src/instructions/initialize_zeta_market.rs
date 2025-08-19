use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x74efe2952ea3dd03")]
pub struct InitializeZetaMarket {
    pub args: InitializeMarketArgs,
}

pub struct InitializeZetaMarketInstructionAccounts {
    pub state: solana_pubkey::Pubkey,
    pub market_indexes: solana_pubkey::Pubkey,
    pub pricing: solana_pubkey::Pubkey,
    pub admin: solana_pubkey::Pubkey,
    pub market: solana_pubkey::Pubkey,
    pub request_queue: solana_pubkey::Pubkey,
    pub event_queue: solana_pubkey::Pubkey,
    pub bids: solana_pubkey::Pubkey,
    pub asks: solana_pubkey::Pubkey,
    pub base_mint: solana_pubkey::Pubkey,
    pub quote_mint: solana_pubkey::Pubkey,
    pub dex_base_vault: solana_pubkey::Pubkey,
    pub dex_quote_vault: solana_pubkey::Pubkey,
    pub vault_owner: solana_pubkey::Pubkey,
    pub mint_authority: solana_pubkey::Pubkey,
    pub serum_authority: solana_pubkey::Pubkey,
    pub dex_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub rent: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializeZetaMarket {
    type ArrangedAccounts = InitializeZetaMarketInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, market_indexes, pricing, admin, market, request_queue, event_queue, bids, asks, base_mint, quote_mint, dex_base_vault, dex_quote_vault, vault_owner, mint_authority, serum_authority, dex_program, system_program, token_program, rent, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(InitializeZetaMarketInstructionAccounts {
            state: state.pubkey,
            market_indexes: market_indexes.pubkey,
            pricing: pricing.pubkey,
            admin: admin.pubkey,
            market: market.pubkey,
            request_queue: request_queue.pubkey,
            event_queue: event_queue.pubkey,
            bids: bids.pubkey,
            asks: asks.pubkey,
            base_mint: base_mint.pubkey,
            quote_mint: quote_mint.pubkey,
            dex_base_vault: dex_base_vault.pubkey,
            dex_quote_vault: dex_quote_vault.pubkey,
            vault_owner: vault_owner.pubkey,
            mint_authority: mint_authority.pubkey,
            serum_authority: serum_authority.pubkey,
            dex_program: dex_program.pubkey,
            system_program: system_program.pubkey,
            token_program: token_program.pubkey,
            rent: rent.pubkey,
        })
    }
}
