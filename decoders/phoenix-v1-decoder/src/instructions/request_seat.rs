use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x0e")]
pub struct RequestSeat {}

pub struct RequestSeatInstructionAccounts {
    pub phoenix_program: solana_sdk::pubkey::Pubkey,
    pub log_authority: solana_sdk::pubkey::Pubkey,
    pub market: solana_sdk::pubkey::Pubkey,
    pub payer: solana_sdk::pubkey::Pubkey,
    pub seat: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for RequestSeat {
    type ArrangedAccounts = RequestSeatInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [phoenix_program, log_authority, market, payer, seat, system_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(RequestSeatInstructionAccounts {
            phoenix_program: phoenix_program.pubkey,
            log_authority: log_authority.pubkey,
            market: market.pubkey,
            payer: payer.pubkey,
            seat: seat.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
