use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x17")]
pub struct ReservedAuthorityConfigC {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct ReservedAuthorityConfigCInstructionAccounts {}

impl carbon_core::deserialize::ArrangeAccounts for ReservedAuthorityConfigC {
    type ArrangedAccounts = ReservedAuthorityConfigCInstructionAccounts;

    fn arrange_accounts(_: &[solana_instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        Some(ReservedAuthorityConfigCInstructionAccounts {})
    }
}
