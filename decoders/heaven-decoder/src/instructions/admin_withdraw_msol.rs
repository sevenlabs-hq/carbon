use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xf9db8d48d26ed863")]
pub struct AdminWithdrawMsol {
    pub version: u16,
    pub amount: u64,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct AdminWithdrawMsolInstructionAccounts {
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

impl carbon_core::deserialize::ArrangeAccounts for AdminWithdrawMsol {
    type ArrangedAccounts = AdminWithdrawMsolInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [token_program, associated_token_program, payer, admin, protocol_config_state, system_program, protocol_staking_admin_state, address_lookup_program, instruction_sysvar_account_info, temp_sol_holder, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(AdminWithdrawMsolInstructionAccounts {
            token_program: token_program.pubkey,
            associated_token_program: associated_token_program.pubkey,
            payer: payer.pubkey,
            admin: admin.pubkey,
            protocol_config_state: protocol_config_state.pubkey,
            system_program: system_program.pubkey,
            protocol_staking_admin_state: protocol_staking_admin_state.pubkey,
            address_lookup_program: address_lookup_program.pubkey,
            instruction_sysvar_account_info: instruction_sysvar_account_info.pubkey,
            temp_sol_holder: temp_sol_holder.pubkey,
        })
    }
}
