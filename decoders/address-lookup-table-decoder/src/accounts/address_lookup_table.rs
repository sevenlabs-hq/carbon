use carbon_core::{borsh, CarbonDeserialize};

use super::super::types::*;

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
pub struct AddressLookupTable {
    pub meta: LookupTableMeta,
    pub addresses: LookupTableAddresses,
}
