use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x09")]
pub struct CloseAccount {}

pub struct CloseAccountInstructionAccounts {
    pub account: solana_sdk::pubkey::Pubkey,
    pub destination: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CloseAccount {
    type ArrangedAccounts = CloseAccountInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [account, destination, owner, _remaining @ ..] = accounts else {
            return None;
        };

        Some(CloseAccountInstructionAccounts {
            account: account.pubkey,
            destination: destination.pubkey,
            owner: owner.pubkey,
        })
    }
}
