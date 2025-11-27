use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub enum PostOnlyParam {
    None,
    MustPostOnly,
    TryPostOnly,
    Slide,
}
