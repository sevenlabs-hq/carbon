pub mod farm_schema;
pub mod pool_schema;
pub mod pool_v2_schema;
pub mod provider_schema;
pub mod state_schema;

pub use farm_schema::*;
pub use pool_schema::*;
pub use pool_v2_schema::*;
pub use provider_schema::*;
pub use state_schema::*;

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
