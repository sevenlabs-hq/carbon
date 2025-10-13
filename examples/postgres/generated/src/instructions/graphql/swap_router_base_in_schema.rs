use juniper::GraphQLObject;
use carbon_core::graphql::primitives::U64;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "SwapRouterBaseIn")]
pub struct SwapRouterBaseInGraphQL {
        pub amount_in: U64,
        pub amount_out_minimum: U64,
}

impl From<crate::instructions::postgres::SwapRouterBaseInRow> for SwapRouterBaseInGraphQL {
    fn from(row: crate::instructions::postgres::SwapRouterBaseInRow) -> Self {
        Self {
            amount_in: carbon_core::graphql::primitives::U64(*row.amount_in),
            amount_out_minimum: carbon_core::graphql::primitives::U64(*row.amount_out_minimum),
        }
    }
}