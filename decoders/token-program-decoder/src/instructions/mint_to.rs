use carbon_core::borsh;
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x07")]
pub struct MintTo {
    pub amount: u64,
}

pub struct MintToAccounts {
    pub mint: solana_sdk::pubkey::Pubkey,
    pub account: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
    pub remaining_accounts: Vec<solana_sdk::instruction::AccountMeta>,
}

impl ArrangeAccounts for MintTo {
    type ArrangedAccounts = MintToAccounts;

    fn arrange_accounts(
        &self,
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let mint = accounts.get(0)?;
        let account = accounts.get(1)?;
        let authority = accounts.get(2)?;

        Some(MintToAccounts {
            mint: mint.pubkey,
            account: account.pubkey,
            authority: authority.pubkey,
            remaining_accounts: accounts.get(3..).unwrap_or_default().to_vec(),
        })
    }
}
