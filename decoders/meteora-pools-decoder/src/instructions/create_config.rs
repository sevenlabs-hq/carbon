use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xc9cff3724b6f2fbd")]
pub struct CreateConfig {
    pub config_parameters: ConfigParameters,
}

pub struct CreateConfigInstructionAccounts {
    pub config: solana_pubkey::Pubkey,
    pub admin: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CreateConfig {
    type ArrangedAccounts = CreateConfigInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [config, admin, system_program, _remaining @ ..] = accounts else {
            return None;
        };

        Some(CreateConfigInstructionAccounts {
            config: config.pubkey,
            admin: admin.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
