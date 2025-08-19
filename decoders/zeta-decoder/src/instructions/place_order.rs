use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x33c29baf6d82606a")]
pub struct PlaceOrder {
    pub price: u64,
    pub size: u64,
    pub side: Side,
    pub client_order_id: Option<u64>,
}

pub struct PlaceOrderInstructionAccounts {
    pub state: solana_pubkey::Pubkey,
    pub zeta_group: solana_pubkey::Pubkey,
    pub margin_account: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
    pub dex_program: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub serum_authority: solana_pubkey::Pubkey,
    pub greeks: solana_pubkey::Pubkey,
    pub open_orders: solana_pubkey::Pubkey,
    pub rent: solana_pubkey::Pubkey,
    pub market_accounts: solana_pubkey::Pubkey,
    pub oracle: solana_pubkey::Pubkey,
    pub oracle_backup_feed: solana_pubkey::Pubkey,
    pub oracle_backup_program: solana_pubkey::Pubkey,
    pub market_node: solana_pubkey::Pubkey,
    pub market_mint: solana_pubkey::Pubkey,
    pub mint_authority: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for PlaceOrder {
    type ArrangedAccounts = PlaceOrderInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, zeta_group, margin_account, authority, dex_program, token_program, serum_authority, greeks, open_orders, rent, market_accounts, oracle, oracle_backup_feed, oracle_backup_program, market_node, market_mint, mint_authority, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(PlaceOrderInstructionAccounts {
            state: state.pubkey,
            zeta_group: zeta_group.pubkey,
            margin_account: margin_account.pubkey,
            authority: authority.pubkey,
            dex_program: dex_program.pubkey,
            token_program: token_program.pubkey,
            serum_authority: serum_authority.pubkey,
            greeks: greeks.pubkey,
            open_orders: open_orders.pubkey,
            rent: rent.pubkey,
            market_accounts: market_accounts.pubkey,
            oracle: oracle.pubkey,
            oracle_backup_feed: oracle_backup_feed.pubkey,
            oracle_backup_program: oracle_backup_program.pubkey,
            market_node: market_node.pubkey,
            market_mint: market_mint.pubkey,
            mint_authority: mint_authority.pubkey,
        })
    }
}
