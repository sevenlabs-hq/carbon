use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize)]
pub struct AddressLookupTable {
    pub meta: LookupTableMeta,
    pub addresses: LookupTableAddresses,
}
