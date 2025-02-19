use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x13")]
pub struct Collect {}

pub struct CollectInstructionAccounts {
    pub recipient1: solana_sdk::pubkey::Pubkey,
    pub recipient2: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Collect {
    type ArrangedAccounts = CollectInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [recipient1, recipient2, _remaining @ ..] = accounts else {
            return None;
        };

        Some(CollectInstructionAccounts {
            recipient1: recipient1.pubkey,
            recipient2: recipient2.pubkey,
        })
    }
}
