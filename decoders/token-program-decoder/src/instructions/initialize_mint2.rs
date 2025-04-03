use carbon_core::{borsh, CarbonDeserialize};
#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x14")]
pub struct InitializeMint2 {
    pub decimals: u8,
    pub mint_authority: solana_pubkey::Pubkey,
    pub freeze_authority: Option<solana_pubkey::Pubkey>,
}

pub struct InitializeMint2Accounts {
    pub mint: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializeMint2 {
    type ArrangedAccounts = InitializeMint2Accounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mint = accounts.first()?;

        Some(InitializeMint2Accounts { mint: mint.pubkey })
    }
}
