use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x1bd59fbf0c747079")]
pub struct PruneOrders {
    pub limit: u8,
}

pub struct PruneOrdersInstructionAccounts {
    pub close_market_admin: solana_sdk::pubkey::Pubkey,
    pub open_orders_account: solana_sdk::pubkey::Pubkey,
    pub market: solana_sdk::pubkey::Pubkey,
    pub bids: solana_sdk::pubkey::Pubkey,
    pub asks: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for PruneOrders {
    type ArrangedAccounts = PruneOrdersInstructionAccounts;

    fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let close_market_admin = accounts.get(0)?;
        let open_orders_account = accounts.get(1)?;
        let market = accounts.get(2)?;
        let bids = accounts.get(3)?;
        let asks = accounts.get(4)?;

        Some(PruneOrdersInstructionAccounts {
            close_market_admin: close_market_admin.pubkey,
            open_orders_account: open_orders_account.pubkey,
            market: market.pubkey,
            bids: bids.pubkey,
            asks: asks.pubkey,
        })
    }
}
