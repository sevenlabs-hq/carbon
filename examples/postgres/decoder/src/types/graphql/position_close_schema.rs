use juniper::GraphQLObject;
use carbon_core::graphql::primitives::Pubkey;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "PositionClose")]
pub struct PositionCloseGraphQL {
        pub position: Pubkey,
        pub owner: Pubkey,
}

impl From<crate::types::PositionClose> for PositionCloseGraphQL {
    fn from(original: crate::types::PositionClose) -> Self {
        Self {
            position: carbon_core::graphql::primitives::Pubkey(original.position),
            owner: carbon_core::graphql::primitives::Pubkey(original.owner),
        }
    }
}