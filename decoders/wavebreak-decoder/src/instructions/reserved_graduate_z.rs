use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x24")]
pub struct ReservedGraduateZ {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct ReservedGraduateZInstructionAccounts {}

impl carbon_core::deserialize::ArrangeAccounts for ReservedGraduateZ {
    type ArrangedAccounts = ReservedGraduateZInstructionAccounts;

    fn arrange_accounts(_: &[solana_instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        Some(ReservedGraduateZInstructionAccounts {})
    }
}
