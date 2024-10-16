use carbon_core::borsh;
use carbon_core::deserialize::CarbonDeserialize;
use carbon_proc_macros::CarbonDeserialize;

#[derive(CarbonDeserialize, Debug)]
#[carbon(discriminator = "0x183796faa81b65b2")]
pub struct Fee {
    pub normal_fee_bps: u16,
    pub stable_fee_bps: u16,
}
