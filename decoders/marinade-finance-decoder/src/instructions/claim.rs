

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x3ec6d6c1d59f6cd2")]
pub struct Claim{
}

pub struct ClaimInstructionAccounts {
    pub state: solana_sdk::pubkey::Pubkey,
    pub reserve_pda: solana_sdk::pubkey::Pubkey,
    pub ticket_account: solana_sdk::pubkey::Pubkey,
    pub transfer_sol_to: solana_sdk::pubkey::Pubkey,
    pub clock: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Claim {
    type ArrangedAccounts = ClaimInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_sdk::instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let [
            state,
            reserve_pda,
            ticket_account,
            transfer_sol_to,
            clock,
            system_program,
            _remaining @ ..
        ] = accounts else {
            return None;
        };
       

        Some(ClaimInstructionAccounts {
            state: state.pubkey,
            reserve_pda: reserve_pda.pubkey,
            ticket_account: ticket_account.pubkey,
            transfer_sol_to: transfer_sol_to.pubkey,
            clock: clock.pubkey,
            system_program: system_program.pubkey,
        })
    }
}