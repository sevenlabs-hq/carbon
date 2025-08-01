use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x13")]
pub struct ReservedAuthorityConfigY {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct ReservedAuthorityConfigYInstructionAccounts {}

impl carbon_core::deserialize::ArrangeAccounts for ReservedAuthorityConfigY {
    type ArrangedAccounts = ReservedAuthorityConfigYInstructionAccounts;

    fn arrange_accounts(_: &[solana_instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        Some(ReservedAuthorityConfigYInstructionAccounts {})
    }
}
