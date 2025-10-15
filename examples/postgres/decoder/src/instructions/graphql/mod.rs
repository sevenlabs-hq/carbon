pub mod admin_set_coin_creator_schema;
pub mod admin_update_token_incentives_schema;
pub mod buy_schema;
pub mod create_config_schema;
pub mod create_pool_schema;
pub mod deposit_schema;
pub mod disable_schema;
pub mod sell_schema;
pub mod update_fee_config_schema;
pub mod withdraw_schema;

pub use admin_set_coin_creator_schema::*;
pub use admin_update_token_incentives_schema::*;
pub use buy_schema::*;
pub use create_config_schema::*;
pub use create_pool_schema::*;
pub use deposit_schema::*;
pub use disable_schema::*;
pub use sell_schema::*;
pub use update_fee_config_schema::*;
pub use withdraw_schema::*;

use juniper::GraphQLObject;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "InstructionMetadata")]
pub struct InstructionMetadataGraphQL {
    pub signature: String,
    pub instruction_index: carbon_core::graphql::primitives::U32,
    pub stack_height: carbon_core::graphql::primitives::U32,
    pub slot: Option<carbon_core::graphql::primitives::U64>,
}

impl From<carbon_core::postgres::metadata::InstructionRowMetadata> for InstructionMetadataGraphQL {
    fn from(metadata: carbon_core::postgres::metadata::InstructionRowMetadata) -> Self {
        Self {
            signature: metadata.signature,
            instruction_index: carbon_core::graphql::primitives::U32(
                (*metadata.instruction_index) as u32,
            ),
            stack_height: carbon_core::graphql::primitives::U32((*metadata.stack_height) as u32),
            slot: metadata
                .slot
                .map(|v| carbon_core::graphql::primitives::U64(*v)),
        }
    }
}
