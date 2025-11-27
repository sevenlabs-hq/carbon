use {
    carbon_core::{borsh, CarbonDeserialize},
    serde_big_array::BigArray,
};

#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct OracleConfig {
    pub conf_filter: f64,
    pub max_staleness_slots: i64,
    #[serde(with = "BigArray")]
    pub reserved: [u8; 72],
}
