use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x2b")]
pub struct ReservedCreateY {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct ReservedCreateYInstructionAccounts {}

impl carbon_core::deserialize::ArrangeAccounts for ReservedCreateY {
    type ArrangedAccounts = ReservedCreateYInstructionAccounts;

    fn arrange_accounts(_: &[solana_instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        Some(ReservedCreateYInstructionAccounts {})
    }
}
