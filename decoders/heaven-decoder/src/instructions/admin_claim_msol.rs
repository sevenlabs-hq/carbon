use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x7c303cc7cb312429")]
pub struct AdminClaimMsol {
    pub version: u16,
    pub ticket_number: u32,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct AdminClaimMsolInstructionAccounts {
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

impl carbon_core::deserialize::ArrangeAccounts for AdminClaimMsol {
    type ArrangedAccounts = AdminClaimMsolInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let token_program = next_account(&mut iter)?;
        let associated_token_program = next_account(&mut iter)?;
        let payer = next_account(&mut iter)?;
        let admin = next_account(&mut iter)?;
        let protocol_config_state = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;
        let protocol_staking_admin_state = next_account(&mut iter)?;
        let msol_ticket = next_account(&mut iter)?;
        let msol_mint = next_account(&mut iter)?;
        let msol_ticket_sol_spent = next_account(&mut iter)?;

        Some(AdminClaimMsolInstructionAccounts {
            token_program,
            associated_token_program,
            payer,
            admin,
            protocol_config_state,
            system_program,
            protocol_staking_admin_state,
            msol_ticket,
            msol_mint,
            msol_ticket_sol_spent,
        })
    }
}
