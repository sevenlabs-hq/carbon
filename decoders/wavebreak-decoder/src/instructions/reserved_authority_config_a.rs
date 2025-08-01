use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x15")]
pub struct ReservedAuthorityConfigA {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct ReservedAuthorityConfigAInstructionAccounts {}

impl carbon_core::deserialize::ArrangeAccounts for ReservedAuthorityConfigA {
    type ArrangedAccounts = ReservedAuthorityConfigAInstructionAccounts;

    fn arrange_accounts(_: &[solana_instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        Some(ReservedAuthorityConfigAInstructionAccounts {})
    }
}
