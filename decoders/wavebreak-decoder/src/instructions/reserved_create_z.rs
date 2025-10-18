use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x2c")]
pub struct ReservedCreateZ {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct ReservedCreateZInstructionAccounts {}

impl carbon_core::deserialize::ArrangeAccounts for ReservedCreateZ {
    type ArrangedAccounts = ReservedCreateZInstructionAccounts;

    fn arrange_accounts(_: &[solana_instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        Some(ReservedCreateZInstructionAccounts {})
    }
}
