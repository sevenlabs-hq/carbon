use carbon_core::borsh;
use carbon_core::deserialize::CarbonDeserialize;
use carbon_proc_macros::CarbonDeserialize;

#[derive(CarbonDeserialize, Debug)]
#[carbon(discriminator = "0x84edb17eb33c1a99")]
pub struct Uninitialized {}
