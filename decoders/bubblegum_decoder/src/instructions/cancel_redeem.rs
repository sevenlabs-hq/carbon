use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x6f4ce83227af30f2")]
pub struct CancelRedeem {
    pub root: [u8; 32],
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct CancelRedeemInstructionAccounts {
    pub tree_authority: solana_pubkey::Pubkey,
    pub leaf_owner: solana_pubkey::Pubkey,
    pub merkle_tree: solana_pubkey::Pubkey,
    pub voucher: solana_pubkey::Pubkey,
    pub log_wrapper: solana_pubkey::Pubkey,
    pub compression_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CancelRedeem {
    type ArrangedAccounts = CancelRedeemInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [tree_authority, leaf_owner, merkle_tree, voucher, log_wrapper, compression_program, system_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(CancelRedeemInstructionAccounts {
            tree_authority: tree_authority.pubkey,
            leaf_owner: leaf_owner.pubkey,
            merkle_tree: merkle_tree.pubkey,
            voucher: voucher.pubkey,
            log_wrapper: log_wrapper.pubkey,
            compression_program: compression_program.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
