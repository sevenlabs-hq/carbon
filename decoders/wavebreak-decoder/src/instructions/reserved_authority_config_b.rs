use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x16")]
pub struct ReservedAuthorityConfigB {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct ReservedAuthorityConfigBInstructionAccounts {}

impl carbon_core::deserialize::ArrangeAccounts for ReservedAuthorityConfigB {
    type ArrangedAccounts = ReservedAuthorityConfigBInstructionAccounts;

    fn arrange_accounts(_: &[solana_instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        Some(ReservedAuthorityConfigBInstructionAccounts {})
    }
}
