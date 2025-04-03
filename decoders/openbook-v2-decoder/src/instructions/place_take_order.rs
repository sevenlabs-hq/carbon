use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x032c47031ac7cb55")]
pub struct PlaceTakeOrder {
    pub args: PlaceTakeOrderArgs,
}

pub struct PlaceTakeOrderInstructionAccounts {
    pub signer: solana_pubkey::Pubkey,
    pub penalty_payer: solana_pubkey::Pubkey,
    pub market: solana_pubkey::Pubkey,
    pub market_authority: solana_pubkey::Pubkey,
    pub bids: solana_pubkey::Pubkey,
    pub asks: solana_pubkey::Pubkey,
    pub market_base_vault: solana_pubkey::Pubkey,
    pub market_quote_vault: solana_pubkey::Pubkey,
    pub event_heap: solana_pubkey::Pubkey,
    pub user_base_account: solana_pubkey::Pubkey,
    pub user_quote_account: solana_pubkey::Pubkey,
    pub oracle_a: solana_pubkey::Pubkey,
    pub oracle_b: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub open_orders_admin: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for PlaceTakeOrder {
    type ArrangedAccounts = PlaceTakeOrderInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [signer, penalty_payer, market, market_authority, bids, asks, market_base_vault, market_quote_vault, event_heap, user_base_account, user_quote_account, oracle_a, oracle_b, token_program, system_program, open_orders_admin, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(PlaceTakeOrderInstructionAccounts {
            signer: signer.pubkey,
            penalty_payer: penalty_payer.pubkey,
            market: market.pubkey,
            market_authority: market_authority.pubkey,
            bids: bids.pubkey,
            asks: asks.pubkey,
            market_base_vault: market_base_vault.pubkey,
            market_quote_vault: market_quote_vault.pubkey,
            event_heap: event_heap.pubkey,
            user_base_account: user_base_account.pubkey,
            user_quote_account: user_quote_account.pubkey,
            oracle_a: oracle_a.pubkey,
            oracle_b: oracle_b.pubkey,
            token_program: token_program.pubkey,
            system_program: system_program.pubkey,
            open_orders_admin: open_orders_admin.pubkey,
        })
    }
}
