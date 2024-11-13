use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub enum ExtraAccounts {
    None,
    SplHook {
        extra_account_metas: solana_sdk::pubkey::Pubkey,
    },
    MplHook {
        mint_pda: Option<solana_sdk::pubkey::Pubkey>,
        collection_pda: Option<solana_sdk::pubkey::Pubkey>,
        owner_pda: Option<solana_sdk::pubkey::Pubkey>,
    },
}
