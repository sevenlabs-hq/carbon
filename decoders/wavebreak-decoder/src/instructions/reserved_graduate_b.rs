use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x26")]
pub struct ReservedGraduateB {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct ReservedGraduateBInstructionAccounts {}

impl carbon_core::deserialize::ArrangeAccounts for ReservedGraduateB {
    type ArrangedAccounts = ReservedGraduateBInstructionAccounts;

    fn arrange_accounts(_: &[solana_instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        Some(ReservedGraduateBInstructionAccounts {})
    }
}
