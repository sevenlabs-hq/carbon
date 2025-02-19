use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xdd91b1341f2f3fc9")]
pub struct ConsumeEvents {
    pub limit: u64,
}

pub struct ConsumeEventsInstructionAccounts {
    pub consume_events_admin: solana_sdk::pubkey::Pubkey,
    pub market: solana_sdk::pubkey::Pubkey,
    pub event_heap: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ConsumeEvents {
    type ArrangedAccounts = ConsumeEventsInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [consume_events_admin, market, event_heap, _remaining @ ..] = accounts else {
            return None;
        };

        Some(ConsumeEventsInstructionAccounts {
            consume_events_admin: consume_events_admin.pubkey,
            market: market.pubkey,
            event_heap: event_heap.pubkey,
        })
    }
}
