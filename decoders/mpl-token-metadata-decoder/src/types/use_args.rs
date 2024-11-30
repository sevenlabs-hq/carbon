
use super::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub enum UseArgs {
    V1
                {
                    authorization_data: Option<AuthorizationData>,
                }
    ,
}


