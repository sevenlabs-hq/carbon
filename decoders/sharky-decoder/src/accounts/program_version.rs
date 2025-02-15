use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)]
#[carbon(discriminator = "0x8a68f4c5ce2f9f9a")]
pub struct ProgramVersion {
    pub version: u8,
    pub bump: u8,
    pub updated: i64,
}
