
use super::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub enum OracleValidation {
    Uninitialized,
    V1
                {
                    create: ExternalValidationResult,
                    transfer: ExternalValidationResult,
                    burn: ExternalValidationResult,
                    update: ExternalValidationResult,
                }
    ,
}


