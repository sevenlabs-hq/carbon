use carbon_core::{borsh, CarbonDeserialize};
#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x08")]
pub struct AllocateWithSeed {
    base: solana_sdk::pubkey::Pubkey,
    seed: String,
    space: u64,
    owner: solana_sdk::pubkey::Pubkey,
}

pub struct AllocateWithSeedAccounts {
    pub allocated_account: solana_sdk::pubkey::Pubkey,
    pub base_account: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for AllocateWithSeed {
    type ArrangedAccounts = AllocateWithSeedAccounts;

    fn arrange_accounts(
        &self,
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let allocated_account = accounts.get(0)?;
        let base_account = accounts.get(1)?;

        Some(AllocateWithSeedAccounts {
            allocated_account: allocated_account.pubkey,
            base_account: base_account.pubkey,
        })
    }
}
