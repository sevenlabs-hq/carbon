use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x68")]
pub struct ChangeSeatStatus {
    pub approval_status: SeatApprovalStatus,
}

pub struct ChangeSeatStatusInstructionAccounts {
    pub phoenix_program: solana_pubkey::Pubkey,
    pub log_authority: solana_pubkey::Pubkey,
    pub market: solana_pubkey::Pubkey,
    pub market_authority: solana_pubkey::Pubkey,
    pub seat: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ChangeSeatStatus {
    type ArrangedAccounts = ChangeSeatStatusInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [phoenix_program, log_authority, market, market_authority, seat, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(ChangeSeatStatusInstructionAccounts {
            phoenix_program: phoenix_program.pubkey,
            log_authority: log_authority.pubkey,
            market: market.pubkey,
            market_authority: market_authority.pubkey,
            seat: seat.pubkey,
        })
    }
}
