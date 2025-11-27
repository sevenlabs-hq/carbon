use carbon_core::{CarbonDeserialize, borsh};

#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize)]
#[carbon(discriminator = "0x255c934efa649389")]
pub struct PermissionlessFarmSwitch {
    pub bump: u8,
    pub is_on: bool,
}
