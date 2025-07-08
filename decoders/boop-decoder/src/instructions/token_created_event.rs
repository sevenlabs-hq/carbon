use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d607a718a32e39539")]
pub struct TokenCreatedEvent {
    pub name: String,
    pub symbol: String,
    pub uri: String,
}
