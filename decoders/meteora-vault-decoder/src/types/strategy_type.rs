use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub enum StrategyType {
    PortFinanceWithoutLM,
    PortFinanceWithLM,
    SolendWithoutLM,
    Mango,
    SolendWithLM,
    ApricotWithoutLM,
    Francium,
    Tulip,
    Vault,
    Drift,
    Frakt,
    Marginfi,
}
