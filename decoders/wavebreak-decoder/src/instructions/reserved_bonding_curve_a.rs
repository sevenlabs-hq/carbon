use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x37")]
pub struct ReservedBondingCurveA {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct ReservedBondingCurveAInstructionAccounts {}

impl carbon_core::deserialize::ArrangeAccounts for ReservedBondingCurveA {
    type ArrangedAccounts = ReservedBondingCurveAInstructionAccounts;

    fn arrange_accounts(_: &[solana_instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        Some(ReservedBondingCurveAInstructionAccounts {})
    }
}
