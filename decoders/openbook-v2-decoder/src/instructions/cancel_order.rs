use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x5f81edf00831df84")]
pub struct CancelOrder {
    pub order_id: u128,
}

pub struct CancelOrderInstructionAccounts {
    pub signer: solana_pubkey::Pubkey,
    pub open_orders_account: solana_pubkey::Pubkey,
    pub market: solana_pubkey::Pubkey,
    pub bids: solana_pubkey::Pubkey,
    pub asks: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CancelOrder {
    type ArrangedAccounts = CancelOrderInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [signer, open_orders_account, market, bids, asks, _remaining @ ..] = accounts else {
            return None;
        };

        Some(CancelOrderInstructionAccounts {
            signer: signer.pubkey,
            open_orders_account: open_orders_account.pubkey,
            market: market.pubkey,
            bids: bids.pubkey,
            asks: asks.pubkey,
        })
    }
}
