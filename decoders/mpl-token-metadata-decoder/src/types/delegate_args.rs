use super::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub enum DelegateArgs {
    CollectionV1 {
        authorization_data: Option<AuthorizationData>,
    },
    SaleV1 {
        amount: u64,
        authorization_data: Option<AuthorizationData>,
    },
    TransferV1 {
        amount: u64,
        authorization_data: Option<AuthorizationData>,
    },
    DataV1 {
        authorization_data: Option<AuthorizationData>,
    },
    UtilityV1 {
        amount: u64,
        authorization_data: Option<AuthorizationData>,
    },
    StakingV1 {
        amount: u64,
        authorization_data: Option<AuthorizationData>,
    },
    StandardV1 {
        amount: u64,
    },
    LockedTransferV1 {
        amount: u64,
        locked_address: solana_pubkey::Pubkey,
        authorization_data: Option<AuthorizationData>,
    },
    ProgrammableConfigV1 {
        authorization_data: Option<AuthorizationData>,
    },
    AuthorityItemV1 {
        authorization_data: Option<AuthorizationData>,
    },
    DataItemV1 {
        authorization_data: Option<AuthorizationData>,
    },
    CollectionItemV1 {
        authorization_data: Option<AuthorizationData>,
    },
    ProgrammableConfigItemV1 {
        authorization_data: Option<AuthorizationData>,
    },
    PrintDelegateV1 {
        authorization_data: Option<AuthorizationData>,
    },
}
