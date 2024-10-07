use carbon_core::borsh;
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x0B")]
pub struct TransferWithSeed {
    lamports: u64,
    from_seed: String,
    from_owner: solana_sdk::pubkey::Pubkey,
}

pub struct TransferWithSeedAccounts {
    pub funding_account: solana_sdk::pubkey::Pubkey,
    pub base_for_funding_account: solana_sdk::pubkey::Pubkey,
    pub recipient_account: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for TransferWithSeed {
    type ArrangedAccounts = TransferWithSeedAccounts;

    fn arrange_accounts(
        &self,
        accounts: Vec<solana_sdk::pubkey::Pubkey>,
    ) -> Option<Self::ArrangedAccounts> {
        let funding_account = accounts.get(0)?;
        let base_for_funding_account = accounts.get(1)?;
        let recipient_account = accounts.get(2)?;

        Some(TransferWithSeedAccounts {
            funding_account: *funding_account,
            base_for_funding_account: *base_for_funding_account,
            recipient_account: *recipient_account,
        })
    }
}
