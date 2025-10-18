use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x2e")]
pub struct ReservedCreateB {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct ReservedCreateBInstructionAccounts {}

impl carbon_core::deserialize::ArrangeAccounts for ReservedCreateB {
    type ArrangedAccounts = ReservedCreateBInstructionAccounts;

    fn arrange_accounts(_: &[solana_instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        Some(ReservedCreateBInstructionAccounts {})
    }
}
