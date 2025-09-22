use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x370935097239d134")]
pub struct InitializeConfigExtension {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct InitializeConfigExtensionInstructionAccounts {
    pub config: solana_pubkey::Pubkey,
    pub config_extension: solana_pubkey::Pubkey,
    pub funder: solana_pubkey::Pubkey,
    pub fee_authority: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializeConfigExtension {
    type ArrangedAccounts = InitializeConfigExtensionInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let config = next_account(&mut iter)?;
        let config_extension = next_account(&mut iter)?;
        let funder = next_account(&mut iter)?;
        let fee_authority = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;

        Some(InitializeConfigExtensionInstructionAccounts {
            config,
            config_extension,
            funder,
            fee_authority,
            system_program,
        })
    }
}
