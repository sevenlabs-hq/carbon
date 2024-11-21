use carbon_core::{borsh, CarbonDeserialize};
#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x0B")]
pub struct TransferWithSeed {
    pub lamports: u64,
    pub from_seed: String,
    pub from_owner: solana_sdk::pubkey::Pubkey,
}

pub struct TransferWithSeedAccounts {
    pub funding_account: solana_sdk::pubkey::Pubkey,
    pub base_for_funding_account: solana_sdk::pubkey::Pubkey,
    pub recipient_account: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for TransferWithSeed {
    type ArrangedAccounts = TransferWithSeedAccounts;

fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let funding_account = accounts.get(0)?;
        let base_for_funding_account = accounts.get(1)?;
        let recipient_account = accounts.get(2)?;

        Some(TransferWithSeedAccounts {
            funding_account: funding_account.pubkey,
            base_for_funding_account: base_for_funding_account.pubkey,
            recipient_account: recipient_account.pubkey,
        })
    }
}
