use super::super::types::*;

use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x68b867f258976b14")]
pub struct UpdateFeeConfig {
    pub fee_tiers: Vec<FeeTier>,
    pub flat_fees: Fees,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct UpdateFeeConfigInstructionAccounts {
    pub fee_config: solana_pubkey::Pubkey,
    pub admin: solana_pubkey::Pubkey,
    pub config_program_id: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateFeeConfig {
    type ArrangedAccounts = UpdateFeeConfigInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let fee_config = next_account(&mut iter)?;
        let admin = next_account(&mut iter)?;
        let config_program_id = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        Some(UpdateFeeConfigInstructionAccounts {
            fee_config,
            admin,
            config_program_id,
            event_authority,
            program,
        })
    }
}
