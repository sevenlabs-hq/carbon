use juniper::GraphQLObject;
use carbon_core::graphql::primitives::Pubkey;
use carbon_core::graphql::primitives::U64;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "ClaimFee")]
pub struct ClaimFeeGraphQL {
        pub lb_pair: Pubkey,
        pub position: Pubkey,
        pub owner: Pubkey,
        pub fee_x: U64,
        pub fee_y: U64,
}

impl From<crate::types::ClaimFee> for ClaimFeeGraphQL {
    fn from(original: crate::types::ClaimFee) -> Self {
        Self {
            lb_pair: carbon_core::graphql::primitives::Pubkey(original.lb_pair),
            position: carbon_core::graphql::primitives::Pubkey(original.position),
            owner: carbon_core::graphql::primitives::Pubkey(original.owner),
            fee_x: carbon_core::graphql::primitives::U64(original.fee_x),
            fee_y: carbon_core::graphql::primitives::U64(original.fee_y),
        }
    }
}