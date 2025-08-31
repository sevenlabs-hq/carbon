use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x18a3dcabe1dea6f8")]
pub struct AdminClaimStakingRewards {
    pub version: u16,
    pub amount: u64,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct AdminClaimStakingRewardsInstructionAccounts {
    pub token_program: solana_pubkey::Pubkey,
    pub associated_token_program: solana_pubkey::Pubkey,
    pub payer: solana_pubkey::Pubkey,
    pub admin: solana_pubkey::Pubkey,
    pub protocol_config_state: solana_pubkey::Pubkey,
    pub protocol_config_wsol_vault: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub protocol_staking_admin_state: solana_pubkey::Pubkey,
    pub wsol_token_vault: solana_pubkey::Pubkey,
    pub wsol_mint: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for AdminClaimStakingRewards {
    type ArrangedAccounts = AdminClaimStakingRewardsInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [token_program, associated_token_program, payer, admin, protocol_config_state, protocol_config_wsol_vault, system_program, protocol_staking_admin_state, wsol_token_vault, wsol_mint, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(AdminClaimStakingRewardsInstructionAccounts {
            token_program: token_program.pubkey,
            associated_token_program: associated_token_program.pubkey,
            payer: payer.pubkey,
            admin: admin.pubkey,
            protocol_config_state: protocol_config_state.pubkey,
            protocol_config_wsol_vault: protocol_config_wsol_vault.pubkey,
            system_program: system_program.pubkey,
            protocol_staking_admin_state: protocol_staking_admin_state.pubkey,
            wsol_token_vault: wsol_token_vault.pubkey,
            wsol_mint: wsol_mint.pubkey,
        })
    }
}
