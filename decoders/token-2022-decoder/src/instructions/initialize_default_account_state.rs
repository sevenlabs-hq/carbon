use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x1c")]
pub struct InitializeDefaultAccountState {
    pub default_account_state_discriminator: u8,
    pub state: AccountState,
}

pub struct InitializeDefaultAccountStateInstructionAccounts {
    pub mint: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializeDefaultAccountState {
    type ArrangedAccounts = InitializeDefaultAccountStateInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [mint, _remaining @ ..] = accounts else {
            return None;
        };

        Some(InitializeDefaultAccountStateInstructionAccounts { mint: mint.pubkey })
    }
}
