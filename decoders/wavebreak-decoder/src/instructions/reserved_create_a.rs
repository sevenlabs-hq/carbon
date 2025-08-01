use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x2d")]
pub struct ReservedCreateA {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct ReservedCreateAInstructionAccounts {}

impl carbon_core::deserialize::ArrangeAccounts for ReservedCreateA {
    type ArrangedAccounts = ReservedCreateAInstructionAccounts;

    fn arrange_accounts(_: &[solana_instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        Some(ReservedCreateAInstructionAccounts {})
    }
}
