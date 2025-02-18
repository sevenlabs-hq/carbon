use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x73b2c908afb77b77")]
pub struct CancelOrderByClientOrderId {
    pub client_order_id: u64,
}

pub struct CancelOrderByClientOrderIdInstructionAccounts {
    pub signer: solana_sdk::pubkey::Pubkey,
    pub open_orders_account: solana_sdk::pubkey::Pubkey,
    pub market: solana_sdk::pubkey::Pubkey,
    pub bids: solana_sdk::pubkey::Pubkey,
    pub asks: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CancelOrderByClientOrderId {
    type ArrangedAccounts = CancelOrderByClientOrderIdInstructionAccounts;

    fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let signer = accounts.get(0)?;
        let open_orders_account = accounts.get(1)?;
        let market = accounts.get(2)?;
        let bids = accounts.get(3)?;
        let asks = accounts.get(4)?;

        Some(CancelOrderByClientOrderIdInstructionAccounts {
            signer: signer.pubkey,
            open_orders_account: open_orders_account.pubkey,
            market: market.pubkey,
            bids: bids.pubkey,
            asks: asks.pubkey,
        })
    }
}
