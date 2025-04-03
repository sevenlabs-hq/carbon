use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x668fdc88179e889d")]
pub struct AdminCrankEventQueue {
    pub events_to_crank: u16,
}

pub struct AdminCrankEventQueueInstructionAccounts {
    pub state: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
    pub market: solana_pubkey::Pubkey,
    pub event_queue: solana_pubkey::Pubkey,
    pub dex_program: solana_pubkey::Pubkey,
    pub serum_authority: solana_pubkey::Pubkey,
    pub open_orders: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for AdminCrankEventQueue {
    type ArrangedAccounts = AdminCrankEventQueueInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, authority, market, event_queue, dex_program, serum_authority, open_orders, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(AdminCrankEventQueueInstructionAccounts {
            state: state.pubkey,
            authority: authority.pubkey,
            market: market.pubkey,
            event_queue: event_queue.pubkey,
            dex_program: dex_program.pubkey,
            serum_authority: serum_authority.pubkey,
            open_orders: open_orders.pubkey,
        })
    }
}
