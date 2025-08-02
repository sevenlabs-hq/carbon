use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xfd764225be319a66")]
pub struct SetTreeDelegate {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct SetTreeDelegateInstructionAccounts {
    pub tree_authority: solana_pubkey::Pubkey,
    pub tree_creator: solana_pubkey::Pubkey,
    pub new_tree_delegate: solana_pubkey::Pubkey,
    pub merkle_tree: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SetTreeDelegate {
    type ArrangedAccounts = SetTreeDelegateInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [tree_authority, tree_creator, new_tree_delegate, merkle_tree, system_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(SetTreeDelegateInstructionAccounts {
            tree_authority: tree_authority.pubkey,
            tree_creator: tree_creator.pubkey,
            new_tree_delegate: new_tree_delegate.pubkey,
            merkle_tree: merkle_tree.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
