use carbon_core::{borsh, CarbonDeserialize};

#[repr(u8)]
#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub enum AuthorityType {
    MintTokens,
    FreezeAccount,
    AccountOwner,
    CloseAccount,
}
