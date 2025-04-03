use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x71d87a83e1d11637")]
pub struct InitializeGlobalConfig {}

pub struct InitializeGlobalConfigInstructionAccounts {
    pub admin_authority: solana_pubkey::Pubkey,
    pub pda_authority: solana_pubkey::Pubkey,
    pub global_config: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializeGlobalConfig {
    type ArrangedAccounts = InitializeGlobalConfigInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [admin_authority, pda_authority, global_config, _remaining @ ..] = accounts else {
            return None;
        };

        Some(InitializeGlobalConfigInstructionAccounts {
            admin_authority: admin_authority.pubkey,
            pda_authority: pda_authority.pubkey,
            global_config: global_config.pubkey,
        })
    }
}
