use carbon_core::borsh;
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x09")]
pub struct CloseAccount {}

pub struct CloseAccountAccounts {
    pub account: solana_sdk::pubkey::Pubkey,
    pub destination: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub _remaining_accounts: Vec<solana_sdk::instruction::AccountMeta>,
}

impl ArrangeAccounts for CloseAccount {
    type ArrangedAccounts = CloseAccountAccounts;

    fn arrange_accounts(
        &self,
        accounts: Vec<solana_sdk::pubkey::Pubkey>,
    ) -> Option<Self::ArrangedAccounts> {
        let account = accounts.get(0)?;
        let destination = accounts.get(1)?;
        let owner = accounts.get(2)?;

        Some(CloseAccountAccounts {
            account: *account,
            destination: *destination,
            owner: *owner,
            _remaining_accounts: accounts
                .get(3..)
                .unwrap_or_default()
                .to_vec()
                .into_iter()
                .map(|pubkey| solana_sdk::instruction::AccountMeta {
                    pubkey,
                    is_signer: true,
                    is_writable: false,
                })
                .collect::<Vec<solana_sdk::instruction::AccountMeta>>(),
        })
    }
}
