use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xfdda8dfc2809079a")]
pub struct AdminUnstakeMsol {
    pub version: u16,
    pub ticket_number: u32,
    pub amount: u64,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct AdminUnstakeMsolInstructionAccounts {
    pub token_program: solana_pubkey::Pubkey,
    pub associated_token_program: solana_pubkey::Pubkey,
    pub payer: solana_pubkey::Pubkey,
    pub admin: solana_pubkey::Pubkey,
    pub protocol_config_state: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub protocol_staking_admin_state: solana_pubkey::Pubkey,
    pub msol_ticket: solana_pubkey::Pubkey,
    pub msol_mint: solana_pubkey::Pubkey,
    pub msol_ticket_sol_spent: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for AdminUnstakeMsol {
    type ArrangedAccounts = AdminUnstakeMsolInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [token_program, associated_token_program, payer, admin, protocol_config_state, system_program, protocol_staking_admin_state, msol_ticket, msol_mint, msol_ticket_sol_spent, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(AdminUnstakeMsolInstructionAccounts {
            token_program: token_program.pubkey,
            associated_token_program: associated_token_program.pubkey,
            payer: payer.pubkey,
            admin: admin.pubkey,
            protocol_config_state: protocol_config_state.pubkey,
            system_program: system_program.pubkey,
            protocol_staking_admin_state: protocol_staking_admin_state.pubkey,
            msol_ticket: msol_ticket.pubkey,
            msol_mint: msol_mint.pubkey,
            msol_ticket_sol_spent: msol_ticket_sol_spent.pubkey,
        })
    }
}
