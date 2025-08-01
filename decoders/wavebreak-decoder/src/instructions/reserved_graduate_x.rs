use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x22")]
pub struct ReservedGraduateX {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct ReservedGraduateXInstructionAccounts {}

impl carbon_core::deserialize::ArrangeAccounts for ReservedGraduateX {
    type ArrangedAccounts = ReservedGraduateXInstructionAccounts;

    fn arrange_accounts(_: &[solana_instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        Some(ReservedGraduateXInstructionAccounts {})
    }
}
