use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub enum OracleSource {
    Pyth,
    Switchboard,
    QuoteAsset,
    Pyth1K,
    Pyth1M,
    PythStableCoin,
    Prelaunch,
    PythPull,
    Pyth1KPull,
    Pyth1MPull,
    PythStableCoinPull,
    SwitchboardOnDemand,
    PythLazer,
    PythLazer1K,
    PythLazer1M,
    PythLazerStableCoin,
}
