use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x1f")]
pub struct ReservedMintConfigC {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct ReservedMintConfigCInstructionAccounts {}

impl carbon_core::deserialize::ArrangeAccounts for ReservedMintConfigC {
    type ArrangedAccounts = ReservedMintConfigCInstructionAccounts;

    fn arrange_accounts(_: &[solana_instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        Some(ReservedMintConfigCInstructionAccounts {})
    }
}
