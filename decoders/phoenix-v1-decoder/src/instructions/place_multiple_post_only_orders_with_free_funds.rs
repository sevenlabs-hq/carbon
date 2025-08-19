use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x11")]
pub struct PlaceMultiplePostOnlyOrdersWithFreeFunds {
    pub multiple_order_packet: MultipleOrderPacket,
}

pub struct PlaceMultiplePostOnlyOrdersWithFreeFundsInstructionAccounts {
    pub phoenix_program: solana_pubkey::Pubkey,
    pub log_authority: solana_pubkey::Pubkey,
    pub market: solana_pubkey::Pubkey,
    pub trader: solana_pubkey::Pubkey,
    pub seat: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for PlaceMultiplePostOnlyOrdersWithFreeFunds {
    type ArrangedAccounts = PlaceMultiplePostOnlyOrdersWithFreeFundsInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [phoenix_program, log_authority, market, trader, seat, _remaining @ ..] = accounts
        else {
            return None;
        };

        Some(
            PlaceMultiplePostOnlyOrdersWithFreeFundsInstructionAccounts {
                phoenix_program: phoenix_program.pubkey,
                log_authority: log_authority.pubkey,
                market: market.pubkey,
                trader: trader.pubkey,
                seat: seat.pubkey,
            },
        )
    }
}
