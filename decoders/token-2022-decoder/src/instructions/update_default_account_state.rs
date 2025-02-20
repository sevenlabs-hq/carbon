use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x1c")]
pub struct UpdateDefaultAccountState {
    pub default_account_state_discriminator: u8,
    pub state: AccountState,
}

pub struct UpdateDefaultAccountStateInstructionAccounts {
    pub mint: solana_sdk::pubkey::Pubkey,
    pub freeze_authority: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateDefaultAccountState {
    type ArrangedAccounts = UpdateDefaultAccountStateInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [mint, freeze_authority, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdateDefaultAccountStateInstructionAccounts {
            mint: mint.pubkey,
            freeze_authority: freeze_authority.pubkey,
        })
    }
}
