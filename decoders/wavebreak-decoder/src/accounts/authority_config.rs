use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x03")]
pub struct AuthorityConfig {
    pub discriminator: AccountDiscriminator,
    #[serde(with = "serde_big_array::BigArray")]
    pub authorities: [ProgramAuthority; 64],
}
