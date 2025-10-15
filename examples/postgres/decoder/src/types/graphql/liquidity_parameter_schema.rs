use juniper::GraphQLObject;
use carbon_core::graphql::primitives::U64;
use crate::types::graphql::BinLiquidityDistributionGraphQL;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "LiquidityParameter")]
pub struct LiquidityParameterGraphQL {
            /// Amount of X token to deposit
            pub amount_x: U64,
            /// Amount of Y token to deposit
            pub amount_y: U64,
            /// Liquidity distribution to each bins
            pub bin_liquidity_dist: Vec<BinLiquidityDistributionGraphQL>,
}

impl From<crate::types::LiquidityParameter> for LiquidityParameterGraphQL {
    fn from(original: crate::types::LiquidityParameter) -> Self {
        Self {
            amount_x: carbon_core::graphql::primitives::U64(original.amount_x),
            amount_y: carbon_core::graphql::primitives::U64(original.amount_y),
            bin_liquidity_dist: original.bin_liquidity_dist.into_iter().map(|item| item.into()).collect(),
        }
    }
}