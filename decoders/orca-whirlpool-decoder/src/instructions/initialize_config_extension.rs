use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x370935097239d134")]
pub struct InitializeConfigExtension {}

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
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [config, config_extension, funder, fee_authority, system_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(InitializeConfigExtensionInstructionAccounts {
            config: config.pubkey,
            config_extension: config_extension.pubkey,
            funder: funder.pubkey,
            fee_authority: fee_authority.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
