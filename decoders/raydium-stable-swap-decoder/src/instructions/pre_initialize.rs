use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x0a")]
pub struct PreInitialize {
    pub nonce: u8,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct PreInitializeInstructionAccounts {}

impl carbon_core::deserialize::ArrangeAccounts for PreInitialize {
    type ArrangedAccounts = PreInitializeInstructionAccounts;

    fn arrange_accounts(
        _accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        Some(PreInitializeInstructionAccounts {})
    }
}
