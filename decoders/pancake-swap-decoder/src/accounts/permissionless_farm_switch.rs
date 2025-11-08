use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize)]
#[carbon(discriminator = "0x255c934efa649389")]
pub struct PermissionlessFarmSwitch {
    pub bump: u8,
    pub is_on: bool,
}
