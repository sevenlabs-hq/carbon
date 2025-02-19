use super::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct LifecycleHook {
    pub hooked_program: solana_sdk::pubkey::Pubkey,
    pub extra_accounts: Option<Vec<ExtraAccount>>,
    pub data_authority: Option<Authority>,
    pub schema: ExternalPluginAdapterSchema,
}
