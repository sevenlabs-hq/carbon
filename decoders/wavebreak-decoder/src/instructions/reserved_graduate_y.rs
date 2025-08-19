use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x23")]
pub struct ReservedGraduateY {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct ReservedGraduateYInstructionAccounts {}

impl carbon_core::deserialize::ArrangeAccounts for ReservedGraduateY {
    type ArrangedAccounts = ReservedGraduateYInstructionAccounts;

    fn arrange_accounts(_: &[solana_instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        Some(ReservedGraduateYInstructionAccounts {})
    }
}
