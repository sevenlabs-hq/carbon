use carbon_core::borsh;
use carbon_core::deserialize::CarbonDeserialize;
use carbon_proc_macros::CarbonDeserialize;

#[derive(CarbonDeserialize, Debug)]
#[carbon(discriminator = "0x8bc283b38cb3e5f4")]
pub struct Oracle {
    pub idx: u64,
    pub active_size: u64,
    pub length: u64,
}
