
 
use carbon_core::deserialize::CarbonDeserialize; 
use carbon_proc_macros::CarbonDeserialize; 
use carbon_core::borsh;

#[derive(CarbonDeserialize, Debug)] 
 

#[carbon(discriminator = "0x7c8627b40b49e869")] 
pub struct NftList { 
        pub version: u8, 
        pub collection_name: String, 
}