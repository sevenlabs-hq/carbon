use carbon_core::borsh;
use carbon_core::deserialize::CarbonDeserialize;
use carbon_proc_macros::CarbonDeserialize;

#[repr(u8)]
#[derive(
    CarbonDeserialize,
    Debug,
    serde::Serialize,
    serde::Deserialize,
    PartialEq,
    Eq,
    Clone,
    Hash,
    Default,
)]
pub enum AccountState {
    #[default]
    Uninitialized,
    Initialized,
    Frozen,
}
