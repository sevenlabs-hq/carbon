use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x150b9f2f04c36a38")]
pub struct CollectV2 {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct CollectV2InstructionAccounts {
    pub tree_authority: solana_pubkey::Pubkey,
    pub destination: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CollectV2 {
    type ArrangedAccounts = CollectV2InstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [tree_authority, destination, _remaining @ ..] = accounts else {
            return None;
        };

        Some(CollectV2InstructionAccounts {
            tree_authority: tree_authority.pubkey,
            destination: destination.pubkey,
        })
    }
}
