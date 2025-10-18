use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x2f")]
pub struct ReservedCreateC {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct ReservedCreateCInstructionAccounts {}

impl carbon_core::deserialize::ArrangeAccounts for ReservedCreateC {
    type ArrangedAccounts = ReservedCreateCInstructionAccounts;

    fn arrange_accounts(_: &[solana_instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        Some(ReservedCreateCInstructionAccounts {})
    }
}
