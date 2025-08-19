use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x6816227b56e08246")]
pub struct ExpireSeriesOverride {
    pub args: ExpireSeriesOverrideArgs,
}

pub struct ExpireSeriesOverrideInstructionAccounts {}

impl carbon_core::deserialize::ArrangeAccounts for ExpireSeriesOverride {
    type ArrangedAccounts = ExpireSeriesOverrideInstructionAccounts;

    fn arrange_accounts(
        _accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        Some(ExpireSeriesOverrideInstructionAccounts {})
    }
}
