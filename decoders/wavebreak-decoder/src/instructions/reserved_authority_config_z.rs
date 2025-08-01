use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x14")]
pub struct ReservedAuthorityConfigZ {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct ReservedAuthorityConfigZInstructionAccounts {}

impl carbon_core::deserialize::ArrangeAccounts for ReservedAuthorityConfigZ {
    type ArrangedAccounts = ReservedAuthorityConfigZInstructionAccounts;

    fn arrange_accounts(_: &[solana_instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        Some(ReservedAuthorityConfigZInstructionAccounts {})
    }
}
