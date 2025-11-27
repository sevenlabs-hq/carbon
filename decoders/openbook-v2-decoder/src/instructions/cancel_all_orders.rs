use {
    super::super::types::*,
    carbon_core::{CarbonDeserialize, borsh},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xc453f3ab1164a08f")]
pub struct CancelAllOrders {
    pub side_option: Option<Side>,
    pub limit: u8,
}

pub struct CancelAllOrdersInstructionAccounts {
    pub signer: solana_pubkey::Pubkey,
    pub open_orders_account: solana_pubkey::Pubkey,
    pub market: solana_pubkey::Pubkey,
    pub bids: solana_pubkey::Pubkey,
    pub asks: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CancelAllOrders {
    type ArrangedAccounts = CancelAllOrdersInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [
            signer,
            open_orders_account,
            market,
            bids,
            asks,
            _remaining @ ..,
        ] = accounts
        else {
            return None;
        };

        Some(CancelAllOrdersInstructionAccounts {
            signer: signer.pubkey,
            open_orders_account: open_orders_account.pubkey,
            market: market.pubkey,
            bids: bids.pubkey,
            asks: asks.pubkey,
        })
    }
}
