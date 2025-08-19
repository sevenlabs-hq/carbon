use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x1b")]
pub struct ReservedMintConfigY {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct ReservedMintConfigYInstructionAccounts {}

impl carbon_core::deserialize::ArrangeAccounts for ReservedMintConfigY {
    type ArrangedAccounts = ReservedMintConfigYInstructionAccounts;

    fn arrange_accounts(_: &[solana_instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        Some(ReservedMintConfigYInstructionAccounts {})
    }
}
