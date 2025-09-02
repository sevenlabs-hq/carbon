use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x3ea214857941911b")]
pub struct InitializeFeeConfig {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct InitializeFeeConfigInstructionAccounts {
    pub admin: solana_pubkey::Pubkey,
    pub fee_config: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub config_program_id: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializeFeeConfig {
    type ArrangedAccounts = InitializeFeeConfigInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let admin = next_account(&mut iter)?;
        let fee_config = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;
        let config_program_id = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        Some(InitializeFeeConfigInstructionAccounts {
            admin,
            fee_config,
            system_program,
            config_program_id,
            event_authority,
            program,
        })
    }
}
