use carbon_core::borsh;
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x00")]
pub struct InitializeMint {
    pub decimals: u8,
    pub mint_authority: solana_sdk::pubkey::Pubkey,
    // TODO: It's COPTION on github so don't know
    pub freeze_authority: Option<solana_sdk::pubkey::Pubkey>,
}

pub struct InitializeMintAccounts {
    pub mint: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for InitializeMint {
    type ArrangedAccounts = InitializeMintAccounts;

    fn arrange_accounts(
        &self,
        accounts: Vec<solana_sdk::pubkey::Pubkey>,
    ) -> Option<Self::ArrangedAccounts> {
        let mint = accounts.get(0)?;
        let rent = accounts.get(1)?;

        Some(InitializeMintAccounts {
            mint: *mint,
            rent: *rent,
        })
    }
}
