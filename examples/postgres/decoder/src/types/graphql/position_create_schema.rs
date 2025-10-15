use juniper::GraphQLObject;
use carbon_core::graphql::primitives::Pubkey;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "PositionCreate")]
pub struct PositionCreateGraphQL {
        pub lb_pair: Pubkey,
        pub position: Pubkey,
        pub owner: Pubkey,
}

impl From<crate::types::PositionCreate> for PositionCreateGraphQL {
    fn from(original: crate::types::PositionCreate) -> Self {
        Self {
            lb_pair: carbon_core::graphql::primitives::Pubkey(original.lb_pair),
            position: carbon_core::graphql::primitives::Pubkey(original.position),
            owner: carbon_core::graphql::primitives::Pubkey(original.owner),
        }
    }
}