use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x1c")]
pub struct ReservedMintConfigZ {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct ReservedMintConfigZInstructionAccounts {}

impl carbon_core::deserialize::ArrangeAccounts for ReservedMintConfigZ {
    type ArrangedAccounts = ReservedMintConfigZInstructionAccounts;

    fn arrange_accounts(_: &[solana_instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        Some(ReservedMintConfigZInstructionAccounts {})
    }
}
