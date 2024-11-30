
use super::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub enum ExtraAccount {
    PreconfiguredProgram
                {
                    is_signer: bool,
                    is_writable: bool,
                }
    ,
    PreconfiguredCollection
                {
                    is_signer: bool,
                    is_writable: bool,
                }
    ,
    PreconfiguredOwner
                {
                    is_signer: bool,
                    is_writable: bool,
                }
    ,
    PreconfiguredRecipient
                {
                    is_signer: bool,
                    is_writable: bool,
                }
    ,
    PreconfiguredAsset
                {
                    is_signer: bool,
                    is_writable: bool,
                }
    ,
    CustomPda
                {
                    seeds: Vec<Seed>,
                    custom_program_id: Option<solana_sdk::pubkey::Pubkey>,
                    is_signer: bool,
                    is_writable: bool,
                }
    ,
    Address
                {
                    address: solana_sdk::pubkey::Pubkey,
                    is_signer: bool,
                    is_writable: bool,
                }
    ,
}


