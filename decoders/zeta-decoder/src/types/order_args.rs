use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct OrderArgs {
    pub price: u64,
    pub size: u64,
    pub client_order_id: Option<u64>,
    pub tif_offset: Option<u16>,
}
