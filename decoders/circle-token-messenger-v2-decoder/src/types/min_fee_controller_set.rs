use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct MinFeeControllerSet {
    pub new_min_fee_controller: solana_pubkey::Pubkey,
}
