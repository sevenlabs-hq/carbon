use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x0a18a8775630e111")]
pub struct ConfigLp {
    pub params: ConfigLpParams,
}

pub struct ConfigLpInstructionAccounts {
    pub state: solana_sdk::pubkey::Pubkey,
    pub admin_authority: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ConfigLp {
    type ArrangedAccounts = ConfigLpInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, admin_authority, _remaining @ ..] = accounts else {
            return None;
        };

        Some(ConfigLpInstructionAccounts {
            state: state.pubkey,
            admin_authority: admin_authority.pubkey,
        })
    }
}
