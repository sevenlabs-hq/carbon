use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x0f")]
pub struct ReservedTokenA {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct ReservedTokenAInstructionAccounts {}

impl carbon_core::deserialize::ArrangeAccounts for ReservedTokenA {
    type ArrangedAccounts = ReservedTokenAInstructionAccounts;

    fn arrange_accounts(_: &[solana_instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        Some(ReservedTokenAInstructionAccounts {})
    }
}
