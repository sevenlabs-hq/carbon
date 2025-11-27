use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct SetFeeRecipientParams {
    pub new_fee_recipient: solana_pubkey::Pubkey,
}
