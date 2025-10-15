use juniper::GraphQLObject;
use carbon_core::graphql::primitives::Pubkey;
use carbon_core::graphql::primitives::U64;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "RemoveLiquidity")]
pub struct RemoveLiquidityGraphQL {
        pub lb_pair: Pubkey,
        pub from: Pubkey,
        pub position: Pubkey,
        pub amounts: Vec<U64>,
        pub active_bin_id: i32,
}

impl From<crate::types::RemoveLiquidity> for RemoveLiquidityGraphQL {
    fn from(original: crate::types::RemoveLiquidity) -> Self {
        Self {
            lb_pair: carbon_core::graphql::primitives::Pubkey(original.lb_pair),
            from: carbon_core::graphql::primitives::Pubkey(original.from),
            position: carbon_core::graphql::primitives::Pubkey(original.position),
            amounts: original.amounts.into_iter().map(|item| carbon_core::graphql::primitives::U64(item)).collect(),
            active_bin_id: original.active_bin_id,
        }
    }
}