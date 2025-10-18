use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x36")]
pub struct ReservedBondingCurveZ {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct ReservedBondingCurveZInstructionAccounts {}

impl carbon_core::deserialize::ArrangeAccounts for ReservedBondingCurveZ {
    type ArrangedAccounts = ReservedBondingCurveZInstructionAccounts;

    fn arrange_accounts(_: &[solana_instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        Some(ReservedBondingCurveZInstructionAccounts {})
    }
}
