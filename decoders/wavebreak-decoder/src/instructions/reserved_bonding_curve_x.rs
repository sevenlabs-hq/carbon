use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x34")]
pub struct ReservedBondingCurveX {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct ReservedBondingCurveXInstructionAccounts {}

impl carbon_core::deserialize::ArrangeAccounts for ReservedBondingCurveX {
    type ArrangedAccounts = ReservedBondingCurveXInstructionAccounts;

    fn arrange_accounts(_: &[solana_instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        Some(ReservedBondingCurveXInstructionAccounts {})
    }
}
