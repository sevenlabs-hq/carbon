use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x1e")]
pub struct ReservedMintConfigB {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct ReservedMintConfigBInstructionAccounts {}

impl carbon_core::deserialize::ArrangeAccounts for ReservedMintConfigB {
    type ArrangedAccounts = ReservedMintConfigBInstructionAccounts;

    fn arrange_accounts(_: &[solana_instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        Some(ReservedMintConfigBInstructionAccounts {})
    }
}
