use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize)]
#[carbon(discriminator = "0x51c5b457ed001663")]
pub struct AddressLookupTable {
    pub meta: LookupTableMeta,
    pub addresses: LookupTableAddresses,
}
