use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x883d30e8a61acf2e")]
pub struct AdminRepaySol {
    pub version: u16,
    pub amount: u64,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct AdminRepaySolInstructionAccounts {
    pub token_program: solana_pubkey::Pubkey,
    pub associated_token_program: solana_pubkey::Pubkey,
    pub payer: solana_pubkey::Pubkey,
    pub admin: solana_pubkey::Pubkey,
    pub protocol_config_state: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub protocol_staking_admin_state: solana_pubkey::Pubkey,
    pub address_lookup_program: solana_pubkey::Pubkey,
    pub instruction_sysvar_account_info: solana_pubkey::Pubkey,
    pub temp_sol_holder: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for AdminRepaySol {
    type ArrangedAccounts = AdminRepaySolInstructionAccounts;

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
        let address_lookup_program = next_account(&mut iter)?;
        let instruction_sysvar_account_info = next_account(&mut iter)?;
        let temp_sol_holder = next_account(&mut iter)?;

        Some(AdminRepaySolInstructionAccounts {
            token_program,
            associated_token_program,
            payer,
            admin,
            protocol_config_state,
            system_program,
            protocol_staking_admin_state,
            address_lookup_program,
            instruction_sysvar_account_info,
            temp_sol_holder,
        })
    }
}
