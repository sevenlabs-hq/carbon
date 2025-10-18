use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

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
        let mut iter = accounts.iter();
        let token_program = next_account(&mut iter)?;
        let associated_token_program = next_account(&mut iter)?;
        let payer = next_account(&mut iter)?;
        let admin = next_account(&mut iter)?;
        let protocol_config_state = next_account(&mut iter)?;
        let protocol_config_wsol_vault = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;
        let protocol_staking_admin_state = next_account(&mut iter)?;
        let wsol_token_vault = next_account(&mut iter)?;
        let wsol_mint = next_account(&mut iter)?;

        Some(AdminClaimStakingRewardsInstructionAccounts {
            token_program,
            associated_token_program,
            payer,
            admin,
            protocol_config_state,
            protocol_config_wsol_vault,
            system_program,
            protocol_staking_admin_state,
            wsol_token_vault,
            wsol_mint,
        })
    }
}
