use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xcc257c399ed3b2d1")]
pub struct PruneExpiredTifOrdersV2 {
    pub limit: u16,
}

pub struct PruneExpiredTifOrdersV2InstructionAccounts {
    pub dex_program: solana_pubkey::Pubkey,
    pub state: solana_pubkey::Pubkey,
    pub serum_authority: solana_pubkey::Pubkey,
    pub market: solana_pubkey::Pubkey,
    pub bids: solana_pubkey::Pubkey,
    pub asks: solana_pubkey::Pubkey,
    pub event_queue: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for PruneExpiredTifOrdersV2 {
    type ArrangedAccounts = PruneExpiredTifOrdersV2InstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [dex_program, state, serum_authority, market, bids, asks, event_queue, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(PruneExpiredTifOrdersV2InstructionAccounts {
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
