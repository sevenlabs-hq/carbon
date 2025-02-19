use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x36")]
pub struct Collect {}

pub struct CollectInstructionAccounts {
    pub authority: solana_sdk::pubkey::Pubkey,
    pub recipient: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Collect {
    type ArrangedAccounts = CollectInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [authority, recipient, _remaining @ ..] = accounts else {
            return None;
        };

        Some(CollectInstructionAccounts {
            authority: authority.pubkey,
            recipient: recipient.pubkey,
        })
    }
}
