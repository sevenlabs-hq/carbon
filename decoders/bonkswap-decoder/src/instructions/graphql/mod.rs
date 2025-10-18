pub mod add_supply_schema;
pub mod add_tokens_schema;
pub mod create_dual_farm_schema;
pub mod create_farm_schema;
pub mod create_pool_schema;
pub mod create_provider_schema;
pub mod create_state_schema;
pub mod create_triple_farm_schema;
pub mod swap_schema;
pub mod update_fees_schema;
pub mod withdraw_shares_schema;

pub use add_supply_schema::*;
pub use add_tokens_schema::*;
pub use create_dual_farm_schema::*;
pub use create_farm_schema::*;
pub use create_pool_schema::*;
pub use create_provider_schema::*;
pub use create_state_schema::*;
pub use create_triple_farm_schema::*;
pub use swap_schema::*;
pub use update_fees_schema::*;
pub use withdraw_shares_schema::*;

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
