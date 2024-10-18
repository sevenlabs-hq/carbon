

use carbon_core::deserialize::CarbonDeserialize;
use carbon_proc_macros::CarbonDeserialize;
use carbon_core::borsh;


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub enum OrderBookType {
    Collection
                {
                    collection_key: solana_sdk::pubkey::Pubkey,
                }
    ,
    NFTList
                {
                    list_account: solana_sdk::pubkey::Pubkey,
                }
    ,
}


