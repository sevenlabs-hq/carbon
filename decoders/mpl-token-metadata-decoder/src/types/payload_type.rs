
use super::*;
use carbon_core::deserialize::CarbonDeserialize;
use carbon_proc_macros::CarbonDeserialize;
use carbon_core::borsh;


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub enum PayloadType {
    Pubkey
                (
                    solana_sdk::pubkey::Pubkey,
                )
    ,
    Seeds
                (
                    SeedsVec,
                )
    ,
    MerkleProof
                (
                    LeafInfo,
                )
    ,
    Number
                (
                    u64,
                )
    ,
}


