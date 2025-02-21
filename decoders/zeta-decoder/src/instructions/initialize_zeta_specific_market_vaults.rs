use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xf945ba92886b4f71")]
pub struct InitializeZetaSpecificMarketVaults {
    pub asset: Asset,
}

pub struct InitializeZetaSpecificMarketVaultsInstructionAccounts {
    pub state: solana_sdk::pubkey::Pubkey,
    pub market_indexes: solana_sdk::pubkey::Pubkey,
    pub pricing: solana_sdk::pubkey::Pubkey,
    pub admin: solana_sdk::pubkey::Pubkey,
    pub market: solana_sdk::pubkey::Pubkey,
    pub base_mint: solana_sdk::pubkey::Pubkey,
    pub quote_mint: solana_sdk::pubkey::Pubkey,
    pub zeta_base_vault: solana_sdk::pubkey::Pubkey,
    pub zeta_quote_vault: solana_sdk::pubkey::Pubkey,
    pub serum_authority: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializeZetaSpecificMarketVaults {
    type ArrangedAccounts = InitializeZetaSpecificMarketVaultsInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, market_indexes, pricing, admin, market, base_mint, quote_mint, zeta_base_vault, zeta_quote_vault, serum_authority, system_program, token_program, rent, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(InitializeZetaSpecificMarketVaultsInstructionAccounts {
            state: state.pubkey,
            market_indexes: market_indexes.pubkey,
            pricing: pricing.pubkey,
            admin: admin.pubkey,
            market: market.pubkey,
            base_mint: base_mint.pubkey,
            quote_mint: quote_mint.pubkey,
            zeta_base_vault: zeta_base_vault.pubkey,
            zeta_quote_vault: zeta_quote_vault.pubkey,
            serum_authority: serum_authority.pubkey,
            system_program: system_program.pubkey,
            token_program: token_program.pubkey,
            rent: rent.pubkey,
        })
    }
}
