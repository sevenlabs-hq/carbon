use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)]
#[carbon(discriminator = "0x95089ccaa0fcb0d9")]
pub struct GlobalConfig {}
