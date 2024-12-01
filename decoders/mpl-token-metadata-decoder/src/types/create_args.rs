
use super::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub enum CreateArgs {
    V1
                {
                    asset_data: AssetData,
                    decimals: Option<u8>,
                    print_supply: Option<PrintSupply>,
                }
    ,
}


