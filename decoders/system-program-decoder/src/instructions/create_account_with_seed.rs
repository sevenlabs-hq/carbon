use carbon_core::{borsh, CarbonDeserialize};
#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x03")]
pub struct CreateAccountWithSeed {
    pub base: solana_sdk::pubkey::Pubkey,
    pub seed: String,
    pub lamports: u64,
    pub space: u64,
    pub owner: solana_sdk::pubkey::Pubkey,
}

pub struct CreateAccountWithSeedAccounts {
    pub funding_account: solana_sdk::pubkey::Pubkey,
    pub created_account: solana_sdk::pubkey::Pubkey,
    pub base_account: Option<solana_sdk::pubkey::Pubkey>,
}

impl carbon_core::deserialize::ArrangeAccounts for CreateAccountWithSeed {
    type ArrangedAccounts = CreateAccountWithSeedAccounts;

fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let funding_account = accounts.get(0)?;
        let created_account = accounts.get(1)?;

        Some(CreateAccountWithSeedAccounts {
            funding_account: funding_account.pubkey,
            created_account: created_account.pubkey,
            base_account: if let Some(acc) = accounts.get(2).cloned() {
                Some(acc.pubkey)
            } else {
                None
            },
        })
    }
}
