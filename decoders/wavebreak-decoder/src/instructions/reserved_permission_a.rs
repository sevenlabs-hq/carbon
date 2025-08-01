use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x07")]
pub struct ReservedPermissionA {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct ReservedPermissionAInstructionAccounts {}

impl carbon_core::deserialize::ArrangeAccounts for ReservedPermissionA {
    type ArrangedAccounts = ReservedPermissionAInstructionAccounts;

    fn arrange_accounts(_: &[solana_instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        Some(ReservedPermissionAInstructionAccounts {})
    }
}
