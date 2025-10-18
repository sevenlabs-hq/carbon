use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x25")]
pub struct ReservedGraduateA {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct ReservedGraduateAInstructionAccounts {}

impl carbon_core::deserialize::ArrangeAccounts for ReservedGraduateA {
    type ArrangedAccounts = ReservedGraduateAInstructionAccounts;

    fn arrange_accounts(_: &[solana_instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        Some(ReservedGraduateAInstructionAccounts {})
    }
}
