use carbon_core::{borsh, CarbonDeserialize};
#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x0A")]
pub struct AssignWithSeed {
    pub base: solana_sdk::pubkey::Pubkey,
    pub seed: u64,
    pub owner: solana_sdk::pubkey::Pubkey,
}

pub struct AssignWithSeedAccounts {
    pub assigned_account: solana_sdk::pubkey::Pubkey,
    pub base_account: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for AssignWithSeed {
    type ArrangedAccounts = AssignWithSeedAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [assigned_account, base_account] = accounts else {
            return None;
        };

        Some(AssignWithSeedAccounts {
            assigned_account: assigned_account.pubkey,
            base_account: base_account.pubkey,
        })
    }
}
