use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x18e3e2d45d1af2e6")]
pub struct PruneExpiredTifOrders {}

pub struct PruneExpiredTifOrdersInstructionAccounts {
    pub dex_program: solana_pubkey::Pubkey,
    pub state: solana_pubkey::Pubkey,
    pub serum_authority: solana_pubkey::Pubkey,
    pub market: solana_pubkey::Pubkey,
    pub bids: solana_pubkey::Pubkey,
    pub asks: solana_pubkey::Pubkey,
    pub event_queue: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for PruneExpiredTifOrders {
    type ArrangedAccounts = PruneExpiredTifOrdersInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [dex_program, state, serum_authority, market, bids, asks, event_queue, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(PruneExpiredTifOrdersInstructionAccounts {
            dex_program: dex_program.pubkey,
            state: state.pubkey,
            serum_authority: serum_authority.pubkey,
            market: market.pubkey,
            bids: bids.pubkey,
            asks: asks.pubkey,
            event_queue: event_queue.pubkey,
        })
    }
}
