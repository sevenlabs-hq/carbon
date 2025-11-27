use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub enum PostOnlyParam {
    None,
    MustPostOnly,
    TryPostOnly,
    Slide,
}
