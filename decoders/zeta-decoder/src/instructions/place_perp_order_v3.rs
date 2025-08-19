use {
    super::super::types::*,
    alloc::string::String,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x5bf660073516eae1")]
pub struct PlacePerpOrderV3 {
    pub price: u64,
    pub size: u64,
    pub side: Side,
    pub order_type: OrderType,
    pub client_order_id: Option<u64>,
    pub tag: Option<String>,
    pub tif_offset: Option<u16>,
    pub asset: Asset,
}

pub struct PlacePerpOrderV3InstructionAccounts {
    pub state: solana_pubkey::Pubkey,
    pub pricing: solana_pubkey::Pubkey,
    pub margin_account: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
    pub dex_program: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub serum_authority: solana_pubkey::Pubkey,
    pub open_orders: solana_pubkey::Pubkey,
    pub rent: solana_pubkey::Pubkey,
    pub market_accounts: solana_pubkey::Pubkey,
    pub oracle: solana_pubkey::Pubkey,
    pub oracle_backup_feed: solana_pubkey::Pubkey,
    pub oracle_backup_program: solana_pubkey::Pubkey,
    pub market_mint: solana_pubkey::Pubkey,
    pub mint_authority: solana_pubkey::Pubkey,
    pub perp_sync_queue: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for PlacePerpOrderV3 {
    type ArrangedAccounts = PlacePerpOrderV3InstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, pricing, margin_account, authority, dex_program, token_program, serum_authority, open_orders, rent, market_accounts, oracle, oracle_backup_feed, oracle_backup_program, market_mint, mint_authority, perp_sync_queue, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(PlacePerpOrderV3InstructionAccounts {
            state: state.pubkey,
            pricing: pricing.pubkey,
            margin_account: margin_account.pubkey,
            authority: authority.pubkey,
            dex_program: dex_program.pubkey,
            token_program: token_program.pubkey,
            serum_authority: serum_authority.pubkey,
            open_orders: open_orders.pubkey,
            rent: rent.pubkey,
            market_accounts: market_accounts.pubkey,
            oracle: oracle.pubkey,
            oracle_backup_feed: oracle_backup_feed.pubkey,
            oracle_backup_program: oracle_backup_program.pubkey,
            market_mint: market_mint.pubkey,
            mint_authority: mint_authority.pubkey,
            perp_sync_queue: perp_sync_queue.pubkey,
        })
    }
}
