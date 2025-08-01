use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x0e")]
pub struct ReservedTokenZ {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct ReservedTokenZInstructionAccounts {}

impl carbon_core::deserialize::ArrangeAccounts for ReservedTokenZ {
    type ArrangedAccounts = ReservedTokenZInstructionAccounts;

    fn arrange_accounts(_: &[solana_instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        Some(ReservedTokenZInstructionAccounts {})
    }
}
