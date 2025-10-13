pub mod bonding_curve_schema;
pub mod fee_config_schema;
pub mod global_config_schema;
pub mod global_volume_accumulator_schema;
pub mod pool_schema;
pub mod user_volume_accumulator_schema;

pub use bonding_curve_schema::*;
pub use fee_config_schema::*;
pub use global_config_schema::*;
pub use global_volume_accumulator_schema::*;
pub use pool_schema::*;
pub use user_volume_accumulator_schema::*;

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