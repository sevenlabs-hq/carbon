use {
    super::super::types::*,
    carbon_core::{CarbonDeserialize, borsh},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x50256d88528759f1")]
pub struct ConfigUpdate {
    pub data: ConfigParams,
}

#[derive(Debug)]
pub struct ConfigUpdateInstructionAccounts {
    pub config_authority: solana_pubkey::Pubkey,
    pub config_account: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ConfigUpdate {
    type ArrangedAccounts = ConfigUpdateInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [config_authority, config_account, _remaining @ ..] = accounts else {
            return None;
        };

        Some(ConfigUpdateInstructionAccounts {
            config_authority: config_authority.pubkey,
            config_account: config_account.pubkey,
        })
    }
}
