use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xd1e336046dac2947")]
pub struct ConsumeGivenEvents {
    pub slots: Vec<u64>,
}

pub struct ConsumeGivenEventsInstructionAccounts {
    pub consume_events_admin: solana_sdk::pubkey::Pubkey,
    pub market: solana_sdk::pubkey::Pubkey,
    pub event_heap: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ConsumeGivenEvents {
    type ArrangedAccounts = ConsumeGivenEventsInstructionAccounts;

    fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let consume_events_admin = accounts.get(0)?;
        let market = accounts.get(1)?;
        let event_heap = accounts.get(2)?;

        Some(ConsumeGivenEventsInstructionAccounts {
            consume_events_admin: consume_events_admin.pubkey,
            market: market.pubkey,
            event_heap: event_heap.pubkey,
        })
    }
}
