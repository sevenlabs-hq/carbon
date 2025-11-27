use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x61a7906b75be8024")]
pub struct OrderUnstake {
    pub msol_amount: u64,
}

pub struct OrderUnstakeInstructionAccounts {
    pub state: solana_pubkey::Pubkey,
    pub msol_mint: solana_pubkey::Pubkey,
    pub burn_msol_from: solana_pubkey::Pubkey,
    pub burn_msol_authority: solana_pubkey::Pubkey,
    pub new_ticket_account: solana_pubkey::Pubkey,
    pub clock: solana_pubkey::Pubkey,
    pub rent: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for OrderUnstake {
    type ArrangedAccounts = OrderUnstakeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [
            state,
            msol_mint,
            burn_msol_from,
            burn_msol_authority,
            new_ticket_account,
            clock,
            rent,
            token_program,
            _remaining @ ..,
        ] = accounts
        else {
            return None;
        };

        Some(OrderUnstakeInstructionAccounts {
            state: state.pubkey,
            msol_mint: msol_mint.pubkey,
            burn_msol_from: burn_msol_from.pubkey,
            burn_msol_authority: burn_msol_authority.pubkey,
            new_ticket_account: new_ticket_account.pubkey,
            clock: clock.pubkey,
            rent: rent.pubkey,
            token_program: token_program.pubkey,
        })
    }
}
