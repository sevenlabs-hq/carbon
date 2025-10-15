pub mod bin_array_schema;
pub mod bin_array_bitmap_extension_schema;
pub mod claim_fee_operator_schema;
pub mod dummy_zc_account_schema;
pub mod lb_pair_schema;
pub mod oracle_schema;
pub mod position_schema;
pub mod position_v2_schema;
pub mod preset_parameter_schema;
pub mod preset_parameter2_schema;
pub mod token_badge_schema;

pub use bin_array_schema::*;
pub use bin_array_bitmap_extension_schema::*;
pub use claim_fee_operator_schema::*;
pub use dummy_zc_account_schema::*;
pub use lb_pair_schema::*;
pub use oracle_schema::*;
pub use position_schema::*;
pub use position_v2_schema::*;
pub use preset_parameter_schema::*;
pub use preset_parameter2_schema::*;
pub use token_badge_schema::*;

use juniper::GraphQLObject;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "AccountMetadata")]
pub struct AccountMetadataGraphQL {
    pub pubkey: carbon_core::graphql::primitives::Pubkey,
    pub slot: Option<carbon_core::graphql::primitives::U64>,
}

impl From<carbon_core::postgres::metadata::AccountRowMetadata> for AccountMetadataGraphQL {
    fn from(metadata: carbon_core::postgres::metadata::AccountRowMetadata) -> Self {
        Self {
            pubkey: carbon_core::graphql::primitives::Pubkey(*metadata.pubkey),
            slot: metadata
                .slot
                .map(|v| carbon_core::graphql::primitives::U64(*v)),
        }
    }
}