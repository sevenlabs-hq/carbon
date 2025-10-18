use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct TransferFeeExtensionParams {
    pub transfer_fee_basis_points: u16,
    pub maximum_fee: u64,
}
