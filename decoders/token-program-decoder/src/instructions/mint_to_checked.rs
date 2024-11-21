use carbon_core::{borsh, CarbonDeserialize};
#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x0e")]
pub struct MintToChecked {
    pub amount: u64,
    pub decimals: u8,
}

pub struct MintToCheckedAccounts {
    pub mint: solana_sdk::pubkey::Pubkey,
    pub account: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
    pub remaining_accounts: Vec<solana_sdk::instruction::AccountMeta>,
}

impl carbon_core::deserialize::ArrangeAccounts for MintToChecked {
    type ArrangedAccounts = MintToCheckedAccounts;

fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let mint = accounts.get(0)?;
        let account = accounts.get(1)?;
        let authority = accounts.get(2)?;

        Some(MintToCheckedAccounts {
            mint: mint.pubkey,
            account: account.pubkey,
            authority: authority.pubkey,
            remaining_accounts: accounts.get(3..).unwrap_or_default().to_vec(),
        })
    }
}
