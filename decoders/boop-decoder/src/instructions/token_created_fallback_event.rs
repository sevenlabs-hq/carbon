use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d9dca235ca5a32738")]
pub struct TokenCreatedFallbackEvent {
    pub name: String,
    pub symbol: String,
    pub uri: String,
}
