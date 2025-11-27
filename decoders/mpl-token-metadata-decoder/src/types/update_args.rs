use super::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub enum UpdateArgs {
    V1 {
        new_update_authority: Option<solana_pubkey::Pubkey>,
        data: Option<Data>,
        primary_sale_happened: Option<bool>,
        is_mutable: Option<bool>,
        collection: CollectionToggle,
        collection_details: CollectionDetailsToggle,
        uses: UsesToggle,
        rule_set: RuleSetToggle,
        authorization_data: Option<AuthorizationData>,
    },
    AsUpdateAuthorityV2 {
        new_update_authority: Option<solana_pubkey::Pubkey>,
        data: Option<Data>,
        primary_sale_happened: Option<bool>,
        is_mutable: Option<bool>,
        collection: CollectionToggle,
        collection_details: CollectionDetailsToggle,
        uses: UsesToggle,
        rule_set: RuleSetToggle,
        token_standard: Option<TokenStandard>,
        authorization_data: Option<AuthorizationData>,
    },
    AsAuthorityItemDelegateV2 {
        new_update_authority: Option<solana_pubkey::Pubkey>,
        primary_sale_happened: Option<bool>,
        is_mutable: Option<bool>,
        token_standard: Option<TokenStandard>,
        authorization_data: Option<AuthorizationData>,
    },
    AsCollectionDelegateV2 {
        collection: CollectionToggle,
        authorization_data: Option<AuthorizationData>,
    },
    AsDataDelegateV2 {
        data: Option<Data>,
        authorization_data: Option<AuthorizationData>,
    },
    AsProgrammableConfigDelegateV2 {
        rule_set: RuleSetToggle,
        authorization_data: Option<AuthorizationData>,
    },
    AsDataItemDelegateV2 {
        data: Option<Data>,
        authorization_data: Option<AuthorizationData>,
    },
    AsCollectionItemDelegateV2 {
        collection: CollectionToggle,
        authorization_data: Option<AuthorizationData>,
    },
    AsProgrammableConfigItemDelegateV2 {
        rule_set: RuleSetToggle,
        authorization_data: Option<AuthorizationData>,
    },
}
