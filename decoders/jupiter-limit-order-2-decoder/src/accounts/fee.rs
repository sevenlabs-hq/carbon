use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x183796faa81b65b2")]
pub struct Fee {
    pub normal_fee_bps: u16,
    pub stable_fee_bps: u16,
}
