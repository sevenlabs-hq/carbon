use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x69")]
pub struct RequestSeatAuthorized {}

pub struct RequestSeatAuthorizedInstructionAccounts {
    pub phoenix_program: solana_pubkey::Pubkey,
    pub log_authority: solana_pubkey::Pubkey,
    pub market: solana_pubkey::Pubkey,
    pub market_authority: solana_pubkey::Pubkey,
    pub payer: solana_pubkey::Pubkey,
    pub trader: solana_pubkey::Pubkey,
    pub seat: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for RequestSeatAuthorized {
    type ArrangedAccounts = RequestSeatAuthorizedInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [
            phoenix_program,
            log_authority,
            market,
            market_authority,
            payer,
            trader,
            seat,
            system_program,
            _remaining @ ..,
        ] = accounts
        else {
            return None;
        };

        Some(RequestSeatAuthorizedInstructionAccounts {
            phoenix_program: phoenix_program.pubkey,
            log_authority: log_authority.pubkey,
            market: market.pubkey,
            market_authority: market_authority.pubkey,
            payer: payer.pubkey,
            trader: trader.pubkey,
            seat: seat.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
