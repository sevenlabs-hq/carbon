use carbon_core::borsh;
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x0d")]
pub struct ApproveChecked {
    pub amount: u64,
    pub decimals: u8,
}

pub struct ApproveCheckedAccounts {
    pub source: solana_sdk::pubkey::Pubkey,
    pub mint: solana_sdk::pubkey::Pubkey,
    pub delegate: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub remaining_accounts: Vec<solana_sdk::instruction::AccountMeta>,
}

impl ArrangeAccounts for ApproveChecked {
    type ArrangedAccounts = ApproveCheckedAccounts;

    fn arrange_accounts(
        &self,
        accounts: Vec<solana_sdk::pubkey::Pubkey>,
    ) -> Option<Self::ArrangedAccounts> {
        let source = accounts.get(0)?;
        let mint = accounts.get(1)?;
        let delegate = accounts.get(2)?;
        let owner = accounts.get(3)?;

        Some(ApproveCheckedAccounts {
            source: *source,
            mint: *mint,
            delegate: *delegate,
            owner: *owner,
            remaining_accounts: accounts
                .get(4..)
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
