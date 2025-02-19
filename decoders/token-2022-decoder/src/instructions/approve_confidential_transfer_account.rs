use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x1b")]
pub struct ApproveConfidentialTransferAccount {
    pub confidential_transfer_discriminator: u8,
}

pub struct ApproveConfidentialTransferAccountInstructionAccounts {
    pub token: solana_sdk::pubkey::Pubkey,
    pub mint: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ApproveConfidentialTransferAccount {
    type ArrangedAccounts = ApproveConfidentialTransferAccountInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [token, mint, authority, _remaining @ ..] = accounts else {
            return None;
        };

        Some(ApproveConfidentialTransferAccountInstructionAccounts {
            token: token.pubkey,
            mint: mint.pubkey,
            authority: authority.pubkey,
        })
    }
}
